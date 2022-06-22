use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Roomlist {
    list: HashMap<Uuid, Room>,
}

#[derive(Deserialize, Serialize)]
pub struct Room {
    id: Uuid,
    name: String,
    description: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Roomlist {
    pub fn empty_roomlist() -> Self {
        Roomlist {
            list: HashMap::new(),
        }
    }

    pub fn create_room(
        &mut self,
        name: String,
        description: String,
        now: DateTime<Local>,
    ) -> &Room {
        let id = Uuid::new_v4();
        let room = Room {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
        };
        self.list.insert(id, room);
        self.list.get(&id).unwrap()
    }

    pub fn find(&self, id: &Uuid) -> Option<&Room> {
        self.list.get(id)
    }

    pub fn update_room(
        &mut self,
        id: Uuid,
        name: String,
        description: String,
        now: DateTime<Local>,
    ) -> Option<&Room> {
        let target_room = self.find(&id)?;
        let new_room = Room {
            id,
            name,
            description,
            created_at: target_room.created_at,
            updated_at: now,
        };
        self.list.insert(id, new_room);
        self.find(&id)
    }
}

impl Room {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Local> {
        &self.updated_at
    }
}
