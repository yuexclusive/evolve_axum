use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
};
use fancy_regex::Regex;
use futures::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use rand::Rng;
use redis::RedisResult;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::task::JoinHandle;

use crate::{
    error::{WSError, WSResult},
    DEFAULT_ROOM_MANAGER_UID, DEFAULT_ROOM_MANAGER_UNAME,
};

use super::{
    BoradCastContent, BroadCastType, ContentType, ReplyContent, ReplyType, DEFAULT_ROOM,
    REDIS_WS_CHANNEL,
};

use super::Store;

pub struct WSState<T>
where
    T: Store + Sync + Send + 'static,
{
    pub store: T,
}

impl<T> WSState<T>
where
    T: Store + Sync + Send + 'static,
{
    pub fn new(store: T) -> Self {
        Self { store }
    }
}

pub async fn websocket_handler<T>(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WSState<T>>>,
) -> Result<impl IntoResponse, WSError>
where
    T: Store + Sync + Send + 'static,
{
    let uid = uuid::Uuid::new_v4().to_string();
    let uname = format!("tourist_{}", rand::rng().random_range(10000..99999));
    let res = ws
        .on_failed_upgrade(|e| tracing::error!("Failed to upgrade websocket: {e}"))
        .on_upgrade(move |socket| websocket(socket, state, uid, uname));
    Ok(res)
}

pub async fn publish_message_to_default_room(msg: &str) -> RedisResult<()> {
    let content = BoradCastContent {
        ty: BroadCastType::Message,
        from_uid: DEFAULT_ROOM_MANAGER_UID.to_string(),
        from_uname: DEFAULT_ROOM_MANAGER_UNAME.to_string(),
        rooms: Vec::from([DEFAULT_ROOM.to_string()]),
        msg: Some(msg.to_string()),
    };

    let res = publish(content).await?;

    Ok(res)
}

async fn publish(message: BoradCastContent) -> RedisResult<()> {
    let res = evolve_redis::publish(REDIS_WS_CHANNEL, message).await?;
    Ok(res)
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket<T>(stream: WebSocket, state: Arc<WSState<T>>, uid: String, uname: String)
where
    T: Store + Sync + Send + 'static,
{
    // By splitting, we can send and receive at the same time.
    let (sender, receiver) = stream.split();

    let sender = Arc::new(tokio::sync::Mutex::new(sender));

    /* receive message from broadcast */

    let current_uid = uid;

    /*join the default room */
    state
        .store
        .join(&current_uid, &uname, DEFAULT_ROOM)
        .unwrap();

    let current_uid_for_send = current_uid.clone();
    let state_for_send = state.clone();
    let send_for_broadcast = sender.clone();

    let mut send_task =
        handle_msg_from_hub(state_for_send, current_uid_for_send, send_for_broadcast)
            .await
            .unwrap();

    // send join message, must after send_task
    let _ = publish(BoradCastContent {
        ty: BroadCastType::Join,
        from_uid: current_uid.clone(),
        from_uname: state.store.uname(&current_uid).unwrap(),
        rooms: Vec::from([DEFAULT_ROOM.to_string()]),
        msg: None,
    })
    .await;

    let current_uid_for_recv = current_uid.clone();
    let state_for_recv = state.clone();
    let send_for_reply = sender.clone();

    let mut recv_task = handle_msg_from_websocket(
        state_for_recv,
        current_uid_for_recv,
        receiver,
        send_for_reply,
    )
    .await;

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    let _ = publish(BoradCastContent {
        ty: BroadCastType::Quit,
        from_uid: current_uid.clone(),
        from_uname: state.store.uname(&current_uid).unwrap(),
        rooms: Vec::from(state.store.rooms(&current_uid, 1, 1000).unwrap()),
        msg: None,
    })
    .await;

    state.store.quit(&current_uid, None).unwrap();
}

async fn handle_msg_from_hub<T>(
    state: Arc<WSState<T>>,
    current_uid_for_send: String,
    tx_websocket: Arc<tokio::sync::Mutex<SplitSink<WebSocket, Message>>>,
) -> WSResult<JoinHandle<()>>
where
    T: Store + Sync + Send + 'static,
{
    let mut pubsub = evolve_redis::pubsub().await?;
    pubsub.subscribe(REDIS_WS_CHANNEL).await?;

    let mut stream = pubsub.into_on_message();
    Ok(tokio::spawn(async move {
        while let Some(msg) = stream.next().await {
            let content = msg.get_payload::<BoradCastContent>().unwrap();

            let rooms = state.store.rooms(&current_uid_for_send, 1, 1000).unwrap();
            if rooms
                .clone()
                .into_iter()
                .any(|x| content.rooms.contains(&x))
            {
                if content.ty == BroadCastType::Message {
                    state
                        .store
                        .update_room_order(&current_uid_for_send, content.rooms.first().unwrap())
                        .unwrap();
                }

                let message = match content.ty {
                    BroadCastType::Join => Message::Text(format!(
                        "join_room:{}",
                        serde_json::json!({"room": &content.rooms.first(),"rooms":rooms,"uname":content.from_uname,"from_self": current_uid_for_send == content.from_uid})
                    ).into()),
                    BroadCastType::Quit => Message::Text(format!(
                        "quit_room:{}",
                        serde_json::json!({"room": &content.rooms.first(),"uname":content.from_uname})
                    ).into()),
                    BroadCastType::ReName => Message::Text(format!(
                        "update_uname:{}",
                        serde_json::json!({"room":&content.rooms.first(),"old_uname":content.msg.unwrap_or_default(),"uname":content.from_uname})
                    ).into()),
                    BroadCastType::Message => Message::Text(format!(
                        "message:{}",
                        serde_json::json!({"room":&content.rooms.first(), "rooms": rooms,"from_uname":content.from_uname,"content":content.msg})
                    ).into()),
                };
                tx_websocket.lock().await.send(message).await.unwrap();
            }
        }
    }))
}

async fn handle_msg_from_websocket<T>(
    state: Arc<WSState<T>>,
    current_uid_for_recv: String,
    mut rx_websocket: SplitStream<WebSocket>,
    tx_websocket: Arc<tokio::sync::Mutex<SplitSink<WebSocket, Message>>>,
) -> JoinHandle<()>
where
    T: Store + Sync + Send + 'static,
{
    tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = rx_websocket.next().await {
            let contents = match &text {
                x if x.starts_with("/list") => {
                    let rooms = state.store.rooms(&current_uid_for_recv, 1, 1000).unwrap();
                    let data = rooms
                        .into_iter()
                        .map(|x| {
                            let room = x.clone();
                            let uids = state.store.uids(&x).unwrap();
                            let u_map = uids
                                .into_iter()
                                .map(|uid| {
                                    let uname = state.store.uname(&uid).unwrap();
                                    (uid, uname)
                                })
                                .collect::<HashMap<String, String>>();
                            (room, u_map)
                        })
                        .collect::<HashMap<String, HashMap<String, String>>>();
                    vec![ContentType::Reply(ReplyContent {
                        ty: ReplyType::List,
                        msg: Some(json!(data).to_string()),
                    })]
                }
                x if x.starts_with("/join ") => {
                    let room = text.trim_start_matches("/join ");
                    if room.is_empty() {
                        vec![ContentType::Reply(ReplyContent {
                            ty: ReplyType::Notify,
                            msg: Some("empty room name".to_string()),
                        })]
                    } else {
                        let name = state.store.uname(&current_uid_for_recv).unwrap();
                        state
                            .store
                            .join(&current_uid_for_recv, &name, room)
                            .unwrap();
                        vec![ContentType::BoradCast(BoradCastContent {
                            ty: BroadCastType::Join,
                            from_uid: current_uid_for_recv.clone(),
                            from_uname: name,
                            rooms: Vec::from([room.to_string()]),
                            msg: None,
                        })]
                    }
                }
                x if x.starts_with("/quit ") => {
                    let room = text.trim_start_matches("/quit ");
                    if room.is_empty() {
                        vec![ContentType::Reply(ReplyContent {
                            ty: ReplyType::Notify,
                            msg: Some("empty room name".to_string()),
                        })]
                    } else if room == DEFAULT_ROOM {
                        vec![ContentType::Reply(ReplyContent {
                            ty: ReplyType::Notify,
                            msg: Some(format!("can not quit room:【{}】", room)),
                        })]
                    } else {
                        match state
                            .store
                            .is_already_in_room(&current_uid_for_recv, room)
                            .unwrap()
                        {
                            false => vec![ContentType::Reply(ReplyContent {
                                ty: ReplyType::Notify,
                                msg: Some(format!("you are not in room:【{}】", room)),
                            })],
                            true => {
                                state.store.quit(&current_uid_for_recv, Some(room)).unwrap();
                                vec![
                                    ContentType::BoradCast(BoradCastContent {
                                        ty: BroadCastType::Quit,
                                        from_uid: current_uid_for_recv.clone(),
                                        from_uname: state
                                            .store
                                            .uname(&current_uid_for_recv)
                                            .unwrap(),
                                        rooms: Vec::from([room.to_string()]),
                                        msg: None,
                                    }),
                                    ContentType::Reply(ReplyContent {
                                        ty: ReplyType::Rooms,
                                        msg: Some(
                                            json!(state
                                                .store
                                                .rooms(&current_uid_for_recv, 1, 1000)
                                                .unwrap())
                                            .to_string(),
                                        ),
                                    }),
                                ]
                            }
                        }
                    }
                }
                x if x.starts_with("/name ") => {
                    let new_name = text.trim_start_matches("/name ");
                    if new_name.is_empty() {
                        vec![ContentType::Reply(ReplyContent {
                            ty: ReplyType::Notify,
                            msg: Some("empty name".to_string()),
                        })]
                    } else {
                        let old_name = Some(state.store.uname(&current_uid_for_recv).unwrap());
                        state
                            .store
                            .update_name(&current_uid_for_recv, &new_name)
                            .unwrap();

                        vec![ContentType::BoradCast(BoradCastContent {
                            ty: BroadCastType::ReName,
                            from_uid: current_uid_for_recv.clone(),
                            from_uname: new_name.to_string(),
                            rooms: state.store.rooms(&current_uid_for_recv, 1, 1000).unwrap(),
                            msg: old_name,
                        })]
                    }
                }
                _ => {
                    let reg = Regex::new("【(.+)】(.*)").unwrap();

                    if let Ok(Some(c)) = reg.captures(text.as_str()) {
                        let room = c.get(1).unwrap().as_str();
                        let msg = c.get(2).unwrap().as_str();
                        state
                            .store
                            .update_room_order(&current_uid_for_recv, room)
                            .unwrap();
                        vec![ContentType::BoradCast(BoradCastContent {
                            ty: BroadCastType::Message,
                            from_uid: current_uid_for_recv.clone(),
                            from_uname: state.store.uname(&current_uid_for_recv).unwrap(),
                            rooms: Vec::from([room.to_string()]),
                            msg: Some(msg.to_string()),
                        })]
                    } else {
                        vec![ContentType::Reply(ReplyContent {
                            ty: ReplyType::Notify,
                            msg: Some("empty room".to_string()),
                        })]
                    }
                }
            };

            for item in contents {
                match item {
                    ContentType::BoradCast(content) => {
                        let _ = publish(content).await;
                    }
                    ContentType::Reply(content) => {
                        let _ = tx_websocket
                            .lock()
                            .await
                            .send(Message::Text(match content.ty {
                                ReplyType::List => {
                                    format!("list:{}", &content.msg.unwrap_or_default()).into()
                                }
                                ReplyType::Notify => {
                                    format!("notify:{}", content.msg.unwrap_or_default()).into()
                                }
                                ReplyType::Rooms => {
                                    format!("rooms:{}", content.msg.unwrap_or_default()).into()
                                }
                            }))
                            .await;
                    }
                }
            }
        }
    })
}

// Include utf-8 file at **compile** time.
pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../static/chat.html"))
}
