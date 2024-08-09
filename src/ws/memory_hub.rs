use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::error::AppResult;
// use tokio::sync::broadcast;

// use super::model::BoradCastContent;

pub struct MemoryHub {
    // room-->[uid,uname]
    room_user_map: Mutex<HashMap<String, HashSet<String>>>,
    // user-->room
    user_room_map: Mutex<HashMap<String, (String, HashSet<String>)>>,
}

impl MemoryHub {
    pub fn new(
        room_user_map: Mutex<HashMap<String, HashSet<String>>>,
        user_room_map: Mutex<HashMap<String, (String, HashSet<String>)>>,
    ) -> Self {
        Self {
            room_user_map,
            user_room_map,
        }
    }
}

pub trait Hub {
    fn uname(&self, uid: &str) -> AppResult<String>;

    fn rooms(&self, uid: &str) -> AppResult<HashSet<String>>;

    fn uids(&self, room: &str) -> AppResult<HashSet<String>>;

    fn is_already_in_room(&self, uid: &str, room: &str) -> AppResult<bool>;

    fn  join(&self, uid: &str, uname: &str, room: &str) -> AppResult<()>;

    fn quit(&self, uid: &str, room: Option<&str>) -> AppResult<()>;

    fn update_name(&self, uid: &str, uname: &str) -> AppResult<()>;
}

impl Hub for MemoryHub {
    fn uname(&self, uid: &str) -> AppResult<String> {
        let res = self
            .user_room_map
            .lock()
            .unwrap()
            .get(uid)
            .unwrap()
            .0
            .clone();

        Ok(res)
    }

    fn rooms(&self, uid: &str) -> AppResult<HashSet<String>> {
        let res = self
            .user_room_map
            .lock()
            .unwrap()
            .get(uid)
            .unwrap()
            .1
            .clone();

        Ok(res)
    }

    fn uids(&self, room: &str) -> AppResult<HashSet<String>> {
        let room_user_map = self.room_user_map.lock().unwrap();
        tracing::error!("fuck you: {:?}", room_user_map);
        let res = room_user_map
            .get(room)
            .map(|x| x.clone())
            .unwrap_or_default();

        Ok(res)
    }

    fn is_already_in_room(&self, uid: &str, room: &str) -> AppResult<bool> {
        let res = self
            .user_room_map
            .lock()
            .unwrap()
            .get(uid)
            .unwrap()
            .1
            .contains(room);
        Ok(res)
    }

    fn join(&self, uid: &str, uname: &str, room: &str) -> AppResult<()> {
        // uid.to_string(),
        // (uname.to_string(), HashSet::from([room.to_string()])
        self.user_room_map
            .lock()
            .unwrap()
            .entry(uid.to_string())
            .or_insert((uname.to_string(), HashSet::new()))
            .1
            .insert(room.to_string());

        self.room_user_map
            .lock()
            .unwrap()
            .entry(room.to_string())
            .or_insert(HashSet::new())
            .insert(uid.to_string());

        Ok(())
    }

    fn quit(&self, uid: &str, room: Option<&str>) -> AppResult<()> {
        match room {
            Some(room) => {
                let mut room_user_map = self.room_user_map.lock().unwrap();
                let mut clear_room = false;
                if let Some(room_user_set) = room_user_map.get_mut(room) {
                    room_user_set.remove(uid);
                    if room_user_set.is_empty() {
                        clear_room = true;
                    }
                }
                if clear_room {
                    room_user_map.remove(room);
                }
                let mut user_room_set = self.user_room_map.lock().unwrap();
                if let Some(room_set) = user_room_set.get_mut(uid) {
                    room_set.1.remove(room);
                }
            }
            None => {
                let mut user_room_map = self.user_room_map.lock().unwrap();
                if let Some((_, room_set)) = user_room_map.get(uid) {
                    for r in room_set.iter() {
                        let mut room_user_map = self.room_user_map.lock().unwrap();
                        let mut clear_room = false;
                        if let Some(user_set) = room_user_map.get_mut(r) {
                            user_set.remove(uid);
                            if user_set.is_empty() {
                                clear_room = true;
                            }
                        }
                        if clear_room {
                            room_user_map.remove(r); /*if room is empty, remove it */
                        }
                    }
                }

                // remove user from user_room_map
                user_room_map.remove(uid);
            }
        }
        Ok(())
    }

    fn update_name(&self, uid: &str, uname: &str) -> AppResult<()> {
        self.user_room_map.lock().unwrap().get_mut(uid).unwrap().0 = uname.to_string();

        Ok(())
    }
}
