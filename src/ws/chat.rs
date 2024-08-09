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
use serde_json::json;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::{
    sync::broadcast::{self, Receiver},
    task::JoinHandle,
};

use super::model::{BoradCastContent, BroadCastType, ContentType, ReplyContent, ReplyType};

use super::memory_hub::MemoryHub;

use crate::api::auth::Claims;

use super::memory_hub::Hub;

pub const DEFAULT_ROOM: &str = "main";

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<(MemoryHub, broadcast::Sender<BoradCastContent>)>>,
    claims: Option<Claims>,
) -> impl IntoResponse {
    let (uid, uname) = claims.map(|x| (x.sub.clone(), x.sub)).unwrap_or_else(|| {
        let name = format!("tourist_{}", rand::thread_rng().gen_range(10000..99999));
        (uuid::Uuid::new_v4().to_string(), name)
    });
    ws.on_upgrade(move |socket| websocket(socket, state, uid, uname))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
pub async fn websocket<T>(
    stream: WebSocket,
    state: Arc<(T, broadcast::Sender<BoradCastContent>)>,
    uid: String,
    uname: String,
) where
    T: Hub + Sync + Send + 'static,
{
    // By splitting, we can send and receive at the same time.
    let (sender, receiver) = stream.split();

    let sender = Arc::new(tokio::sync::Mutex::new(sender));

    /* receive message from broadcast */
    let tx_broadcast = state.1.clone();
    let rx = tx_broadcast.subscribe();

    let current_uid = uid;

    /*join the default room */
    state.0.join(&current_uid, &uname, DEFAULT_ROOM).unwrap();

    // Now send the "joined" message to all subscribers.
    let _ = tx_broadcast.send(BoradCastContent {
        ty: BroadCastType::Join,
        from_uid: current_uid.clone(),
        from_uname: state.0.uname(&current_uid).unwrap(),
        rooms: HashSet::from([DEFAULT_ROOM.to_string()]),
        msg: None,
    });

    let current_uid_for_send = current_uid.clone();
    let state_for_send = state.clone();
    let send_for_broadcast = sender.clone();

    let mut send_task =
        handle_msg_from_hub(state_for_send, current_uid_for_send, rx, send_for_broadcast);

    let current_uid_for_recv = current_uid.clone();
    let state_for_recv = state.clone();
    let send_for_reply = sender.clone();

    let mut recv_task = handle_msg_from_websocket(
        state_for_recv,
        current_uid_for_recv,
        tx_broadcast.clone(),
        receiver,
        send_for_reply,
    );

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    let _ = tx_broadcast.send(BoradCastContent {
        ty: BroadCastType::Quit,
        from_uid: current_uid.clone(),
        from_uname: state.0.uname(&current_uid).unwrap(),
        rooms: HashSet::from(state.0.rooms(&current_uid).unwrap()),
        msg: None,
    });

    state.0.quit(&current_uid, None).unwrap();
}

pub fn handle_msg_from_hub<T>(
    state: Arc<(T, broadcast::Sender<BoradCastContent>)>,
    current_uid_for_send: String,
    mut rx_boardcast: Receiver<BoradCastContent>,
    tx_websocket: Arc<tokio::sync::Mutex<SplitSink<WebSocket, Message>>>,
) -> JoinHandle<()>
where
    T: Hub + Sync + Send + 'static,
{
    tokio::spawn(async move {
        while let Ok(content) = rx_boardcast.recv().await {
            let rooms = state.0.rooms(&current_uid_for_send).unwrap();
            let should_send = rooms
                .clone()
                .into_iter()
                .any(|x| content.rooms.contains(&x));

            if !should_send {
                continue;
            }

            let from_self = current_uid_for_send == content.from_uid;

            let message = match content.ty {
                BroadCastType::Join => Message::Text(format!(
                    "join_room:{}",
                    serde_json::json!({"room":content.rooms,"uname":content.from_uname,"from_self": from_self})
                )),
                BroadCastType::Quit => Message::Text(format!(
                    "quit_room:{}",
                    serde_json::json!({"room":content.rooms,"uname":content.from_uname,"from_self": from_self})
                )),
                BroadCastType::ReName => Message::Text(format!(
                    "update_uname:{}",
                    serde_json::json!({"room":content.rooms,"old_uname":content.msg.unwrap_or_default(),"uname":content.from_uname,"from_self": from_self})
                )),
                BroadCastType::Message => Message::Text(format!(
                    "message:{}",
                    serde_json::json!({"room":content.rooms,"from_uname":content.from_uname,"content":content.msg,"from_self": from_self})
                )),
            };

            if tx_websocket.lock().await.send(message).await.is_err() {
                break;
            };
        }
    })
}

pub fn handle_msg_from_websocket<T>(
    state: Arc<(T, broadcast::Sender<BoradCastContent>)>,
    current_uid_for_recv: String,
    tx_broadcast: broadcast::Sender<BoradCastContent>,
    mut rx_websocket: SplitStream<WebSocket>,
    tx_websocket: Arc<tokio::sync::Mutex<SplitSink<WebSocket, Message>>>,
) -> JoinHandle<()>
where
    T: Hub + Sync + Send + 'static,
{
    tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = rx_websocket.next().await {
            let content = match &text {
                x if x.starts_with("/list") => {
                    let rooms = state.0.rooms(&current_uid_for_recv).unwrap();
                    let data = rooms
                        .into_iter()
                        .map(|x| {
                            let room = x.clone();
                            let uids = state.0.uids(&x).unwrap();
                            tracing::error!("{},{:?}", current_uid_for_recv, &uids);
                            let u_map = uids
                                .into_iter()
                                .map(|uid| {
                                    let uname = state.0.uname(&uid).unwrap();
                                    (uid, uname)
                                })
                                .collect::<HashMap<String, String>>();
                            (room, u_map)
                        })
                        .collect::<HashMap<String, HashMap<String, String>>>();
                    ContentType::Reply(ReplyContent {
                        ty: ReplyType::List,
                        msg: Some(json!(data).to_string()),
                    })
                }
                x if x.starts_with("/join ") => {
                    let room = text.trim_start_matches("/join ");
                    if room.is_empty() {
                        ContentType::Reply(ReplyContent {
                            ty: ReplyType::Notify,
                            msg: Some("empty room name".to_string()),
                        })
                    } else {
                        let name = state.0.uname(&current_uid_for_recv).unwrap();
                        if !state
                            .0
                            .is_already_in_room(&current_uid_for_recv, room)
                            .unwrap()
                        {
                            state.0.join(&current_uid_for_recv, &name, room).unwrap();
                        }
                        ContentType::BoradCast(BoradCastContent {
                            ty: BroadCastType::Join,
                            from_uid: current_uid_for_recv.clone(),
                            from_uname: name,
                            rooms: HashSet::from([room.to_string()]),
                            msg: None,
                        })
                    }
                }
                x if x.starts_with("/quit ") => {
                    let room = text.trim_start_matches("/quit ");
                    if room.is_empty() {
                        ContentType::Reply(ReplyContent {
                            ty: ReplyType::Notify,
                            msg: Some("empty room name".to_string()),
                        })
                    } else if room == DEFAULT_ROOM {
                        ContentType::Reply(ReplyContent {
                            ty: ReplyType::Notify,
                            msg: Some(format!("can not quit room:【{}】", room)),
                        })
                    } else {
                        match state
                            .0
                            .is_already_in_room(&current_uid_for_recv, room)
                            .unwrap()
                        {
                            false => ContentType::Reply(ReplyContent {
                                ty: ReplyType::Notify,
                                msg: Some(format!("you are not in room:【{}】", room)),
                            }),
                            true => {
                                state.0.quit(&current_uid_for_recv, Some(room)).unwrap();
                                ContentType::BoradCast(BoradCastContent {
                                    ty: BroadCastType::Quit,
                                    from_uid: current_uid_for_recv.clone(),
                                    from_uname: state.0.uname(&current_uid_for_recv).unwrap(),
                                    rooms: HashSet::from([room.to_string()]),
                                    msg: None,
                                })
                            }
                        }
                    }
                }
                x if x.starts_with("/name ") => {
                    let new_name = text.trim_start_matches("/name ");
                    if new_name.is_empty() {
                        ContentType::Reply(ReplyContent {
                            ty: ReplyType::Notify,
                            msg: Some("empty name".to_string()),
                        })
                    } else {
                        let old_name = Some(state.0.uname(&current_uid_for_recv).unwrap());
                        state
                            .0
                            .update_name(&current_uid_for_recv, &new_name)
                            .unwrap();

                        ContentType::BoradCast(BoradCastContent {
                            ty: BroadCastType::ReName,
                            from_uid: current_uid_for_recv.clone(),
                            from_uname: new_name.to_string(),
                            rooms: state.0.rooms(&current_uid_for_recv).unwrap(),
                            msg: old_name,
                        })
                    }
                }
                _ => {
                    let reg = Regex::new("【(.+)】(.*)").unwrap();
                    if let Ok(Some(c)) = reg.captures(text.as_str()) {
                        let room = c.get(1).unwrap().as_str();
                        let msg = c.get(2).unwrap().as_str();
                        ContentType::BoradCast(BoradCastContent {
                            ty: BroadCastType::Message,
                            from_uid: current_uid_for_recv.clone(),
                            from_uname: state.0.uname(&current_uid_for_recv).unwrap(),
                            rooms: HashSet::from([room.to_string()]),
                            msg: Some(msg.to_string()),
                        })
                    } else {
                        ContentType::BoradCast(BoradCastContent {
                            ty: BroadCastType::Message,
                            from_uid: current_uid_for_recv.clone(),
                            from_uname: state.0.uname(&current_uid_for_recv).unwrap(),
                            rooms: HashSet::from([DEFAULT_ROOM.to_string()]),
                            msg: Some(text),
                        })
                    }
                }
            };

            match content {
                ContentType::BoradCast(content) => {
                    let _ = tx_broadcast.send(content).unwrap();
                }
                ContentType::Reply(content) => {
                    let _ = tx_websocket
                        .lock()
                        .await
                        .send(Message::Text(match content.ty {
                            ReplyType::List => {
                                format!("list:{}", &content.msg.unwrap_or_default())
                            }
                            ReplyType::Notify => {
                                format!("notify:{}", content.msg.unwrap_or_default())
                            }
                        }))
                        .await;
                }
            }
        }
    })
}

// Include utf-8 file at **compile** time.
pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../../static/chat.html"))
}
