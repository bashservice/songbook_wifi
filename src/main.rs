mod json;
mod server;
mod state;
mod ui;

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use server::handle_client;
use state::{log_warn, AppState};

fn main() {
    let port = 8080;
    let state = Arc::new(Mutex::new(AppState::new(port)));

    {
        let guard = state.lock().unwrap();
        println!("=== songbookclear ===");
        println!(
            "Tryb prowadzącego (lokalnie): http://127.0.0.1:{}",
            guard.port
        );
        println!(
            "Adres dla uczestników (Wi-Fi): http://{}:{}",
            guard.local_ip, guard.port
        );
    }

    let listener =
        TcpListener::bind(("0.0.0.0", port)).expect("Nie można uruchomić nasłuchiwania TCP");
    for stream in listener.incoming() {
        match stream {
            Ok(client_stream) => {
                let state_ref = Arc::clone(&state);
                thread::spawn(move || {
                    handle_client(client_stream, state_ref);
                });
            }
            Err(err) => {
                log_warn(&format!("Błąd przyjmowania połączenia TCP: {err}"));
            }
        }
    }
}
