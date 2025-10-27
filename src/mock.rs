use std::{fmt::Debug, sync::{mpsc::{channel, Sender}, Arc, Mutex}, thread::spawn, time::Duration};

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
            Variant::Folder => write!(f, "folder"),
            Variant::Stream => write!(f, "stream"),
            Variant::File => write!(f, "file"),
            Variant::Spotify => write!(f, "spotify"),
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
        LibraryEntry::new_folder(0, "Root", Some(-1), "".to_string(), 0),
        LibraryEntry::new_folder(1, "Hoerspiele", Some(0), "hoerspiele.jpeg".to_string(), 0),
        LibraryEntry::new_folder(2, "Radio", Some(0), "radio.jpeg".to_string(), 1),
        LibraryEntry::new_folder(3, "Musik", Some(0), "musik.jpeg".to_string(), 2),
        LibraryEntry::new_folder(13, "Hoerspiele", Some(0), "hoerspiele.jpeg".to_string(), 0),
        LibraryEntry::new_folder(14, "Radio", Some(0), "radio.jpeg".to_string(), 1),
        LibraryEntry::new_folder(15, "Musik", Some(0), "musik.jpeg".to_string(), 2),
        LibraryEntry::new_folder(16, "Hoerspiele", Some(0), "hoerspiele.jpeg".to_string(), 0),
        LibraryEntry::new_folder(17, "Radio", Some(0), "radio.jpeg".to_string(), 1),
        LibraryEntry::new_folder(18, "Musik", Some(0), "musik.jpeg".to_string(), 2),
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

#[derive(Default)]
struct PlayerInner {
    is_playing: bool,
    current_position: Duration,
    current_track: Option<LibraryEntry>,
    volume: f32,
    subscribers: Vec<Sender<PlayerEvent>>
}

#[derive(Clone)]
pub struct Player {
    inner: Arc<Mutex<PlayerInner>>
}

impl Player {
    pub fn new() -> Self {
        Player {
            inner: Arc::new(Mutex::new(PlayerInner::default()))
        }
    }

    async fn random_delay(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let delay_ms = (now % 1000);
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    }

    pub async fn play(&self, library_entry: LibraryEntry) -> Result<(), String> {
        self.random_delay().await;
        println!("Playing {:?}", library_entry);

        let mut inner = self.inner.lock().unwrap();
        inner.is_playing = true;
        inner.current_track = Some(library_entry.clone());
        inner.current_position = Duration::ZERO;

        self.notify(PlayerEvent::Playing(library_entry));

        Ok(())
    }

    pub async fn pause(&self) -> Result<(), String> {
        self.random_delay().await;

        let mut inner = self.inner.lock().unwrap();
        inner.is_playing = false;

        self.notify(PlayerEvent::Paused);

        Ok(())
    }

    pub async fn resume(&self) -> Result<(), String> {
        self.random_delay().await;

        let mut inner = self.inner.lock().unwrap();
        if inner.current_track.is_none() {
            return Err("No track loaded".to_string());
        }

        inner.is_playing = true;

        self.notify(PlayerEvent::Resumed);

        Ok(())
    }

    pub async fn seek(&self, position: Duration) -> Result<(), String> {
        self.random_delay().await;

        let mut inner = self.inner.lock().unwrap();
        inner.current_position = position;

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), String> {
        self.random_delay().await;

        let mut inner = self.inner.lock().unwrap();
        inner.current_track = None;
        inner.is_playing = false;

        self.notify(PlayerEvent::Stopped);

        Ok(())
    }

    pub async fn set_volume(&self, volume: f32) -> Result<(), String> {
        self.random_delay().await;

        let mut inner = self.inner.lock().unwrap();
        inner.volume = volume;

        Ok(())
    }

    fn notify(&self, event: PlayerEvent) {
        let inner = self.inner.lock().unwrap();
        for subscriber in inner.subscribers.iter() {
            subscriber.send(event.clone()).unwrap();
        }
    }

    pub fn subscribe<C>(&self, callback: C) -> Result<(), String>
    where
        C: Fn(PlayerEvent) + Send + 'static
    {
        let (sender, receiver) = channel::<PlayerEvent>();

        let mut inner = self.inner.lock().unwrap();
        inner.subscribers.push(sender);

        spawn(move || {
            for event in receiver {
                callback(event);
            }
        });

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Progress {
    position: Duration,
    duration: Duration
}

impl Default for Progress {
    fn default() -> Self {
        Self { position: Duration::ZERO, duration: Duration::from_secs(100)}
    }
}

#[derive(Clone)]
pub struct PlayerState {
    is_playing: bool,
    loaded_track: LibraryEntry
}

#[derive(Clone)]
pub enum PlayerEvent {
    PositionChanged(Progress),
    Playing(LibraryEntry),
    Paused,
    Resumed,
    Stopped
}