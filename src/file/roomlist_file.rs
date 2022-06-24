use crate::room::Roomlist;
use fs2::FileExt;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

pub struct RoomlistFile {
    file: File,
}

impl RoomlistFile {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let file = File::open(path)?;
        Ok(RoomlistFile { file })
    }

    pub fn read(&mut self) -> Option<Roomlist> {
        self.file.lock_shared().ok()?;
        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf).ok()?;
        self.file.unlock().ok()?;
        let roomlist = rmp_serde::from_slice(&buf).ok()?;
        Some(roomlist)
    }

    pub fn update<F: FnOnce(Roomlist) -> Roomlist>(&mut self, f: F) -> Option<usize> {
        self.file.lock_exclusive().ok()?;

        let mut buf: Vec<u8> = Vec::new();
        self.file.read_to_end(&mut buf).ok()?;
        let roomlist = rmp_serde::from_slice(&buf).ok()?;

        let new_roomlist = f(roomlist);
        let bytes = rmp_serde::to_vec(&new_roomlist).ok()?;
        self.file.write_all(&bytes).ok()?;

        self.file.unlock().ok()?;

        Some(bytes.len())
    }
}
