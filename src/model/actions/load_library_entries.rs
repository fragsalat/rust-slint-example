use crate::{mock::get_library_entry, model::{actions::StateChange, state::State}};

impl State {
    pub fn load_library_entry(&mut self, id: i32) -> StateChange {
        let mut inner = self.inner.lock().unwrap();
        match get_library_entry(id) {
            Some(entry) => {
                println!("Loaded library entry: {:?}", entry);
                inner.library_entry = Some(entry);
                StateChange {
                    changed_fields: vec!["library_entry".into()],
                }
            }
            None => {
                println!("Library entry with id {} not found", id);
                inner.messages.push(format!("Library entry with id {} not found", id));
                StateChange {
                    changed_fields: vec!["messages".into()],
                }
            }
        }
    }
}
