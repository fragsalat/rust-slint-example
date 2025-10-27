use tokio::runtime::Handle;

use crate::model::{state::Field, State};


impl State {
  pub(in crate::model) fn toggle_play(&self, is_playing: bool) {
    let player = self.player.clone();
    let state = self.clone();

    Handle::current().block_on(async move {
      let result = if is_playing {
        player.pause().await
      } else {
        player.resume().await
      };

      let mut inner = state.inner.lock().unwrap();
      match result {
        Ok(_) => {
          inner.set(Field::is_playing(is_playing));
        }
        Err(error) => {
          let mut new_messages = inner.messages.clone();
          new_messages.push(format!("Could not toggle play: {}", error));
          inner.set(Field::messages(new_messages));
        }
      }
    })
  }
}