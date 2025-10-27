use slint::{ComponentHandle, Weak};

use crate::{model::State, AppWindow, Navbar};


pub struct NavbarVM {
    ui: Weak<AppWindow>,
    state: State,
}


impl NavbarVM {
    pub fn new(ui: Weak<AppWindow>, state: State) -> Self {
        let vm = NavbarVM { ui, state };
        vm.setup_ui();
        vm.setup_state_listeners();
        vm
    }

    pub fn setup_ui(&self) {
        // Setup UI logic here
        if let Some(ui) = self.ui.upgrade() {
            let navbar_global = ui.global::<Navbar>();
            navbar_global.set_visible(false);

            {
                let state_ = self.state.clone();
                navbar_global.on_go_back(move |parent_id| {
                    state_.dispatch(crate::model::actions::Action::LoadLibraryEntry(parent_id));
                });
            }
        }
    }

    pub fn setup_state_listeners(&self) {
        // Clone for move closure
        let ui_weak = self.ui.clone();
        let state_clone = self.state.clone();
        
        // Subscribe to state changes
        self.state.subscribe(move |change| {
            match change.changed_fields.as_slice() {
                fields if fields.contains(&"active_library_entry".to_string()) => {
                    // Update the UI with the new library entry
                    let entry = state_clone.active_library_entry();
                    let ui_weak_clone = ui_weak.clone();

                    if entry.is_none() {
                        println!("No library entry available");
                        return;
                    }

                    if entry.is_none() {
                        println!("No library entry available");
                        return;
                    }

                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_weak_clone.upgrade() {
                            println!("UI upgrade successful!");
                            let navbar = ui.global::<Navbar>();
                            let entry = entry.unwrap();
                            navbar.set_visible(entry.id != 0);
                            navbar.set_entry_name(entry.name.into());
                            navbar.set_parent_id(entry.parent_id.unwrap_or(0));
                        }
                    });
                }
                _ => {}
            }
        });
    }
}