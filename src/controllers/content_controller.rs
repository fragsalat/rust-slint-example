use std::sync::{Arc, Mutex};

use slint::{VecModel, ModelRc, SharedString};

use crate::AppWindow;
use crate::model::{Action, State};
use crate::mock::{LibraryEntry, get_library_entry};

use crate::UILibraryEntry;

/*
#[derive(ViewModel)]
struct ViewModel {
    #[ui_property]
    library_entries: ModelRc<VecModel<UILibraryEntry>>,
}
*/

#[derive(Clone)]
pub struct ContentController {
    ui: Arc<Mutex<AppWindow>>,
    state: State,
}

impl ContentController {
    pub fn new(ui: Arc<Mutex<AppWindow>>, state: State) -> Self {
        let ctrl = ContentController {
            ui,
            state: state.clone(),
        };

        let self_ = ctrl.clone();
        state.subscribe(move |change| {
            match change.changed_fields.as_slice() {
                fields if fields.contains(&"library_entries".to_string()) => {
                    // Update the UI with the new library entries
                    let entries = self_.state.library_entries();
                    // Assuming there's a method in the UI to update the list
                    // ui.update_library_entries(entries);
                }
                // Handle different state changes if needed
                _ => {}
            }
        });

        ctrl
    }

    pub fn init(&self) {
        // Load initial data if necessary
        self.state.dispatch(Action::LoadLibraryEntry(0));
    }

    pub fn update(&self) {
        let mut rows = VecModel::default();
        for chunk in self.state.library_entries().chunks(3) {
            let row = chunk
                .iter()
                .map(|child| UILibraryEntry {
                    id: child.id,
                    image: ModelRc::new(VecModel::from(
                        child
                            .image
                            .clone()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|b| b as i32)
                            .collect::<Vec<i32>>(),
                    )),
                    name: child.name.clone().into(),
                    parent_id: child.parent_id.clone().unwrap_or(-1),
                    played_at: SharedString::from(
                        child
                            .played_at
                            .as_ref()
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or_default(),
                    ),
                    sort_key: child.sort_key,
                    variant: SharedString::from(format!("{:?}", child.variant)),
                })
                .collect::<Vec<UILibraryEntry>>();
            rows.push(ModelRc::new(VecModel::from(row)));
        }


        self.ui.set_rows(ModelRc::new(rows));
    }

    pub fn on_select(&self, id: i32) {
        println!("Selected item with id: {}", id);
        self.state.dispatch(Action::LoadLibraryEntry(id));
    }
}
