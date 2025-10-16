use std::fmt::Debug;

#[derive(Clone)]
pub enum Variant {
    Folder,
    Stream,
    File,
    Spotify,
} 

impl Debug for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Folder => write!(f, "Folder"),
            Variant::Stream => write!(f, "Stream"),
            Variant::File => write!(f, "File"),
            Variant::Spotify => write!(f, "Spotify"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TrackSource {
    pub id: i32,
    pub library_entry_id: Option<i32>,
    pub title: String,
    pub url: Option<String>,
    pub file: Option<Vec<u8>>,
    pub spotify_id: Option<String>,
    pub spotify_type: Option<String>,
}

#[derive(Clone, Debug)]
pub struct LibraryEntry {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub variant: Variant,
    pub name: String,
    pub image: Option<Vec<u8>>,
    pub played_at: Option<chrono::DateTime<chrono::Utc>>,
    pub sort_key: i32,
    pub children: Option<Vec<LibraryEntry>>, // Just used to pass children from API to client
    pub track_source: Option<TrackSource>, // Just used to pass children from API to client
    // Only relevant for the user interface
    pub parent_name: Option<String>,
    // Only relevant for the user interface
    pub parent_image: Option<Vec<u8>>,
}

impl LibraryEntry {
    pub fn new_folder(id: i32, name: &str, parent_id: Option<i32>, image: String, sort_key: i32) -> Self {
        LibraryEntry {
            id,
            parent_id,
            variant: Variant::Folder,
            name: name.to_string(),
            image: load_image(image),
            played_at: None,
            sort_key,
            children: Some(vec![]),
            track_source: None,
            parent_name: None,
            parent_image: None,
        }
    }

    pub fn new_spotify(
        id: i32,
        name: &str,
        parent_id: Option<i32>,
        sort_key: i32,
    ) -> Self {
        LibraryEntry {
            id,
            parent_id,
            variant: Variant::Spotify,
            name: name.to_string(),
            image: None,
            played_at: None,
            sort_key,
            children: None,
            track_source: Some(TrackSource {
                id: 1,
                library_entry_id: None,
                title: name.to_string(),
                url: None,
                file: None,
                spotify_id: Some("some_id".to_string()),
                spotify_type: Some("track".to_string()),
            }),
            parent_name: None,
            parent_image: None,
        }
    }
}

fn load_image(name: String) -> Option<Vec<u8>> {
    let path = format!("{}/images/{}", std::env::current_dir().ok()?.display(), name);
    println!("Loading image from path: {}", path);
    std::fs::read(path).ok()
}

pub fn get_library_entry(id: i32) -> Option<LibraryEntry> {
    let entries = vec![        
        LibraryEntry::new_folder(0, "Root", Some(0), "".to_string(), 0),
        LibraryEntry::new_folder(1, "Hoerspiele", Some(0), "hoerspiele.jpeg".to_string(), 0),
        LibraryEntry::new_folder(2, "Radio", Some(0), "radio.jpeg".to_string(), 1),
        LibraryEntry::new_folder(3, "Musik", Some(0), "musik.jpeg".to_string(), 2),
        LibraryEntry::new_folder(4, "Aquaparty", Some(3), "aquaparty.jpeg".to_string(), 0),
        LibraryEntry::new_folder(5, "Boomschakalaka", Some(1), "boomschakalaka.jpeg".to_string(), 0),
        LibraryEntry::new_folder(6, "Kitahits", Some(3), "kitahits.jpeg".to_string(), 1),
        LibraryEntry::new_spotify(7, "Spotify Track 1", Some(4), 0),
        LibraryEntry::new_spotify(8, "Spotify Track 2", Some(4), 1),
        LibraryEntry::new_spotify(9, "Spotify Track 3", Some(4), 2),
        LibraryEntry::new_spotify(10, "Spotify Track 4", Some(6), 0),
        LibraryEntry::new_spotify(11, "Spotify Track 5", Some(6), 1),
        LibraryEntry::new_spotify(12, "Spotify Track 6", Some(6), 2),
    ];

    let entry = entries.iter().find(|e| e.id == id).cloned();
    if let Some(mut entry) = entry {
        let children = entries.iter().filter(|e| e.parent_id == Some(entry.id)).cloned().collect();
        entry.children = Some(children);
        return Some(entry);
    }

    None
}