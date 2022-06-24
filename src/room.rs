use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Roomlist {
    list: HashMap<Uuid, Room>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Roomlist {
    pub fn empty_roomlist() -> Self {
        Roomlist {
            list: HashMap::new(),
        }
    }

    pub fn create_room(&mut self, name: String, description: String, now: DateTime<Local>) -> Uuid {
        let id = Uuid::new_v4();
        let room = Room {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
        };
        self.list.insert(id, room);
        id
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
    ) -> Option<()> {
        let target_room = self.find(&id)?;
        let new_room = Room {
            id,
            name,
            description,
            created_at: target_room.created_at,
            updated_at: now,
        };
        self.list.insert(id, new_room);
        Some(())
    }
}

#[cfg(test)]
mod test {
    use super::Roomlist;
    use chrono::Local;
    use uuid::Uuid;

    #[test]
    fn create_and_find_room() {
        let mut roomlist = Roomlist::empty_roomlist();
        let name = String::from("name");
        let description = String::from("description");
        let now = Local::now();

        let room_id = roomlist.create_room(name, description, now);
        assert!(roomlist.find(&room_id).is_some());

        let room = roomlist.find(&room_id).unwrap();
        assert_eq!("name", room.name);
        assert_eq!("description", room.description);
        assert_eq!(now.timestamp(), room.created_at.timestamp());
        assert_eq!(now.timestamp(), room.updated_at.timestamp());
    }

    #[test]
    fn room_id_not_found() {
        let mut roomlist = Roomlist::empty_roomlist();
        let name = String::from("部屋名");
        let description = String::from("説明");
        let now = Local::now();

        roomlist.create_room(name, description, now);
        let not_exists_room_id = Uuid::new_v4();
        assert!(roomlist.find(&not_exists_room_id).is_none());
    }

    #[test]
    fn update_room() {
        let mut roomlist = Roomlist::empty_roomlist();
        let name = String::from("name");
        let description = String::from("description");
        let created_at = Local::now();

        let room_id = roomlist.create_room(name, description, created_at);
        let updated_at = Local::now();
        let new_name = String::from("new_name");
        let new_description = String::from("new_description");
        let result = roomlist.update_room(room_id, new_name, new_description, updated_at);

        assert!(result.is_some());
        assert!(roomlist.find(&room_id).is_some());
        let updated_room = roomlist.find(&room_id).unwrap();
        assert_eq!(room_id, updated_room.id);
        assert_eq!("new_name", updated_room.name);
        assert_eq!("new_description", updated_room.description);
        assert_eq!(created_at.timestamp(), updated_room.created_at.timestamp());
        assert_eq!(updated_at.timestamp(), updated_room.updated_at.timestamp());
    }
}
