use crate::{model::State, AppWindow, Content};
use slint::{ComponentHandle, ModelRc, VecModel, Weak};
use crate::mock::LibraryEntry;
use crate::model::Field;

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
        if let Some(ui) = self.ui.upgrade() {
            let content = ui.global::<Content>();

            {
                let state_ = self.state.clone();
                content.on_select_library_entry(move |id| {
                    state_.dispatch(crate::model::actions::Action::LoadLibraryEntry(id));
                });
            }
        }
    }

    pub fn setup_state_listeners(&self) {
        // Clone for move closure
        let ui_weak = self.ui.clone();
        let state = self.state.clone();
        
        // Subscribe to state changes
        self.state.subscribe(move |changes| {
            let has_active_library_entry = changes.iter().any(|field| matches!(field, Field::active_library_entry(_)));
            match change.changed_fields.as_slice() {
                fields if fields.contains(&"active_library_entry".to_string()) => {
                    // Update the UI with the new library entry
                    let entry = state.active_library_entry();
                    let ui_weak = ui_weak.clone();

                    if entry.is_none() {
                        println!("No library entry available");
                        return;
                    }

                    let state = state.clone();
                    // UI Updates müssen auf dem Hauptthread stattfinden
                    slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_weak.upgrade() {
                            println!("UI upgrade successful!");
                            let content = ui.global::<Content>();
                            
                            let children = entry.unwrap().children;
                            if children.is_none() {
                                return;
                            }

                            let children = children.unwrap();
                            let variant = children.first().map(|e| format!("{:?}", e.variant)).unwrap_or_default();
                            content.set_variant(variant.to_owned().into());

                            if variant == "folder" || variant == "stream" {
                                println!("Setting tile view data");
                                Self::set_tile_view_data(&content, children, state);
                            } else {
                                println!("Setting detail view data");
                                Self::set_detail_view_data(&content, children, state);
                            }
                        } else {
                            println!("UI has been dropped");
                        }
                    }).ok();
                }
                _ => {}
            }
        });
    }

    fn set_tile_view_data(ui: &Content<'_>, entries: Vec<LibraryEntry>, state: State) {
        // Convert entries to the format expected by the UI
        let rows_data: Vec<Vec<_>> = entries
            .chunks(3) // Group into rows of 3 items
            .map(|chunk| {
                chunk.iter().map(|entry| Self::map_library_entry_to_ui(entry, &state)).collect()
            })
            .collect();
        
        let rows_model = ModelRc::new(VecModel::from(
            rows_data.into_iter()
                .map(|row| ModelRc::new(VecModel::from(row)))
                .collect::<Vec<_>>()
        ));
        
        ui.set_tile_rows(rows_model);
        ui.set_detail_rows(ModelRc::default());
    }

    fn set_detail_view_data(ui: &Content<'_>, entries: Vec<LibraryEntry>, state: State) {
        // Convert entries to the format expected by the UI
        let rows_data: Vec<_> = entries.iter().map(|entry| Self::map_library_entry_to_ui(entry, &state)).collect();
        
        let rows_model = ModelRc::new(VecModel::from(rows_data));
        
        ui.set_tile_rows(ModelRc::default());
        ui.set_detail_rows(rows_model);
    }

    fn map_library_entry_to_ui(entry: &LibraryEntry, state: &State) -> crate::UILibraryEntry {
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
            is_playing: state.is_playing(),
            is_loaded: match state.playing_library_entry() {
                Some(playing_library_entry) if playing_library_entry.id == entry.id => true,
                _ => false,
            },
            play_progress: 0,
        }
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