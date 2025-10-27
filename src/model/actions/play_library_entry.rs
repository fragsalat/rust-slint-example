use crate::{mock::LibraryEntry, model::{actions::StateChange, state::Field, State}};


impl State {
  pub(in crate::model) fn play_library_entry(&self, library_entry: LibraryEntry) {
    let player = self.player.clone();
    let state = self.clone();

    tokio::runtime::Handle::current().block_on(async move {
      let result = player.play(library_entry.clone()).await;
      let mut inner = state.inner.lock().unwrap();

      match result {
        Ok(_) => {
          inner.set(Field::playing_library_entry(Some(library_entry)));
          inner.set(Field::is_playing(true));
        }
        Err(error) => {
          let mut new_messages = inner.messages.clone();
          new_messages.push(format!("Could not play track: {}", error));
          inner.set(Field::messages(new_messages));
        }
      }
    })
  }
}