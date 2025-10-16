mod load_library_entries;

#[derive(Debug)]
pub enum Action {
    LoadLibraryEntry(i32),
}

#[derive(Debug, Default, Clone)]
pub struct StateChange {
    pub changed_fields: Vec<String>,
}
