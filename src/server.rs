use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::json::{parse, parse_song, Value};
use crate::state::{escape_json, Action, AppState};
use crate::ui::HTML_PAGE;

/// Edycja piosenek dostępna wyłącznie dla prowadzącego (lokalnie).
const ALLOW_REMOTE_SONGS_EDIT: bool = false;

fn is_local_client(peer_ip: IpAddr, server_local_ip: &str) -> bool {
    if peer_ip.is_loopback() {
        return true;
    }
    peer_ip.to_string() == server_local_ip
}

pub fn handle_client(mut stream: TcpStream, state: Arc<Mutex<AppState>>) {
    let peer_ip = match stream.peer_addr() {
        Ok(addr) => addr.ip(),
        Err(_) => return,
    };

    let is_local = {
        let guard = state.lock().unwrap();
        is_local_client(peer_ip, &guard.local_ip)
    };
    let can_edit_songs = is_local || ALLOW_REMOTE_SONGS_EDIT;

    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() || request_line.is_empty() {
        return;
    }

    let mut headers = BTreeMap::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err()
            || line == "\r\n"
            || line == "\n"
            || line.is_empty()
        {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }

    let content_length: usize = headers
        .get("content-length")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let mut body = vec![0u8; content_length];
    if content_length > 0 && reader.read_exact(&mut body).is_err() {
        return;
    }

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = parts.first().copied().unwrap_or("");
    let raw_path = parts.get(1).copied().unwrap_or("/");
    let path = raw_path.split('?').next().unwrap_or("/");

    match (method, path) {
        ("GET", "/") => {
            write_response(&mut stream, "200 OK", "text/html; charset=utf-8", HTML_PAGE);
        }
        ("GET", "/api/state") => {
            let json = {
                let guard = state.lock().unwrap();
                guard.to_json(is_local)
            };
            write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &json);
        }
        ("GET", "/api/themes") => {
            if !is_local {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Zarządzanie motywami dostępne wyłącznie lokalnie.",
                );
                return;
            }
            let json = {
                let guard = state.lock().unwrap();
                guard.themes_json()
            };
            write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &json);
        }
        ("POST", "/api/theme") => {
            if !is_local {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Zmiana motywu dostępna wyłącznie lokalnie.",
                );
                return;
            }
            let text = String::from_utf8_lossy(&body);
            let Ok(Value::Object(map)) = parse(&text) else {
                write_response(
                    &mut stream,
                    "400 Bad Request",
                    "application/json; charset=utf-8",
                    r#"{"ok":false,"error":"Nieprawidłowy JSON motywu"}"#,
                );
                return;
            };
            let bg = match map.get("bg") {
                Some(Value::String(s)) => Some(s.clone()),
                _ => None,
            };
            let text_col = match map.get("text") {
                Some(Value::String(s)) => Some(s.clone()),
                _ => None,
            };
            let chord = match map.get("chord") {
                Some(Value::String(s)) => Some(s.clone()),
                _ => None,
            };
            let show_chords = match map.get("showChords") {
                Some(Value::Bool(b)) => Some(*b),
                _ => None,
            };

            let json = {
                let mut guard = state.lock().unwrap();
                guard.set_theme_live(bg, text_col, chord, show_chords);
                guard.to_json(is_local)
            };
            write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &json);
        }
        ("POST", "/api/themes/save") => {
            if !is_local {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Zapis motywów dostępny wyłącznie lokalnie.",
                );
                return;
            }
            let text = String::from_utf8_lossy(&body);
            let Some(name) = extract_string_field(&text, "name") else {
                write_response(
                    &mut stream,
                    "400 Bad Request",
                    "application/json; charset=utf-8",
                    r#"{"ok":false,"error":"Brak nazwy motywu"}"#,
                );
                return;
            };
            let result = {
                let mut guard = state.lock().unwrap();
                guard
                    .save_theme_preset(&name)
                    .map(|_| guard.to_json(is_local))
            };
            match result {
                Ok(state_json) => {
                    let resp = format!(r#"{{"ok":true,"state":{}}}"#, state_json);
                    write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &resp);
                }
                Err(err) => {
                    let resp = format!(r#"{{"ok":false,"error":"{}"}}"#, escape_json(&err));
                    write_response(&mut stream, "400 Bad Request", "application/json; charset=utf-8", &resp);
                }
            }
        }
        ("POST", "/api/themes/select") => {
            if !is_local {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Wybór motywu dostępny wyłącznie lokalnie.",
                );
                return;
            }
            let text = String::from_utf8_lossy(&body);
            let Some(name) = extract_string_field(&text, "name") else {
                write_response(
                    &mut stream,
                    "400 Bad Request",
                    "application/json; charset=utf-8",
                    r#"{"ok":false,"error":"Brak nazwy motywu"}"#,
                );
                return;
            };
            let result = {
                let mut guard = state.lock().unwrap();
                guard
                    .select_theme_preset(&name)
                    .map(|_| guard.to_json(is_local))
            };
            match result {
                Ok(state_json) => {
                    let resp = format!(r#"{{"ok":true,"state":{}}}"#, state_json);
                    write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &resp);
                }
                Err(err) => {
                    let resp = format!(r#"{{"ok":false,"error":"{}"}}"#, escape_json(&err));
                    write_response(&mut stream, "400 Bad Request", "application/json; charset=utf-8", &resp);
                }
            }
        }
        ("POST", "/api/server/toggle") => {
            if !is_local {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Przełączanie stanu serwera dostępne wyłącznie lokalnie.",
                );
                return;
            }
            let json = {
                let mut guard = state.lock().unwrap();
                guard.server_running = !guard.server_running;
                guard.to_json(is_local)
            };
            write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &json);
        }
        ("POST", "/api/shutdown") => {
            if !is_local {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Wyłączenie programu dostępne wyłącznie lokalnie na urządzeniu prowadzącego.",
                );
                return;
            }
            {
                let mut guard = state.lock().unwrap();
                guard.server_running = false;
            }
            write_response(
                &mut stream,
                "200 OK",
                "application/json; charset=utf-8",
                r#"{"ok":true,"shutdown":true}"#,
            );
            println!("Zamykanie programu i odłączanie wszystkich klientów...");
            thread::spawn(|| {
                thread::sleep(Duration::from_millis(150));
                std::process::exit(0);
            });
        }
        ("POST", "/api/action") => {
            if !is_local {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Sterowanie dostępne wyłącznie lokalnie na urządzeniu prowadzącego.",
                );
                return;
            }
            let cmd = String::from_utf8_lossy(&body).trim().to_string();
            let result = {
                let mut guard = state.lock().unwrap();
                if !guard.server_running {
                    Err("Serwer jest wstrzymany (STOP). Uruchom go przyciskiem [Start], aby sterować prezentacją.")
                } else {
                    if cmd == "next" {
                        guard.apply(Action::NextSlide);
                    } else if cmd == "prev" {
                        guard.apply(Action::PrevSlide);
                    } else if let Some(rest) = cmd.strip_prefix("song:") {
                        if let Ok(idx) = rest.parse::<usize>() {
                            guard.apply(Action::SelectSong(idx));
                        }
                    }
                    Ok(guard.to_json(is_local))
                }
            };

            match result {
                Ok(json) => {
                    write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &json);
                }
                Err(msg) => {
                    write_response(
                        &mut stream,
                        "503 Service Unavailable",
                        "text/plain; charset=utf-8",
                        msg,
                    );
                }
            }
        }
        ("GET", "/api/songs") => {
            if !can_edit_songs {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Brak dostępu",
                );
                return;
            }
            let json = {
                let mut guard = state.lock().unwrap();
                guard.reload_songs(None);
                guard.songs_catalog_json()
            };
            write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &json);
        }
        ("POST", "/api/songs/save") => {
            if !can_edit_songs {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Brak dostępu",
                );
                return;
            }
            let text = String::from_utf8_lossy(&body);
            let filename = extract_string_field(&text, "filename");
            let song = match parse_song(&text) {
                Ok(s) => s,
                Err(err) => {
                    let resp = format!(r#"{{"ok":false,"error":"{}"}}"#, escape_json(&err));
                    write_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json; charset=utf-8",
                        &resp,
                    );
                    return;
                }
            };

            let result = {
                let mut guard = state.lock().unwrap();
                guard.save_song(filename.as_deref(), song).map(|saved_file| {
                    (
                        saved_file,
                        guard.songs_catalog_json(),
                        guard.to_json(is_local),
                    )
                })
            };

            match result {
                Ok((saved_file, catalog_json, state_json)) => {
                    let resp = format!(
                        r#"{{"ok":true,"filename":"{}","songs":{},"state":{}}}"#,
                        escape_json(&saved_file),
                        catalog_json,
                        state_json
                    );
                    write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &resp);
                }
                Err(err) => {
                    let resp = format!(r#"{{"ok":false,"error":"{}"}}"#, escape_json(&err));
                    write_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json; charset=utf-8",
                        &resp,
                    );
                }
            }
        }
        ("POST", "/api/songs/delete") => {
            if !can_edit_songs {
                write_response(
                    &mut stream,
                    "403 Forbidden",
                    "text/plain; charset=utf-8",
                    "Brak dostępu",
                );
                return;
            }
            let text = String::from_utf8_lossy(&body);
            let Some(filename) = extract_string_field(&text, "filename") else {
                write_response(
                    &mut stream,
                    "400 Bad Request",
                    "application/json; charset=utf-8",
                    r#"{"ok":false,"error":"Brak nazwy pliku do usunięcia"}"#,
                );
                return;
            };

            let result = {
                let mut guard = state.lock().unwrap();
                guard
                    .delete_song(&filename)
                    .map(|_| (guard.songs_catalog_json(), guard.to_json(is_local)))
            };

            match result {
                Ok((catalog_json, state_json)) => {
                    let resp = format!(
                        r#"{{"ok":true,"songs":{},"state":{}}}"#,
                        catalog_json, state_json
                    );
                    write_response(&mut stream, "200 OK", "application/json; charset=utf-8", &resp);
                }
                Err(err) => {
                    let resp = format!(r#"{{"ok":false,"error":"{}"}}"#, escape_json(&err));
                    write_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json; charset=utf-8",
                        &resp,
                    );
                }
            }
        }
        _ => {
            write_response(&mut stream, "404 Not Found", "text/plain; charset=utf-8", "Not found");
        }
    }
}

fn extract_string_field(text: &str, key: &str) -> Option<String> {
    let Ok(Value::Object(map)) = parse(text) else {
        return None;
    };
    match map.get(key) {
        Some(Value::String(s)) if !s.trim().is_empty() => Some(s.trim().to_string()),
        _ => None,
    }
}

fn write_response(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n{}",
        status,
        content_type,
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}
