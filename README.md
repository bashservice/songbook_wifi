
# Śpiewnik Wifi 🎵

**Śpiewnik Wifi** to lekka, desktopowa aplikacja napisaną w języku **Rust** przy użyciu frameworka **egui** (eframe). Narzędzie służy do zarządzania bazą utworów/pieśni, edycji ich tekstów, akordów oraz prowadzenia dynamicznych prezentacji (np. wyświetlania tekstów na rzutnik podczas nabożeństw lub spotkań śpiewaczych).

<div align="center">
  <img width="600" alt="image" src="https://github.com/user-attachments/assets/26ac7cec-6714-4927-9f82-b45a8879a9a6" />
</div>

---

## 🚀 Główne funkcjonalności

* **Zarządzanie bazą utworów:** Przechowywanie pieśni w plikach formatu **JSON** w folderze lokalnym `songs/`.
* **Wyszukiwanie w czasie rzeczywistym:** Szybkie filtrowanie listy utworów po nazwie pliku/tytule.
* **Edytor pieśni:**
  * Edycja metadanych (tytuł, autor, tonacja, metrum, tempo, numer hymnu).
  * System części (np. zwrotki `[V1]`, refreny `[C1]`) oraz definiowanie kolejności prezentacji (`presentation`).
  * Podział tekstu na slajdy za pomocą separatora `||`.
* **Podgląd prezentacji na żywo:**
  * Responsywny podgląd slajdu o proporcjach 16:9 z wyśrodkowanym tekstem.
  * Opcja ukrywania/pokazywania linii akordów (linie zaczynające się od kropki `.`).
  * Automatyczna zmiana slajdów w zdefiniowanym przedziale czasowym lub sterowanie ręczne (strzałki `←` / `→` bądź kliknięcie w podgląd)
 
    [demo_songbook_wifi.webm](https://github.com/user-attachments/assets/71c32ed5-404a-43b9-82f2-cbd41af39245)

Lekki serwer prezentacji pieśni i tekstów z akordami w języku **Rust**, udostępniający panel prowadzącego na komputerze lokalnym oraz podgląd na żywo dla uczestników po sieci Wi-Fi / LAN (przez przeglądarkę).


## 🛠 Wymagania wstępne

Aby skompilować i uruchomić projekt, musisz posiadać zainstalowane środowisko języka **Rust** (zalecana wersja z edycją 2024 lub nowszą).

* [Instalacja Rusta (rustup)](https://rustup.rs/)

---

## 📦 Instalacja i Uruchomienie

1. **Sklonuj repozytorium:**
   ```bash
   git clone [https://github.com/twoj-login/piesni-prototyp.git](https://github.com/twoj-login/piesni-prototyp.git)
   cd piesni-prototyp
## 🔮 Planowane funkcjonalności (Roadmap)

### 📱 Udostępnianie tekstów na telefony (Web Viewer)
Planujemy rozbudowę aplikacji o funkcję zdalnego podglądu slajdów na smartfonach bez konieczności instalowania dedykowanej aplikacji – wystarczy zwykła przeglądarka internetowa.

* **Lokalny serwer WWW:** Aplikacja na komputerze uruchomi wbudowany serwer (np. z użyciem biblioteki **Axum**), który będzie informował połączone urządzenia o zmianie utworu lub slajdu w czasie rzeczywistym.
* **Proste połączenie:** 
  * Wystarczy, że komputer i telefony znajdują się w tej samej sieci Wi-Fi (router domowy/kościelny). W przypadku braku routera możliwe będzie skorzystanie z hotspotu.
  * Aplikacja wyświetli czytelny adres URL (np. `http://192.168.1.20:3000`) oraz **kod QR** do szybkiego zeskanowania przez uczestników.
* **Tryb tylko do odczytu:** Pełna kontrola nad przełączaniem slajd
---

## Architektura i schemat komunikacji

Aplikacja uruchamia wbudowany serwer HTTP (`TcpListener` na porcie `8080`) i rozdziela uprawnienia na podstawie adresu IP klienta:
- **Prowadzący (adres lokalny / loopback)**: pełny dostęp do sterowania slajdami, edytora piosenek, motywów, stanu serwera oraz zamykania programu[cite: 18, 20].
- **Uczestnik (inne urządzenie w sieci lokalnej)**: widok tylko do odczytu (`GET /` oraz cykliczne odpytywanie `GET /api/state`)[cite: 18, 20].

```text
+-----------------------------------+                  +-----------------------------------+
|     KOMPUTER PROWADZĄCEGO         |                  |    URZĄDZENIA UCZESTNIKÓW       |
|         (127.0.0.1:8080)          |                  |     (Wi-Fi / LAN, np. :8080)      |
+-----------------------------------+                  +-----------------------------------+
| - Panel sterowania i motywy       |                  | - Przeglądarka (podgląd slajdu) |
| - Edytor piosenek i czasów        |                  | - Brak edycji i sterowania        |
| - Tryb AUTO / pauza / start-stop  |                  | - Cykliczny odczyt stanu          |
+-----------------+-----------------+                  +-----------------+-----------------+
                  |                                                     |
                  | HTTP (GET / POST)                                   | HTTP (GET /api/state)
                  v                                                     v
+-------------------------------------------------------------------------------------------+
|                              SERWER RUST (src/main.rs + src/server.rs)                  |
|                                  Port TCP: 8080 (0.0.0.0:8080)                            |
|                          Współdzielony stan: Arc<Mutex<AppState>>                         |
+---------------------------------------------+---------------------------------------------+
                                              |
                     +------------------------+------------------------+
                     v                                                 v
     +-------------------------------+                 +-------------------------------+
     |      MODUŁ STANU (state.rs)   |                 |     MODUŁ JSON (json.rs)      |
     | - Ładowanie i zapis piosenek  |                 | - Własny parser i serializer  |
     | - Budowanie slajdów i czasów  |                 | - Walidacja pól utworu        |
     | - Motywy i wykrywanie IP      |                 | - Obsługa kolejki i pauz      |
     +---------------+---------------+                 +-------------------------------+
                     |
         +-----------+-----------+
         v                       v
+-----------------+     +-----------------+
|    songs/*.json |     |   themes.json   |
|  (baza pieśni)  |     | (zapis motywów) |
+-----------------+     +-----------------+
         |
         v
+-----------------+
| log/songbookwifi.log |
|  (ostrzeżenia)  |
+-----------------+
```

---

## Przepływ działania (od tekstu do slajdu)

```text
  lyrics ([V1], [C1], linie z ".") + presentation ("V1.12 C1.10 V2")
                                │
                                ▼
                  build_slides() w src/state.rs
        ┌───────────────────────┴───────────────────────┐
        ▼                                               ▼
  Tablica slajdów                               Tablica czasów (slideDurations)
  (linie z "." oznaczane jako akordy)           (czas z tokenu np. .12 lub slide_seconds)
        │                                               │
        └───────────────────────┬───────────────────────┘
                                ▼
                 Przeglądarka (Canvas + licznik AUTO)
     + pauza między utworami: next_song_delay_seconds na ostatnim slajdzie
```

---

## Struktura plików projektu

```text
songbookwifi/
├── Cargo.toml              # Konfiguracja pakietu Rust
├── README.md               # Dokumentacja projektu
├── themes.json             # Zapisane motywy kolorystyczne (tworzone automatycznie)
├── src/
│   ├── main.rs             # Punkt wejścia, nasłuch TCP i wątki klientów[cite: 17]
│   ├── server.rs           # Obsługa żądań HTTP, autoryzacja IP i endpointy API[cite: 18]
│   ├── state.rs            # Stan aplikacji, parsowanie sekcji/czasów, pliki i logi[cite: 19]
│   ├── json.rs             # Parser i generator JSON dla piosenek oraz struktur danych[cite: 16]
│   └── ui.rs               # Interfejs webowy (HTML + CSS + JS z renderowaniem na Canvas)[cite: 20]
├── songs/                  # Katalog z plikami piosenek w formacie *.json[cite: 19]
│   └── *.json
└── log/                    # Katalog logów ostrzegawczych aplikacji[cite: 19]
    └── songbookwifi.log
```

---

## Endpointy HTTP (API)

| Metoda | Ścieżka | Dostęp | Przeznaczenie |
| :--- | :--- | :--- | :--- |
| `GET` | `/` | Wszyscy | Zwraca stronę interfejsu webowego (`HTML_PAGE`)[cite: 18, 20] |
| `GET` | `/api/state` | Wszyscy | Zwraca bieżący stan slajdu, utworu, czasów i motywu[cite: 18, 19] |
| `POST` | `/api/action` | Prowadzący | Przełączanie slajdów (`next`, `prev`) oraz wybór utworu (`song:<idx>`)[cite: 18] |
| `GET` | `/api/songs` | Prowadzący | Przeładowanie i zwrócenie katalogu piosenek z `songs/`[cite: 18, 19] |
| `POST` | `/api/songs/save` | Prowadzący | Zapis nowej lub edytowanej piosenki do pliku JSON[cite: 18, 19] |
| `POST` | `/api/songs/delete` | Prowadzący | Usunięcie pliku piosenki z katalogu `songs/`[cite: 18, 19] |
| `GET` | `/api/themes` | Prowadzący | Pobranie aktywnego motywu i listy zapisanych motywów[cite: 18, 19] |
| `POST` | `/api/theme` | Prowadzący | Podgląd/zmiana kolorów i widoczności akordów na żywo[cite: 18, 19] |
| `POST` | `/api/themes/save` | Prowadzący | Zapisanie motywu do `themes.json`[cite: 18, 19] |
| `POST` | `/api/themes/select` | Prowadzący | Aktywacja zapisanego motywu[cite: 18, 19] |
| `POST` | `/api/server/toggle` | Prowadzący | Wstrzymanie / wznowienie działania prezentacji (`server_running`)[cite: 18, 19] |
| `POST` | `/api/shutdown` | Prowadzący | Bezpieczne zamknięcie procesu serwera[cite: 18] |

---

## Format pliku piosenki (`songs/*.json`)

```json
{
  "title": "Przykładowa pieśń",
  "key": "C-dur",
  "time_sig": "4/4",
  "author": "Autor",
  "tempo": "110",
  "hymn_number": "1",
  "presentation": "V1.10 C1.8 V2",
  "lyrics": "[V1]\n.C G Am F\nPierwsza zwrotka tekstu\n\n[C1]\n.F G C\nRefren pieśni\n\n[V2]\n.C G Am F\nDruga zwrotka tekstu",
  "automatic": true,
  "slide_seconds": 10,
  "next_song_delay_seconds": 3
}
```

- **Sekcje w `lyrics`**: znaczniki w nawiasach kwadratowych, np. `[V1]`, `[C1]`.
- **Linie akordów**: linie zaczynające się od kropki (`.`) są wyróżniane jako akordy i mogą być ukrywane przełącznikiem *Pokaż akordy*[cite: 20].
- **Czasy w `presentation`**: zapis np. `V1.12` ustawia `12` sekund dla tego slajdu; brak kropki używa `slide_seconds`[cite: 19, 20].
- **Pauza między utworami**: `next_song_delay_seconds` określa odliczanie po ostatnim slajdzie utworu przed przejściem dalej w trybie automatycznym[cite: 20].

---

## Uruchomienie

```bash
cargo run
```

Po uruchomieniu terminal wyświetli adres lokalny dla prowadzącego (`http://127.0.0.1:8080`) oraz adres sieciowy dla uczestników w tej samej sieci Wi-Fi (`http://<IP>:8080`)[cite: 17].
