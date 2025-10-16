use crate::{model::State, AppWindow, Content};
use slint::{ComponentHandle, ModelRc, VecModel, Weak};

pub struct ContentVM {
    ui: Weak<AppWindow>,
    state: State,
}

impl ContentVM {
    pub fn new(ui: Weak<AppWindow>, state: State) -> Self {
        let vm = ContentVM { ui, state };
        vm.setup_ui();
        vm.setup_state_listeners();
        vm
    }

    pub fn setup_ui(&self) {
        // Setup UI logic here
        if let Some(ui) = self.ui.upgrade() {
            let content_global = ui.global::<Content>();
            // Initialize with empty data
            content_global.set_rows(ModelRc::new(VecModel::default()));
        }
    }

    pub fn setup_state_listeners(&self) {
        // Clone for move closure
        let ui_weak = self.ui.clone();
        let state_clone = self.state.clone();
        
        // Subscribe to state changes
        self.state.subscribe(move |change| {
            match change.changed_fields.as_slice() {
                fields if fields.contains(&"library_entry".to_string()) => {
                    // Update the UI with the new library entry
                    let entry = state_clone.library_entry();
                    let ui_weak_clone = ui_weak.clone();

                    if entry.is_none() {
                        println!("No library entry available");
                        return;
                    }
                    
                    // UI Updates müssen auf dem Hauptthread stattfinden
                    slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_weak_clone.upgrade() {
                            println!("UI upgrade successful!");
                            let content_global = ui.global::<Content>();
                            
                            // Convert entries to the format expected by the UI
                            let rows_data: Vec<Vec<_>> = entry.unwrap().children.unwrap()
                                .chunks(3) // Group into rows of 3 items
                                .map(|chunk| {
                                    chunk.iter().map(|entry| {
                                        // Convert LibraryEntry to UILibraryEntry format
                                        crate::UILibraryEntry {
                                            id: entry.id,
                                            parent_id: entry.parent_id.unwrap_or(0),
                                            variant: format!("{:?}", entry.variant).into(),
                                            name: entry.name.clone().into(),
                                            played_at: entry.played_at
                                                .map(|dt| dt.format("%Y-%m-%d").to_string())
                                                .unwrap_or_default().into(),
                                            image: entry.image.clone()
                                                .map(|img| ModelRc::new(VecModel::from(
                                                    img.into_iter().map(|b| b as i32).collect::<Vec<_>>()
                                                )))
                                                .unwrap_or_else(|| ModelRc::new(VecModel::default())),
                                            sort_key: entry.sort_key,
                                        }
                                    }).collect()
                                })
                                .collect();
                            
                            let rows_model = ModelRc::new(VecModel::from(
                                rows_data.into_iter()
                                    .map(|row| ModelRc::new(VecModel::from(row)))
                                    .collect::<Vec<_>>()
                            ));
                            
                            content_global.set_rows(rows_model);
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

impl Clone for ContentVM {
    fn clone(&self) -> Self {
        ContentVM {
            ui: self.ui.clone(),
            state: self.state.clone(),
        }
    }
}