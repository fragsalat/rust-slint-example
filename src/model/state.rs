use super::actions::{Action, StateChange};
use crate::{mock::{LibraryEntry, Player, Progress}, with_getters_setters};
use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};

type Changes = Vec<Field>;

with_getters_setters! {
    #[derive(Default)]
    pub struct InnerState {
        /// List of library entries
        pub messages: Vec<String>,
        pub active_library_entry: Option<LibraryEntry>,
        pub playing_library_entry: Option<LibraryEntry>,
        pub is_playing: bool,
        pub progress: Progress,
    }

    pub struct State {
        subscribers: Arc<Mutex<Vec<Sender<Changes>>>>,
        action_tx: Arc<Mutex<Sender<Action>>>,
        pub(super) player: Player,
    }
}

impl State {
    pub fn new(player: Player) -> Self {
        let (tx, rx) = channel::<Action>();
        let subscribers = Arc::new(Mutex::new(Vec::new()));
        let self_ = Self {
            inner: Arc::new(Mutex::new(InnerState::default())),
            subscribers,
            action_tx: Arc::new(Mutex::new(tx)),
            player
        };

        {
            let mut self_clone = self_.clone();
            tokio::spawn(async move {
                for action in rx {
                    println!("Received action: {:?}", action);
                    match action {
                        Action::LoadLibraryEntry(id) => self_clone.load_library_entry(id),
                        Action::PlayLibraryEntry(library_entry) => self_clone.play_library_entry(library_entry),
                        Action::TogglePlay(is_playing) => self_clone.toggle_play(is_playing),
                        Action::SetProgress(progress) => todo!(),
                    };

                    // Notify subscribers about the state change
                    let changes = self_clone.inner.lock().unwrap().changes.clone();
                    let subs = self_clone.subscribers.lock().unwrap();
                    for sub in subs.iter() {
                        sub.send(changes.clone()).unwrap();
                    }
                }
            });
        }

        self_
    }

    pub fn dispatch(&self, action: Action) {
        let tx = self.action_tx.lock().unwrap();
        tx.send(action).unwrap();
    }

    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(Changes) + Send + 'static,
    {
        let (tx, rx) = channel::<Changes>();
        {
            let mut subs = self.subscribers.lock().unwrap();
            subs.push(tx);
        }

        std::thread::spawn(move || {
            for change in rx {
                callback(change);
            }
        });
    }
}
