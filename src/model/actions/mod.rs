use crate::mock::{LibraryEntry, Progress};

mod load_library_entries;
mod play_library_entry;
mod toggle_play;

#[derive(Debug)]
pub enum Action {
    LoadLibraryEntry(i32),
    PlayLibraryEntry(LibraryEntry),
    TogglePlay(bool),
    SetProgress(Progress)
}

#[derive(Debug, Default, Clone)]
pub struct StateChange {
    pub changed_fields: Vec<String>,
}

impl StateChange {
    pub fn new(fields: Vec<&str>) -> Self {
        StateChange { changed_fields: fields.iter().map(|&s| s.into()).collect() }
    }
}