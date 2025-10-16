use super::actions::{Action, StateChange};
use crate::{mock::{get_library_entry, LibraryEntry}, with_getters_setters};
// use database::model::library_entry::Model as LibraryEntry;
use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};

with_getters_setters! {
    pub struct InnerState {
        /// List of library entries
        pub library_entry: Option<LibraryEntry>,
        pub messages: Vec<String>,
    }

    pub struct State {
        subscribers: Arc<Mutex<Vec<Sender<StateChange>>>>,
        action_tx: Arc<Mutex<Sender<Action>>>,
    }
}

impl State {
    pub fn new() -> Self {
        let (tx, rx) = channel::<Action>();
        let subscribers = Arc::new(Mutex::new(Vec::new()));
        let self_ = Self {
            inner: Arc::new(Mutex::new(InnerState {
                library_entry: None,
                messages: Vec::new(),
            })),
            subscribers,
            action_tx: Arc::new(Mutex::new(tx)),
        };

        {
            let mut self_clone = self_.clone();
            tokio::spawn(async move {
                for action in rx {
                    println!("Received action: {:?}", action);
                    let state_change = match action {
                        Action::LoadLibraryEntry(id) => self_clone.load_library_entry(id),
                    };

                    // Notify subscribers about the state change
                    let subs = self_clone.subscribers.lock().unwrap();
                    for sub in subs.iter() {
                        sub.send(state_change.clone()).unwrap();
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
        F: Fn(StateChange) + Send + 'static,
    {
        let (tx, rx) = channel::<StateChange>();
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
