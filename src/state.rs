use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::json::{parse_song, SongJson, Value};

pub const SONGS_DIR: &str = "songs";
pub const PLAYLISTS_DIR: &str = "playlists";
pub const LOG_DIR: &str = "log";
pub const LOG_FILE: &str = "log/songbookclear.log";
pub const THEMES_FILE: &str = "themes.json";

pub fn log_warn(message: &str) {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let line = format!("[{ts}] [WARN] {message}\n");

    if fs::create_dir_all(LOG_DIR).is_ok() {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(LOG_FILE) {
            if file.write_all(line.as_bytes()).is_ok() {
                return;
            }
        }
    }
    eprint!("{line}");
}

#[derive(Clone)]
pub struct SlideTheme {
    pub name: String,
    pub bg: String,
    pub text: String,
    pub chord: String,
    pub show_chords: bool,
}

impl Default for SlideTheme {
    fn default() -> Self {
        Self {
            name: "Ciemny klasyczny".to_string(),
            bg: "#121417".to_string(),
            text: "#f9fafb".to_string(),
            chord: "#fbbf24".to_string(),
            show_chords: true,
        }
    }
}

impl SlideTheme {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"name":"{}","bg":"{}","text":"{}","chord":"{}","showChords":{}}}"#,
            escape_json(&self.name),
            escape_json(&self.bg),
            escape_json(&self.text),
            escape_json(&self.chord),
            self.show_chords
        )
    }
}

#[derive(Clone)]
pub struct PlaylistRecord {
    pub filename: String,
    pub name: String,
    pub date: String,
    pub time: String,
    pub songs: Vec<String>,
}

impl PlaylistRecord {
    pub fn schedule_label(&self) -> String {
        let mut parts = Vec::new();
        if !self.date.trim().is_empty() {
            parts.push(self.date.trim().to_string());
        }
        if !self.time.trim().is_empty() {
            parts.push(self.time.trim().to_string());
        }
        parts.join(" ")
    }

    pub fn to_json(&self) -> String {
        let items: Vec<String> = self
            .songs
            .iter()
            .map(|f| format!("\"{}\"", escape_json(f)))
            .collect();
        let schedule = self.schedule_label();
        format!(
            r#"{{"filename":"{}","name":"{}","date":"{}","time":"{}","schedule":"{}","songs":[{}]}}"#,
            escape_json(&self.filename),
            escape_json(&self.name),
            escape_json(&self.date),
            escape_json(&self.time),
            escape_json(&schedule),
            items.join(",")
        )
    }
}

/// Zaszyty w programie wzorzec struktury JSON dla nowej playlisty.
pub fn build_playlist_json_from_template(
    name: &str,
    date: &str,
    time: &str,
    songs: &[String],
) -> String {
    let items: Vec<String> = songs
        .iter()
        .map(|f| format!("\"{}\"", escape_json(f)))
        .collect();
    format!(
        concat!(
            "{{\n",
            "  \"name\": \"{}\",\n",
            "  \"date\": \"{}\",\n",
            "  \"time\": \"{}\",\n",
            "  \"songs\": [{}]\n",
            "}}\n"
        ),
        escape_json(name.trim()),
        escape_json(date.trim()),
        escape_json(time.trim()),
        items.join(", ")
    )
}

#[derive(Clone)]
pub struct SongRecord {
    pub filename: String,
    pub data: SongJson,
    pub slides: Vec<String>,
    pub slide_durations: Vec<u64>,
}

#[derive(Clone)]
pub struct AppState {
    pub all_songs: Vec<SongRecord>,
    pub songs: Vec<SongRecord>,
    pub playlists: Vec<PlaylistRecord>,
    pub active_playlist: Option<String>,
    pub current_song: usize,
    pub current_slide: usize,
    pub local_ip: String,
    pub port: u16,
    pub server_running: bool,
    pub active_theme: SlideTheme,
    pub saved_themes: Vec<SlideTheme>,
}

pub enum Action {
    NextSlide,
    PrevSlide,
    SelectSong(usize),
}

impl AppState {
    pub fn new(port: u16) -> Self {
        if let Err(e) = fs::create_dir_all(SONGS_DIR) {
            log_warn(&format!("Nie udało się utworzyć katalogu '{SONGS_DIR}': {e}"));
        }
        if let Err(e) = fs::create_dir_all(PLAYLISTS_DIR) {
            log_warn(&format!(
                "Nie udało się utworzyć katalogu '{PLAYLISTS_DIR}': {e}"
            ));
        }
        if let Err(e) = fs::create_dir_all(LOG_DIR) {
            eprintln!("[WARN] Nie udało się utworzyć katalogu '{LOG_DIR}': {e}");
        }
        let all_songs = load_songs_from_dir(SONGS_DIR);
        let playlists = load_playlists_from_dir(PLAYLISTS_DIR);
        let active_playlist = playlists.first().map(|p| p.filename.clone());
        let songs = resolve_active_songs(&all_songs, &playlists, active_playlist.as_deref());
        let saved_themes = load_themes_from_file(THEMES_FILE);
        let active_theme = saved_themes.first().cloned().unwrap_or_default();

        Self {
            all_songs,
            songs,
            playlists,
            active_playlist,
            current_song: 0,
            current_slide: 0,
            local_ip: detect_local_ip().unwrap_or_else(|| "127.0.0.1".to_string()),
            port,
            server_running: true,
            active_theme,
            saved_themes,
        }
    }

    pub fn active_playlist_record(&self) -> Option<&PlaylistRecord> {
        let active_file = self.active_playlist.as_deref()?;
        self.playlists.iter().find(|p| p.filename == active_file)
    }

    pub fn select_playlist(&mut self, filename: &str) -> Result<(), String> {
        let safe = sanitize_filename(filename)?;
        if !self.playlists.iter().any(|p| p.filename == safe) {
            self.playlists = load_playlists_from_dir(PLAYLISTS_DIR);
        }
        if !self.playlists.iter().any(|p| p.filename == safe) {
            return Err("Nie znaleziono wybranej playlisty.".to_string());
        }
        self.active_playlist = Some(safe);
        self.songs =
            resolve_active_songs(&self.all_songs, &self.playlists, self.active_playlist.as_deref());
        self.current_song = 0;
        self.current_slide = 0;
        Ok(())
    }

    pub fn save_playlist(
        &mut self,
        name: &str,
        date: &str,
        time: &str,
        song_filenames: Vec<String>,
    ) -> Result<String, String> {
        let clean_name = name.trim();
        if clean_name.is_empty() {
            return Err("Nazwa playlisty jest wymagana.".to_string());
        }

        fs::create_dir_all(PLAYLISTS_DIR).map_err(|e| {
            let msg = format!("Nie można utworzyć katalogu {PLAYLISTS_DIR}: {e}");
            log_warn(&msg);
            msg
        })?;

        let valid_song_files: Vec<String> = song_filenames
            .into_iter()
            .filter(|f| self.all_songs.iter().any(|s| &s.filename == f))
            .collect();

        let target_filename = generate_unique_filename(PLAYLISTS_DIR, clean_name);
        let json_body = build_playlist_json_from_template(
            clean_name,
            date,
            time,
            &valid_song_files,
        );

        let path = Path::new(PLAYLISTS_DIR).join(&target_filename);
        fs::write(&path, json_body).map_err(|e| {
            let msg = format!("Błąd zapisu playlisty {target_filename}: {e}");
            log_warn(&msg);
            msg
        })?;

        self.playlists = load_playlists_from_dir(PLAYLISTS_DIR);
        self.active_playlist = Some(target_filename.clone());
        self.songs =
            resolve_active_songs(&self.all_songs, &self.playlists, self.active_playlist.as_deref());
        self.current_song = 0;
        self.current_slide = 0;
        Ok(target_filename)
    }

    pub fn delete_playlist(&mut self, filename: &str) -> Result<(), String> {
        let safe = sanitize_filename(filename)?;
        let path = Path::new(PLAYLISTS_DIR).join(&safe);
        if path.exists() {
            fs::remove_file(&path).map_err(|e| {
                let msg = format!("Nie udało się usunąć playlisty {safe}: {e}");
                log_warn(&msg);
                msg
            })?;
        }

        self.playlists = load_playlists_from_dir(PLAYLISTS_DIR);
        if self.active_playlist.as_deref() == Some(safe.as_str()) {
            self.active_playlist = self.playlists.first().map(|p| p.filename.clone());
        }
        self.songs =
            resolve_active_songs(&self.all_songs, &self.playlists, self.active_playlist.as_deref());
        self.current_song = 0;
        self.current_slide = 0;
        Ok(())
    }

    pub fn playlists_json(&self) -> String {
        let items: Vec<String> = self.playlists.iter().map(|p| p.to_json()).collect();
        let active = self.active_playlist.as_deref().unwrap_or("");
        format!(
            r#"{{"activeFilename":"{}","playlists":[{}]}}"#,
            escape_json(active),
            items.join(",")
        )
    }

    pub fn set_theme_live(
        &mut self,
        bg: Option<String>,
        text: Option<String>,
        chord: Option<String>,
        show_chords: Option<bool>,
    ) {
        if let Some(c) = bg {
            if !c.trim().is_empty() {
                self.active_theme.bg = c.trim().to_string();
            }
        }
        if let Some(c) = text {
            if !c.trim().is_empty() {
                self.active_theme.text = c.trim().to_string();
            }
        }
        if let Some(c) = chord {
            if !c.trim().is_empty() {
                self.active_theme.chord = c.trim().to_string();
            }
        }
        if let Some(sc) = show_chords {
            self.active_theme.show_chords = sc;
        }
    }

    pub fn save_theme_preset(&mut self, name: &str) -> Result<(), String> {
        let clean_name = name.trim();
        if clean_name.is_empty() {
            return Err("Podaj nazwę motywu.".to_string());
        }
        let mut preset = self.active_theme.clone();
        preset.name = clean_name.to_string();
        self.active_theme = preset.clone();

        if let Some(existing) = self
            .saved_themes
            .iter_mut()
            .find(|t| t.name.eq_ignore_ascii_case(clean_name))
        {
            *existing = preset;
        } else {
            self.saved_themes.push(preset);
        }

        save_themes_to_file(THEMES_FILE, &self.saved_themes)
    }

    pub fn select_theme_preset(&mut self, name: &str) -> Result<(), String> {
        let clean_name = name.trim();
        if let Some(found) = self
            .saved_themes
            .iter()
            .find(|t| t.name.eq_ignore_ascii_case(clean_name))
            .cloned()
        {
            self.active_theme = found;
            Ok(())
        } else {
            Err("Nie znaleziono wybranego motywu.".to_string())
        }
    }

    pub fn themes_json(&self) -> String {
        let items: Vec<String> = self.saved_themes.iter().map(|t| t.to_json()).collect();
        format!(
            r#"{{"active":{},"themes":[{}]}}"#,
            self.active_theme.to_json(),
            items.join(",")
        )
    }

    pub fn reload_songs(&mut self, select_filename: Option<&str>) {
        let prev_filename = select_filename
            .map(|s| s.to_string())
            .or_else(|| self.songs.get(self.current_song).map(|s| s.filename.clone()));

        self.all_songs = load_songs_from_dir(SONGS_DIR);
        self.playlists = load_playlists_from_dir(PLAYLISTS_DIR);
        self.songs =
            resolve_active_songs(&self.all_songs, &self.playlists, self.active_playlist.as_deref());

        if self.songs.is_empty() {
            self.current_song = 0;
            self.current_slide = 0;
            return;
        }

        if let Some(target) = prev_filename {
            if let Some(idx) = self.songs.iter().position(|s| s.filename == target) {
                self.current_song = idx;
                let max_slide = self.songs[idx].slides.len().saturating_sub(1);
                if self.current_slide > max_slide {
                    self.current_slide = 0;
                }
                return;
            }
        }

        if self.current_song >= self.songs.len() {
            self.current_song = 0;
            self.current_slide = 0;
        }
    }

    pub fn save_song(&mut self, filename: Option<&str>, song: SongJson) -> Result<String, String> {
        let title = song.title.trim();
        if title.is_empty() {
            return Err("Tytuł piosenki (title) jest wymagany.".to_string());
        }
        if song.lyrics.trim().is_empty() {
            return Err("Tekst piosenki (lyrics) jest wymagany.".to_string());
        }

        fs::create_dir_all(SONGS_DIR).map_err(|e| {
            let msg = format!("Nie można utworzyć katalogu {SONGS_DIR}: {e}");
            log_warn(&msg);
            msg
        })?;

        let target_filename = match filename {
            Some(existing) if !existing.trim().is_empty() => sanitize_filename(existing)?,
            _ => generate_unique_filename(SONGS_DIR, title),
        };

        let path = Path::new(SONGS_DIR).join(&target_filename);
        let json_text = song.to_json_string();
        fs::write(&path, json_text).map_err(|e| {
            let msg = format!("Błąd zapisu pliku {target_filename}: {e}");
            log_warn(&msg);
            msg
        })?;

        self.reload_songs(Some(&target_filename));
        Ok(target_filename)
    }

    pub fn delete_song(&mut self, filename: &str) -> Result<(), String> {
        let safe_name = sanitize_filename(filename)?;
        let path = Path::new(SONGS_DIR).join(&safe_name);
        if path.exists() {
            fs::remove_file(&path).map_err(|e| {
                let msg = format!("Nie udało się usunąć pliku {safe_name}: {e}");
                log_warn(&msg);
                msg
            })?;
        }
        self.reload_songs(None);
        Ok(())
    }

    pub fn apply(&mut self, action: Action) {
        if self.songs.is_empty() {
            return;
        }
        match action {
            Action::NextSlide => {
                let len = self.songs[self.current_song].slides.len();
                if self.current_slide + 1 < len {
                    self.current_slide += 1;
                } else if self.current_song + 1 < self.songs.len() {
                    self.current_song += 1;
                    self.current_slide = 0;
                } else {
                    self.current_slide = 0;
                }
            }
            Action::PrevSlide => {
                if self.current_slide > 0 {
                    self.current_slide -= 1;
                } else if self.current_song > 0 {
                    self.current_song -= 1;
                    let len = self.songs[self.current_song].slides.len();
                    self.current_slide = len.saturating_sub(1);
                }
            }
            Action::SelectSong(idx) => {
                if idx < self.songs.len() {
                    self.current_song = idx;
                    self.current_slide = 0;
                }
            }
        }
    }

    pub fn to_json(&self, is_local_presenter: bool) -> String {
        let song_items: Vec<String> = self
            .songs
            .iter()
            .enumerate()
            .map(|(i, s)| {
                format!(
                    r#"{{"index":{},"filename":"{}","title":"{}","automatic":{},"nextSongDelaySeconds":{}}}"#,
                    i,
                    escape_json(&s.filename),
                    escape_json(&s.data.title),
                    s.data.automatic,
                    s.data.next_song_delay_seconds
                )
            })
            .collect();

        let theme_json = self.active_theme.to_json();
        let active_pl = self.active_playlist_record();
        let playlist_name = active_pl.map(|p| p.name.as_str()).unwrap_or("");
        let playlist_file = active_pl.map(|p| p.filename.as_str()).unwrap_or("");
        let playlist_schedule = active_pl.map(|p| p.schedule_label()).unwrap_or_default();

        if let Some(song) = self.songs.get(self.current_song) {
            let slide_text = song
                .slides
                .get(self.current_slide)
                .cloned()
                .unwrap_or_default();
            let active_seconds = song
                .slide_durations
                .get(self.current_slide)
                .copied()
                .unwrap_or(song.data.slide_seconds);
            let durations_json: String = song
                .slide_durations
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(",");

            format!(
                r#"{{"canPresent":{},"serverRunning":{},"ip":"{}","port":{},"playlistName":"{}","playlistFile":"{}","playlistSchedule":"{}","songIndex":{},"slideIndex":{},"slideCount":{},"songTitle":"{}","slideText":"{}","automatic":{},"slideSeconds":{},"slideDurations":[{}],"nextSongDelaySeconds":{},"theme":{},"songs":[{}]}}"#,
                is_local_presenter,
                self.server_running,
                escape_json(&self.local_ip),
                self.port,
                escape_json(playlist_name),
                escape_json(playlist_file),
                escape_json(&playlist_schedule),
                self.current_song,
                self.current_slide,
                song.slides.len(),
                escape_json(&song.data.title),
                escape_json(&slide_text),
                song.data.automatic,
                active_seconds,
                durations_json,
                song.data.next_song_delay_seconds,
                theme_json,
                song_items.join(",")
            )
        } else {
            format!(
                r#"{{"canPresent":{},"serverRunning":{},"ip":"{}","port":{},"playlistName":"{}","playlistFile":"{}","playlistSchedule":"{}","songIndex":0,"slideIndex":0,"slideCount":0,"songTitle":"Brak piosenek w aktywnej playliście lub katalogu {}/","slideText":"Wybierz playlistę, utwórz nową lub dodaj utwór w zakładce [Piosenki].","automatic":false,"slideSeconds":0,"slideDurations":[],"nextSongDelaySeconds":3,"theme":{},"songs":[]}}"#,
                is_local_presenter,
                self.server_running,
                escape_json(&self.local_ip),
                self.port,
                escape_json(playlist_name),
                escape_json(playlist_file),
                escape_json(&playlist_schedule),
                SONGS_DIR,
                theme_json
            )
        }
    }

    pub fn songs_catalog_json(&self) -> String {
        let items: Vec<String> = self
            .all_songs
            .iter()
            .enumerate()
            .map(|(i, s)| {
                format!(
                    concat!(
                        "{{",
                        "\"index\":{},",
                        "\"filename\":\"{}\",",
                        "\"title\":\"{}\",",
                        "\"key\":\"{}\",",
                        "\"time_sig\":\"{}\",",
                        "\"author\":\"{}\",",
                        "\"tempo\":\"{}\",",
                        "\"hymn_number\":\"{}\",",
                        "\"presentation\":\"{}\",",
                        "\"lyrics\":\"{}\",",
                        "\"automatic\":{},",
                        "\"slide_seconds\":{},",
                        "\"next_song_delay_seconds\":{}",
                        "}}"
                    ),
                    i,
                    escape_json(&s.filename),
                    escape_json(&s.data.title),
                    escape_json(&s.data.key),
                    escape_json(&s.data.time_sig),
                    escape_json(&s.data.author),
                    escape_json(&s.data.tempo),
                    escape_json(&s.data.hymn_number),
                    escape_json(&s.data.presentation),
                    escape_json(&s.data.lyrics),
                    s.data.automatic,
                    s.data.slide_seconds,
                    s.data.next_song_delay_seconds
                )
            })
            .collect();
        format!("[{}]", items.join(","))
    }
}

fn resolve_active_songs(
    all_songs: &[SongRecord],
    playlists: &[PlaylistRecord],
    active_filename: Option<&str>,
) -> Vec<SongRecord> {
    if let Some(target_file) = active_filename {
        if let Some(pl) = playlists.iter().find(|p| p.filename == target_file) {
            return pl
                .songs
                .iter()
                .filter_map(|song_file| all_songs.iter().find(|s| &s.filename == song_file).cloned())
                .collect();
        }
    }
    all_songs.to_vec()
}

fn load_playlists_from_dir(dir: &str) -> Vec<PlaylistRecord> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("json"))
        .collect();

    files.sort();

    let mut list = Vec::new();
    for path in files {
        let Some(filename) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(Value::Object(map)) = crate::json::parse(&content) else {
            continue;
        };
        let name = match map.get("name") {
            Some(Value::String(s)) if !s.trim().is_empty() => s.trim().to_string(),
            _ => filename.trim_end_matches(".json").to_string(),
        };
        let year_opt = match map.get("year") {
            Some(Value::String(s)) => Some(s.trim().to_string()),
            Some(Value::Number(n)) => Some(n.clone()),
            _ => None,
        };
        let mut date = match map.get("date") {
            Some(Value::String(s)) => s.trim().to_string(),
            _ => String::new(),
        };
        if let Some(y) = year_opt {
            if !y.is_empty() && !date.is_empty() && !date.contains(&y) {
                date = format!("{y}-{date}");
            }
        }
        let time = match map.get("time") {
            Some(Value::String(s)) => s.trim().to_string(),
            _ => String::new(),
        };
        let songs = match map.get("songs") {
            Some(Value::Array(arr)) => arr
                .iter()
                .filter_map(|v| match v {
                    Value::String(s) => Some(s.trim().to_string()),
                    _ => None,
                })
                .filter(|s| !s.is_empty())
                .collect(),
            _ => Vec::new(),
        };

        list.push(PlaylistRecord {
            filename: filename.to_string(),
            name,
            date,
            time,
            songs,
        });
    }

    list
}

fn default_theme_presets() -> Vec<SlideTheme> {
    vec![
        SlideTheme::default(),
        SlideTheme {
            name: "Jasny dzień".to_string(),
            bg: "#f8fafc".to_string(),
            text: "#0f172a".to_string(),
            chord: "#b45309".to_string(),
            show_chords: true,
        },
        SlideTheme {
            name: "Granatowy".to_string(),
            bg: "#0f172a".to_string(),
            text: "#f1f5f9".to_string(),
            chord: "#38bdf8".to_string(),
            show_chords: true,
        },
    ]
}

fn load_themes_from_file(path: &str) -> Vec<SlideTheme> {
    let Ok(content) = fs::read_to_string(path) else {
        return default_theme_presets();
    };
    let Ok(Value::Array(arr)) = crate::json::parse(&content) else {
        return default_theme_presets();
    };
    let mut list = Vec::new();
    for item in arr {
        if let Value::Object(map) = item {
            let name = match map.get("name") {
                Some(Value::String(s)) if !s.trim().is_empty() => s.trim().to_string(),
                _ => continue,
            };
            let bg = match map.get("bg") {
                Some(Value::String(s)) if !s.trim().is_empty() => s.trim().to_string(),
                _ => "#121417".to_string(),
            };
            let text = match map.get("text") {
                Some(Value::String(s)) if !s.trim().is_empty() => s.trim().to_string(),
                _ => "#f9fafb".to_string(),
            };
            let chord = match map.get("chord") {
                Some(Value::String(s)) if !s.trim().is_empty() => s.trim().to_string(),
                _ => "#fbbf24".to_string(),
            };
            let show_chords = match map.get("showChords") {
                Some(Value::Bool(b)) => *b,
                _ => true,
            };
            list.push(SlideTheme {
                name,
                bg,
                text,
                chord,
                show_chords,
            });
        }
    }
    if list.is_empty() {
        default_theme_presets()
    } else {
        list
    }
}

fn save_themes_to_file(path: &str, themes: &[SlideTheme]) -> Result<(), String> {
    let items: Vec<String> = themes.iter().map(|t| t.to_json()).collect();
    let json = format!("[\n  {}\n]\n", items.join(",\n  "));
    fs::write(path, json).map_err(|e| {
        let msg = format!("Błąd zapisu motywów do '{path}': {e}");
        log_warn(&msg);
        msg
    })
}

fn load_songs_from_dir(dir: &str) -> Vec<SongRecord> {
    let Ok(entries) = fs::read_dir(dir) else {
        log_warn(&format!("Nie można odczytać katalogu '{dir}'."));
        return Vec::new();
    };

    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("json"))
        .collect();

    files.sort();

    let mut songs = Vec::new();
    for path in files {
        let Some(filename) = path.file_name().and_then(|n| n.to_str()) else {
            log_warn("Pominięto wpis z niepoprawną nazwą pliku w katalogu piosenek.");
            continue;
        };
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                log_warn(&format!("Błąd odczytu pliku '{dir}/{filename}': {e}"));
                continue;
            }
        };
        match parse_song(&content) {
            Ok(data) => {
                let (slides, slide_durations) = build_slides(&data);
                songs.push(SongRecord {
                    filename: filename.to_string(),
                    data,
                    slides,
                    slide_durations,
                });
            }
            Err(e) => {
                log_warn(&format!("Błąd parsowania JSON w '{dir}/{filename}': {e}"));
            }
        }
    }
    songs
}

fn format_slide_line(raw_line: &str) -> String {
    let trimmed_start = raw_line.trim_start();
    if let Some(after_dot) = trimmed_start.strip_prefix('.') {
        format!("\x01{}", after_dot.trim())
    } else {
        raw_line.trim_end().to_string()
    }
}

fn parse_presentation_token(token: &str) -> (String, Option<u64>) {
    let trimmed = token.trim();
    if let Some((section_part, sec_part)) = trimmed.split_once('.') {
        if let Ok(seconds) = sec_part.trim().parse::<u64>() {
            return (section_part.trim().to_uppercase(), Some(seconds));
        }
    }
    (trimmed.to_uppercase(), None)
}

fn build_slides(song: &SongJson) -> (Vec<String>, Vec<u64>) {
    let normalized = song.lyrics.replace("\r\n", "\n");
    let mut sections: BTreeMap<String, String> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    let mut current_tag: Option<String> = None;
    let mut current_lines: Vec<String> = Vec::new();
    let mut has_tags = false;

    for raw_line in normalized.lines() {
        let trimmed = raw_line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed.len() > 2 {
            if let Some(tag) = current_tag.take() {
                let text = current_lines.join("\n").trim().to_string();
                if !text.is_empty() {
                    sections.insert(tag.clone(), text);
                    order.push(tag);
                }
                current_lines.clear();
            }
            has_tags = true;
            current_tag = Some(trimmed[1..trimmed.len() - 1].trim().to_uppercase());
            continue;
        }

        current_lines.push(format_slide_line(raw_line));
    }

    if let Some(tag) = current_tag {
        let text = current_lines.join("\n").trim().to_string();
        if !text.is_empty() {
            sections.insert(tag.clone(), text);
            order.push(tag);
        }
    }

    let default_sec = song.slide_seconds;

    if has_tags && !sections.is_empty() {
        let mut slides = Vec::new();
        let mut durations = Vec::new();
        let pres_tokens: Vec<&str> = song.presentation.split_whitespace().collect();

        if !pres_tokens.is_empty() {
            for token in pres_tokens {
                let (key, custom_sec) = parse_presentation_token(token);
                let dur = custom_sec.unwrap_or(default_sec);
                if let Some(text) = sections.get(&key) {
                    slides.push(text.clone());
                    durations.push(dur);
                } else if let Some(text) = sections.get(&format!("{}1", key)) {
                    slides.push(text.clone());
                    durations.push(dur);
                }
            }
        }

        if slides.is_empty() {
            for tag in order {
                if let Some(text) = sections.get(&tag) {
                    slides.push(text.clone());
                    durations.push(default_sec);
                }
            }
        }

        if !slides.is_empty() {
            return (slides, durations);
        }
    }

    let fallback: Vec<String> = normalized
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(format_slide_line)
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .collect();

    if fallback.is_empty() {
        (vec![song.title.clone()], vec![default_sec])
    } else {
        let durations = vec![default_sec; fallback.len()];
        (fallback, durations)
    }
}

fn sanitize_filename(name: &str) -> Result<String, String> {
    let base = Path::new(name)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Nieprawidłowa nazwa pliku.".to_string())?;

    if base.is_empty() || base.contains("..") || base.contains('/') || base.contains('\\') {
        return Err("Niedozwolona nazwa pliku.".to_string());
    }
    if !base.ends_with(".json") {
        return Err("Plik musi mieć rozszerzenie .json".to_string());
    }
    Ok(base.to_string())
}

fn generate_unique_filename(dir: &str, title: &str) -> String {
    let slug: String = title
        .chars()
        .map(|c| {
            let mapped = match c {
                'ą' | 'Ą' => 'a',
                'ć' | 'Ć' => 'c',
                'ę' | 'Ę' => 'e',
                'ł' | 'Ł' => 'l',
                'ń' | 'Ń' => 'n',
                'ó' | 'Ó' => 'o',
                'ś' | 'Ś' => 's',
                'ź' | 'Ź' | 'ż' | 'Ż' => 'z',
                other => other.to_ascii_lowercase(),
            };
            if mapped.is_ascii_alphanumeric() {
                mapped
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_");

    let base = if slug.is_empty() {
        "piosenka".to_string()
    } else {
        slug
    };

    let mut candidate = format!("{}.json", base);
    let mut counter = 2;
    while Path::new(dir).join(&candidate).exists() {
        candidate = format!("{}_{}.json", base, counter);
        counter += 1;
    }
    candidate
}

pub fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}

pub fn detect_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    match socket.local_addr().ok()? {
        SocketAddr::V4(v4) => Some(v4.ip().to_string()),
        SocketAddr::V6(_) => None,
    }
}