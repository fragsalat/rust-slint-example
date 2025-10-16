use slint::{ComponentHandle, Weak};
use crate::{AppWindow, Messages};
use crate::model::State;

pub struct MessagesVM {
    ui: Weak<AppWindow>,
    state: State,
}

impl MessagesVM {
    pub fn new(ui: Weak<AppWindow>, state: State) -> Self {
        let vm = MessagesVM { ui, state };
        vm.setup_ui();
        vm.setup_state_listeners();
        vm
    }

    pub fn setup_ui(&self) {
        // Setup UI logic here
        if let Some(ui) = self.ui.upgrade() {
            let messages_global = ui.global::<Messages>();
            // Initialize with empty data
            messages_global.set_messages(slint::ModelRc::new(slint::VecModel::default()));
        }
    }

    pub fn setup_state_listeners(&self) {
        // Clone for move closure
        let ui_weak = self.ui.clone();
        let state_clone = self.state.clone();
        
        // Subscribe to state changes
        self.state.subscribe(move |change| {
            println!("State changed: {:?}", change);
            match change.changed_fields.as_slice() {
                fields if fields.contains(&"messages".to_string()) => {
                    // Clone again for the event loop closure
                    let ui_weak_clone = ui_weak.clone();
                    let state_clone_inner = state_clone.clone();
                        
                    slint::invoke_from_event_loop(move || {
                        // Update the UI with the new messages
                        let messages = state_clone_inner.messages();
                        if let Some(ui) = ui_weak_clone.upgrade() {
                            println!("UI has been upgraded");
                            let messages_global = ui.global::<Messages>();
                            
                            // Convert messages to SharedString format
                            let messages_data: Vec<slint::SharedString> = messages
                                .iter()
                                .map(|msg| msg.clone().into())
                                .collect();
                            
                            messages_global.set_messages(slint::ModelRc::new(slint::VecModel::from(messages_data)));
                            println!("Messages updated in UI");
                        } else {
                            println!("UI has been dropped");
                        }
                    }).ok();
                }
                _ => {}
            }
        });
    }
}

impl Clone for MessagesVM {
    fn clone(&self) -> Self {
        MessagesVM {
            ui: self.ui.clone(),
            state: self.state.clone(),
        }
    }
}