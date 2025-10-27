use crate::{mock::get_library_entry, model::{actions::StateChange, state::{Field, State}}};

impl State {
    pub(in crate::model) fn load_library_entry(&mut self, id: i32) {
        let mut inner = self.inner.lock().unwrap();
        match get_library_entry(id) {
            Some(entry) => {
                println!("Loaded library entry: {:?}", entry);
                inner.set(Field::active_library_entry(Some(entry)));
            }
            None => {
                println!("Library entry with id {} not found", id);
                let mut new_messages = inner.messages.clone();
                new_messages.push(format!("Library entry with id {} not found", id));
                inner.set(Field::messages(new_messages));
            }
        }
    }
}
