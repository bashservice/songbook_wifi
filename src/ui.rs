pub const HTML_PAGE: &str = r##"<!doctype html>
<html lang="pl">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
<title>songbookwifi</title>
<style>
* { box-sizing: border-box; }
html, body {
  width: 100%;
  height: 100%;
  margin: 0;
  overflow: hidden;
  background: #121417;
  color: #f3f4f6;
  font-family: system-ui, sans-serif;
  -webkit-tap-highlight-color: transparent;
}
body {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  min-height: 100dvh;
}
.bar {
  display: flex;
  flex: 0 0 auto;
  gap: 6px;
  padding: 7px 10px;
  background: #1c2026;
  align-items: center;
  flex-wrap: wrap;
  border-bottom: 1px solid #2b323d;
}
.icon-nav-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  min-width: 40px;
  min-height: 38px;
  padding: 6px 9px;
  border-radius: 8px;
  background: #252c37;
  border: 1px solid #3d4756;
  color: #f3f4f6;
  font-size: 13px;
  font-weight: 600;
  touch-action: manipulation;
}
.icon-nav-btn .nav-label {
  display: none;
}
@media (min-width: 640px) {
  .icon-nav-btn .nav-label {
    display: inline;
  }
}
.icon-nav-btn.active {
  border-color: #60a5fa;
  background: #1e3a5f;
}
.icon-nav-btn.nav-songs {
  background: #0f4c48;
  border-color: #14b8a6;
}
.icon-nav-btn.nav-playlist {
  background: #172d4d;
  border-color: #3b82f6;
}
.icon-nav-btn.nav-display {
  background: #232838;
  border-color: #6366f1;
}
.icon-nav-btn.srv-power-btn {
  margin-left: auto;
  background: #14532d;
  border-color: #22c55e;
  color: #dcfce7;
}
.icon-nav-btn.srv-power-btn.offline {
  background: #450a0a;
  border-color: #ef4444;
  color: #fecaca;
}
.player-bar {
  display: flex;
  flex: 0 0 auto;
  flex-direction: column;
  gap: 6px;
  padding: 8px 10px calc(8px + env(safe-area-inset-bottom, 0px));
  background: #181c22;
  align-items: center;
  justify-content: center;
  border-top: 1px solid #2b323d;
}
.player-status-row {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  min-height: 22px;
  flex-wrap: wrap;
}
.hidden { display: none !important; }
button {
  background: #2b323d;
  color: #fff;
  border: 1px solid #3d4756;
  border-radius: 6px;
  padding: 7px 12px;
  cursor: pointer;
  font-size: 13px;
  touch-action: manipulation;
}
button:hover { background: #374151; }
button:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}
button.active {
  border-color: #60a5fa;
  background: #1e3a5f;
}
button.presenter-toggle {
  background: #2563eb;
  border-color: #3b82f6;
  color: #ffffff;
}
button.presenter-toggle.panel-hidden {
  background: #1f242d;
  border-color: #4b5563;
  color: #9ca3af;
}
button.btn-save {
  background: #16a34a;
  border-color: #22c55e;
  font-weight: 600;
}
button.btn-danger-inline {
  background: #450a0a;
  border-color: #dc2626;
  color: #fecaca;
  font-weight: 600;
}
button.btn-danger-inline:hover:not(:disabled) {
  background: #7f1d1d;
}
button.btn-sm {
  padding: 6px 10px;
  font-size: 12px;
}
button.btn-transport {
  font-weight: 800;
  font-size: 15px;
  line-height: 1;
  min-width: 48px;
  min-height: 40px;
  padding: 8px 13px;
  letter-spacing: 0.04em;
  background: #252c37;
  border-color: #4b5563;
}
button.btn-transport:hover {
  background: #374151;
  border-color: #60a5fa;
}
button.btn-transport.paused {
  background: #78350f;
  border-color: #f59e0b;
  color: #fef3c7;
}
.transport-group {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  flex-wrap: wrap;
  width: 100%;
}
button.btn-srv-stop {
  background: #7f1d1d;
  border-color: #ef4444;
  font-weight: 700;
  min-height: 40px;
  padding: 8px 12px;
  font-size: 12px;
}
button.btn-srv-start {
  background: #14532d;
  border-color: #22c55e;
  font-weight: 700;
  min-height: 40px;
  padding: 8px 12px;
  font-size: 12px;
}
.playlist-chip {
  font-size: 12px;
  color: #93c5fd;
  background: #152238;
  border: 1px solid #2b4c7e;
  padding: 3px 9px;
  border-radius: 999px;
}
.ip-label {
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  font-size: 12px;
  color: #cbd5e1;
  word-break: break-all;
  text-align: center;
}
.auto-badge {
  font-size: 12px;
  padding: 3px 9px;
  border-radius: 999px;
  background: #1e293b;
  border: 1px solid #38bdf8;
  color: #7dd3fc;
  font-weight: 700;
}
.meta-inline {
  font-size: 12px;
  color: #9ca3af;
}
#stage {
  flex: 1 1 auto;
  display: block;
  width: 100%;
  min-height: 0;
  touch-action: pan-y;
}

.qr-modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.72);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 14px;
  overflow-y: auto;
  touch-action: pan-y;
}
.qr-modal {
  background: #1b2028;
  border: 1px solid #3b4656;
  border-radius: 12px;
  padding: 16px 18px;
  max-width: 440px;
  width: 100%;
  max-height: 90dvh;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  align-items: stretch;
  gap: 12px;
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.55);
  touch-action: pan-y;
}
.qr-modal-title {
  margin: 0;
  font-size: 16px;
  font-weight: 700;
  color: #f3f4f6;
  text-align: center;
}
.qr-modal-url {
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  font-size: 13px;
  color: #93c5fd;
  word-break: break-all;
  text-align: center;
}
.qr-canvas-box {
  background: #ffffff;
  padding: 10px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 188px;
  min-width: 188px;
  align-self: center;
}
.qr-status-text {
  font-size: 12px;
  color: #cbd5e1;
  text-align: center;
  min-height: 18px;
}
.display-sheet-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
}
.theme-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  background: #161a20;
  border: 1px solid #2b323d;
  border-radius: 8px;
  padding: 8px 10px;
  color: #e2e8f0;
  font-size: 13px;
}
.theme-item.span-2 {
  grid-column: span 2;
  justify-content: flex-start;
}
.theme-item input[type="color"] {
  width: 32px;
  height: 26px;
  padding: 0;
  border: 1px solid #3d4756;
  border-radius: 5px;
  background: transparent;
  cursor: pointer;
}
.theme-item input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}
.theme-preset-row {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}
.theme-preset-row input {
  flex: 1 1 150px;
}
.modal-actions-row {
  display: flex;
  gap: 8px;
  justify-content: space-between;
  flex-wrap: wrap;
}
.pl-picker-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 280px;
  overflow-y: auto;
  touch-action: pan-y;
}
.pl-picker-item {
  text-align: left;
  background: #232a35;
  border: 1px solid #364152;
  border-radius: 8px;
  padding: 10px 12px;
  display: flex;
  flex-direction: column;
  gap: 3px;
}
.pl-picker-item.selected {
  border-color: #60a5fa;
  background: #1e3a5f;
}
.pl-picker-name {
  font-weight: 700;
  font-size: 14px;
}
.pl-picker-meta {
  font-size: 12px;
  color: #9ca3af;
}

#playlist-editor-view,
#songs-view {
  flex: 1 1 auto;
  display: grid;
  grid-template-columns: 310px minmax(0, 1fr);
  min-height: 0;
  overflow: hidden;
  touch-action: pan-y;
}
.songs-sidebar {
  background: #161a20;
  border-right: 1px solid #2b323d;
  display: flex;
  flex-direction: column;
  padding: 12px;
  gap: 10px;
  min-height: 0;
  touch-action: pan-y;
}
.sidebar-top {
  display: flex;
  gap: 8px;
}
.sidebar-top input {
  flex: 1;
  min-width: 0;
}
.pl-sidebar-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}
#song-list,
#pl-editor-list,
#pl-available-list,
#pl-selected-list {
  flex: 1 1 auto;
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
  display: flex;
  flex-direction: column;
  gap: 6px;
  touch-action: pan-y;
}
#pl-editor-list {
  max-height: 220px;
}
.song-item {
  text-align: left;
  padding: 10px 12px;
  background: #1f242d;
  border: 1px solid #2e3643;
  border-radius: 6px;
}
.song-item.selected {
  border-color: #60a5fa;
  background: #1e3a5f;
}
.song-item-title {
  font-weight: 600;
  font-size: 14px;
}
.song-item-sub {
  font-size: 12px;
  color: #9ca3af;
  margin-top: 2px;
}
.song-editor {
  padding: 14px 18px 28px;
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-height: 0;
  touch-action: pan-y;
}
.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}
.editor-header h2 {
  margin: 0;
  font-size: 17px;
}
.file-badge {
  font-size: 12px;
  color: #9ca3af;
  background: #1f242d;
  padding: 4px 8px;
  border-radius: 4px;
  border: 1px solid #2e3643;
}
.form-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
}
.pl-datetime-row {
  display: flex;
  align-items: flex-end;
  gap: 10px;
  flex-wrap: wrap;
}
.pl-dt-field {
  flex: 0 0 auto;
}
.pl-dt-field input.compact-date {
  width: 148px;
  font-variant-numeric: tabular-nums;
}
.pl-dt-field input.compact-time {
  width: 112px;
  font-variant-numeric: tabular-nums;
}
.input-with-icon {
  position: relative;
  display: flex;
  align-items: center;
}
.input-with-icon input[type="text"] {
  padding-right: 36px;
}
.icon-trigger-btn {
  position: absolute;
  right: 3px;
  padding: 3px 6px;
  background: #252c37;
  border-color: #3d4756;
  font-size: 13px;
  line-height: 1;
}
.hidden-picker {
  position: absolute;
  opacity: 0;
  pointer-events: none;
  width: 1px;
  height: 1px;
}
.pl-lists-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}
.pl-box {
  background: #161a20;
  border: 1px solid #2b323d;
  border-radius: 8px;
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 240px;
  touch-action: pan-y;
}
.pl-box-title {
  font-size: 13px;
  font-weight: 700;
  color: #e2e8f0;
}
.pl-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  background: #1f242d;
  border: 1px solid #2e3643;
  border-radius: 6px;
  padding: 7px 10px;
  font-size: 13px;
}
.pl-row span {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
}
.span-2 { grid-column: span 2; }
.span-3 { grid-column: span 3; }
label.field {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 12px;
  color: #cbd5e1;
}
input[type="text"], input[type="number"], select, textarea {
  width: 100%;
  background: #1c2026;
  color: #f3f4f6;
  border: 1px solid #3d4756;
  border-radius: 6px;
  padding: 8px 10px;
  font-size: 14px;
  font-family: inherit;
  touch-action: manipulation;
}
textarea {
  min-height: 220px;
  resize: vertical;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  line-height: 1.45;
  touch-action: pan-y;
}
input:focus, select:focus, textarea:focus {
  outline: none;
  border-color: #60a5fa;
}
.checkbox-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-top: 18px;
  font-size: 14px;
}
.editor-actions {
  display: flex;
  flex: 0 0 auto;
  gap: 10px;
  align-items: center;
  flex-wrap: wrap;
  padding-top: 4px;
}
.status-msg {
  font-size: 13px;
  min-height: 18px;
}
.status-ok { color: #4ade80; }
.status-err { color: #f87171; }

@media (max-width: 820px) {
  html, body {
    height: auto;
    min-height: 100dvh;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    touch-action: pan-y;
  }
  #playlist-editor-view,
  #songs-view {
    display: flex;
    flex-direction: column;
    overflow: visible;
    flex: 1 0 auto;
  }
  .songs-sidebar {
    border-right: none;
    border-bottom: 1px solid #2b323d;
    flex: 0 0 auto;
  }
  #pl-editor-list,
  #song-list {
    max-height: 165px;
  }
  .song-editor {
    overflow-y: visible;
    padding: 12px 12px 32px;
    flex: 0 0 auto;
  }
  .form-grid,
  .pl-lists-grid {
    grid-template-columns: 1fr;
  }
  .span-2, .span-3 { grid-column: span 1; }
}
@media (max-width: 600px) {
  .bar { padding: 6px 8px; gap: 5px; }
  .player-bar { padding: 7px 8px calc(7px + env(safe-area-inset-bottom, 0px)); }
}
</style>
</head>
<body>
<div class="bar" id="topbar">
  <button type="button" id="btn-presenter" class="icon-nav-btn presenter-toggle hidden" title="Przełącz panel sterowania" onclick="togglePresenter()">
    <svg id="icon-bust-on" width="17" height="17" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
      <circle cx="12" cy="8" r="4"/>
      <path d="M4 20c0-4 3.6-7 8-7s8 3 8 7H4z"/>
    </svg>
    <svg id="icon-bust-off" class="hidden" width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
      <circle cx="12" cy="8" r="4"/>
      <path d="M4 20c0-4 3.6-7 8-7s8 3 8 7"/>
    </svg>
    <span class="nav-label">Panel</span>
  </button>

  <button type="button" id="btn-songs-view" class="icon-nav-btn nav-songs hidden" title="Piosenki" onclick="toggleSongsView()">
    <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
      <path d="M9 18V5l12-2v13"/>
      <circle cx="6" cy="18" r="3"/>
      <circle cx="18" cy="16" r="3"/>
    </svg>
    <span id="btn-songs-label" class="nav-label">Piosenki</span>
  </button>

  <button type="button" id="btn-pl-select" class="icon-nav-btn nav-playlist hidden" title="Playlisty" onclick="openPlaylistSelectModal()">
    <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
      <line x1="8" y1="6" x2="21" y2="6"/>
      <line x1="8" y1="12" x2="21" y2="12"/>
      <line x1="8" y1="18" x2="21" y2="18"/>
      <circle cx="4" cy="6" r="1.5" fill="currentColor"/>
      <circle cx="4" cy="12" r="1.5" fill="currentColor"/>
      <circle cx="4" cy="18" r="1.5" fill="currentColor"/>
    </svg>
    <span class="nav-label">Playlisty</span>
  </button>

  <button type="button" id="btn-display-modal" class="icon-nav-btn nav-display hidden" title="Prezentacja i motyw" onclick="openDisplayModal()">
    <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
      <rect x="7" y="2" width="10" height="20" rx="2"/>
      <line x1="11" y1="18" x2="13" y2="18"/>
    </svg>
    <span class="nav-label">Prezentacja</span>
  </button>

  <button type="button" id="btn-qr" class="icon-nav-btn hidden" title="Kod QR" onclick="openQrModal()">
    <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
      <rect x="3" y="3" width="7" height="7"/>
      <rect x="14" y="3" width="7" height="7"/>
      <rect x="3" y="14" width="7" height="7"/>
      <rect x="14" y="14" width="3" height="3"/>
      <line x1="21" y1="14" x2="21" y2="21"/>
      <line x1="14" y1="21" x2="21" y2="21"/>
    </svg>
    <span class="nav-label">QR</span>
  </button>

  <button type="button" id="btn-srv-power" class="icon-nav-btn srv-power-btn hidden" title="Zamknij serwer" onclick="shutdownProgram()">
    <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.3" aria-hidden="true">
      <path d="M12 3v9"/>
      <path d="M6.3 6.3a8 8 0 1 0 11.4 0"/>
    </svg>
    <span class="nav-label">Serwer</span>
  </button>
</div>

<canvas id="stage"></canvas>

<div id="player-bar" class="player-bar hidden">
  <div class="player-status-row">
    <span id="auto-indicator" class="auto-badge hidden"></span>
    <span id="active-playlist-label" class="playlist-chip hidden"></span>
    <span class="meta-inline" id="meta">Łączenie...</span>
  </div>
  <div class="transport-group" aria-label="Sterowanie prezentacją">
    <button type="button" id="btn-prev" class="btn-transport" title="Wstecz" onclick="manualStep('prev')">|◄</button>
    <button type="button" id="btn-pause" class="btn-transport" title="Pauza / Wznów automat i listę" onclick="togglePlaybackPause()">❚❚</button>
    <button type="button" id="btn-next" class="btn-transport" title="Dalej" onclick="manualStep('next')">►</button>
    <button type="button" id="btn-srv-toggle" class="btn-srv-stop" onclick="toggleServerState()">● [Stop]</button>
  </div>
</div>

<div id="playlist-editor-view" class="hidden">
  <aside class="songs-sidebar">
    <div class="pl-box-title">Wczytaj playlistę</div>
    <div class="pl-sidebar-actions">
      <button type="button" class="btn-save btn-sm" onclick="resetPlaylistForm()">Nowa playlista</button>
      <button type="button" id="btn-pl-delete" class="btn-danger-inline btn-sm" onclick="deleteSelectedPlaylist()" disabled>Skasuj playlistę</button>
    </div>
    <div id="pl-editor-list"></div>
    <div class="song-item-sub">Kliknij nazwę, aby podświetlić i wczytać. Zmiana nazwy przy zapisie utworzy nowy plik JSON.</div>
  </aside>

  <section class="song-editor">
    <div class="editor-header">
      <h2 id="pl-editor-heading">Tworzenie playlisty</h2>
      <button type="button" onclick="closePlaylistEditor()">← Powrót do prezentacji</button>
    </div>

    <div class="form-grid">
      <label class="field span-3">
        <span>Nazwa playlisty / uroczystości *</span>
        <input type="text" id="pl-name" placeholder="np. Niedziela - Msza poranna" />
      </label>
    </div>

    <div class="pl-datetime-row">
      <label class="field pl-dt-field">
        <span>Data</span>
        <div class="input-with-icon">
          <input type="text" id="pl-date" class="compact-date" placeholder="RRRR-MM-DD" />
          <button type="button" class="icon-trigger-btn" title="Wybierz datę" onclick="openDatePicker()">📅</button>
          <input type="date" id="pl-date-picker" class="hidden-picker" onchange="onDatePickerChange(this.value)" />
        </div>
      </label>
      <label class="field pl-dt-field">
        <span>Godzina</span>
        <div class="input-with-icon">
          <input type="text" id="pl-time" class="compact-time" placeholder="GG:MM" />
          <button type="button" class="icon-trigger-btn" title="Wybierz godzinę" onclick="openTimePicker()">🕒</button>
          <input type="time" id="pl-time-picker" class="hidden-picker" onchange="onTimePickerChange(this.value)" />
        </div>
      </label>
    </div>

    <div class="pl-lists-grid">
      <div class="pl-box">
        <div class="pl-box-title">Dostępne utwory (kliknij Dodaj)</div>
        <div id="pl-available-list"></div>
      </div>
      <div class="pl-box">
        <div class="pl-box-title">Utwory w playliście</div>
        <div id="pl-selected-list"></div>
      </div>
    </div>

    <div id="pl-editor-status" class="status-msg"></div>

    <div class="editor-actions">
      <button type="button" class="btn-save" onclick="savePlaylist()">Zapisz</button>
      <button type="button" onclick="closePlaylistEditor()">Anuluj</button>
    </div>
  </section>
</div>

<div id="songs-view" class="hidden">
  <aside class="songs-sidebar">
    <div class="sidebar-top">
      <input type="text" id="song-search" placeholder="Szukaj po tytule, autorze, nr..." oninput="renderSongCatalog()" />
      <button class="btn-save" onclick="startNewSong()">+ Nowa piosenka</button>
    </div>
    <div id="song-list"></div>
  </aside>

  <section class="song-editor">
    <div class="editor-header">
      <h2 id="editor-heading">Nowa piosenka</h2>
      <span class="file-badge" id="editor-filename">Nowy plik w katalogu songs/</span>
    </div>

    <div class="form-grid">
      <label class="field span-2">
        <span>Tytuł (title) *</span>
        <input type="text" id="f-title" placeholder="np. Abba Ojcze - Ty wyzwoliłeś nas Panie" />
      </label>
      <label class="field">
        <span>Numer pieśni (hymn_number)</span>
        <input type="text" id="f-hymn-number" placeholder="np. 142" />
      </label>

      <label class="field span-2">
        <span>Autor (author)</span>
        <input type="text" id="f-author" placeholder="np. J. Sykulski Jan Góra" />
      </label>
      <label class="field">
        <span>Tonacja (key)</span>
        <input type="text" id="f-key" placeholder="np. C-dur" />
      </label>

      <label class="field">
        <span>Metrum (time_sig)</span>
        <input type="text" id="f-time-sig" placeholder="np. 4/4" />
      </label>
      <label class="field">
        <span>Tempo (tempo)</span>
        <input type="text" id="f-tempo" placeholder="np. 110" />
      </label>
      <label class="field">
        <span>Kolejność slajdów (presentation) — np. V1.12 C1.10 V2</span>
        <input type="text" id="f-presentation" placeholder="np. V1.12 C1.10 V2 V3 V4" />
      </label>

      <label class="field">
        <span>Domyślny czas slajdu (slide_seconds)</span>
        <input type="number" id="f-slide-seconds" min="0" value="10" />
      </label>
      <label class="field">
        <span>Pauza przed kolejnym utworem (next_song_delay_seconds)</span>
        <input type="number" id="f-next-song-delay" min="0" value="3" />
      </label>
      <div class="checkbox-row">
        <input type="checkbox" id="f-automatic" />
        <label for="f-automatic">Automatyczne przełączanie slajdów (automatic)</label>
      </div>

      <label class="field span-3">
        <span>Tekst i sekcje (lyrics) * — sekcje np. [V1], [C1], linie akordów zaczynaj od kropki (.)</span>
        <textarea id="f-lyrics" placeholder="[V1]&#10;.C       F         G C&#10; Ty wyzwoliłeś nas Panie..."></textarea>
      </label>
    </div>

    <div id="editor-status" class="status-msg"></div>

    <div class="editor-actions">
      <button class="btn-save" onclick="saveCurrentSong()">Zapisz</button>
      <button onclick="cancelEditing()">Anuluj</button>
      <button id="btn-delete-song" class="btn-danger-inline" onclick="deleteCurrentSong()">Usuń</button>
    </div>
  </section>
</div>

<div id="display-modal-backdrop" class="qr-modal-backdrop hidden" onclick="onDisplayBackdropClick(event)">
  <div class="qr-modal" role="dialog" aria-modal="true" aria-labelledby="display-modal-title">
    <h3 id="display-modal-title" class="qr-modal-title">Ustawienia prezentacji i motywu</h3>
    <div id="srv-ip" class="ip-label">http://192.168.43.14:8080</div>
    <div class="display-sheet-grid">
      <label class="theme-item" title="Kolor tła slajdu">
        <span>Tło</span>
        <input type="color" id="col-bg" value="#121417" onchange="onThemeChange()" />
      </label>
      <label class="theme-item" title="Kolor tekstu slajdu">
        <span>Tekst</span>
        <input type="color" id="col-text" value="#f9fafb" onchange="onThemeChange()" />
      </label>
      <label class="theme-item" title="Osobny kolor linii akordów (zaczynających się od .)">
        <span>Akordy</span>
        <input type="color" id="col-chord" value="#fbbf24" onchange="onThemeChange()" />
      </label>
      <label class="theme-item" title="Pokazuj lub ukryj linie akordów">
        <span>Akordy widoczne</span>
        <input type="checkbox" id="chk-show-chords" checked onchange="onThemeChange()" />
      </label>
    </div>
    <div class="theme-preset-row">
      <input type="text" id="theme-name" placeholder="Nazwa motywu..." />
      <button type="button" class="btn-sm btn-save" onclick="saveThemePreset()">Zapisz motyw</button>
    </div>
    <select id="theme-select" onchange="selectThemePreset(this.value)">
      <option value="">Wybierz motyw...</option>
    </select>
    <div class="modal-actions-row">
      <button type="button" class="btn-sm" onclick="copyWifiAddress()">Kopiuj adres Wi-Fi</button>
      <button type="button" class="btn-sm" onclick="closeDisplayModal()">Zamknij</button>
    </div>
  </div>
</div>

<div id="qr-modal-backdrop" class="qr-modal-backdrop hidden" onclick="onQrBackdropClick(event)">
  <div class="qr-modal" role="dialog" aria-modal="true" aria-labelledby="qr-modal-title">
    <h3 id="qr-modal-title" class="qr-modal-title">Kod QR dla uczestników</h3>
    <div id="qr-modal-url" class="qr-modal-url"></div>
    <div id="qr-canvas-box" class="qr-canvas-box"></div>
    <div id="qr-status-text" class="qr-status-text"></div>
    <button type="button" class="btn-sm" onclick="closeQrModal()">Zamknij okno</button>
  </div>
</div>

<div id="pl-select-modal-backdrop" class="qr-modal-backdrop hidden" onclick="onPlaylistSelectBackdropClick(event)">
  <div class="qr-modal" role="dialog" aria-modal="true" aria-labelledby="pl-select-modal-title">
    <h3 id="pl-select-modal-title" class="qr-modal-title">Playlisty</h3>
    <input type="text" id="pl-search-input" placeholder="Szukaj po nazwie lub dacie / godzinie..." oninput="renderPlaylistPicker()" />
    <div id="pl-picker-list" class="pl-picker-list"></div>
    <div class="modal-actions-row">
      <button type="button" class="btn-sm btn-save" onclick="openPlaylistEditor()">Utwórz / edytuj playlistę</button>
      <button type="button" class="btn-sm" onclick="closePlaylistSelectModal()">Zamknij</button>
    </div>
  </div>
</div>

<script>
const canvas = document.getElementById('stage');
const ctx = canvas.getContext('2d');
let currentState = null;
let presenterModeOpen = true;
let songsViewOpen = false;
let playlistEditorOpen = false;
let catalogSongs = [];
let allPlaylists = [];
let activePlaylistFilename = '';
let selectedEditorPlaylistFilename = '';
let draftPlaylistSongs = [];
let editingFilename = null;
let resizeObserver = null;
let themesLoadedOnce = false;
let serverShutdownDone = false;

let autoRemaining = 0;
let autoSignature = '';
let playbackPaused = false;
let songTransitionRemaining = 0;
let songTransitionTotal = 0;

let touchStartX = null;
let touchStartY = null;

const CHORD_PREFIX = '\x01';

const DEFAULT_THEME = {
  name: 'Ciemny klasyczny',
  bg: '#121417',
  text: '#f9fafb',
  chord: '#fbbf24',
  showChords: true
};

let activeTheme = { ...DEFAULT_THEME };

function isAnySubViewOpen() {
  return songsViewOpen || playlistEditorOpen;
}

function openDatePicker() {
  const picker = document.getElementById('pl-date-picker');
  const txt = document.getElementById('pl-date');
  if (!picker) return;
  if (txt && /^\d{4}-\d{2}-\d{2}$/.test(txt.value.trim())) {
    picker.value = txt.value.trim();
  }
  if (typeof picker.showPicker === 'function') {
    picker.showPicker();
  } else {
    picker.click();
  }
}

function onDatePickerChange(val) {
  const txt = document.getElementById('pl-date');
  if (txt && val) {
    txt.value = val;
  }
}

function openTimePicker() {
  const picker = document.getElementById('pl-time-picker');
  const txt = document.getElementById('pl-time');
  if (!picker) return;
  if (txt && /^\d{2}:\d{2}$/.test(txt.value.trim())) {
    picker.value = txt.value.trim();
  }
  if (typeof picker.showPicker === 'function') {
    picker.showPicker();
  } else {
    picker.click();
  }
}

function onTimePickerChange(val) {
  const txt = document.getElementById('pl-time');
  if (txt && val) {
    txt.value = val;
  }
}

function openDisplayModal() {
  if (!currentState || !currentState.canPresent) return;
  closeQrModal();
  closePlaylistSelectModal();
  applyThemeFromState(currentState.theme);
  loadThemesList();
  document.getElementById('display-modal-backdrop').classList.remove('hidden');
}

function closeDisplayModal() {
  document.getElementById('display-modal-backdrop').classList.add('hidden');
}

function onDisplayBackdropClick(event) {
  if (event.target && event.target.id === 'display-modal-backdrop') {
    closeDisplayModal();
  }
}

async function openPlaylistSelectModal() {
  if (!currentState || !currentState.canPresent) return;
  closeQrModal();
  closeDisplayModal();
  await fetchPlaylistsData();
  const inp = document.getElementById('pl-search-input');
  if (inp) inp.value = '';
  renderPlaylistPicker();
  document.getElementById('pl-select-modal-backdrop').classList.remove('hidden');
  if (inp) inp.focus();
}

function closePlaylistSelectModal() {
  document.getElementById('pl-select-modal-backdrop').classList.add('hidden');
}

function onPlaylistSelectBackdropClick(event) {
  if (event.target && event.target.id === 'pl-select-modal-backdrop') {
    closePlaylistSelectModal();
  }
}

async function fetchPlaylistsData() {
  try {
    const res = await fetch('/api/playlists', { cache: 'no-store' });
    if (!res.ok) return;
    const data = await res.json();
    allPlaylists = data.playlists || [];
    activePlaylistFilename = data.activeFilename || '';
  } catch (_) {}
}

function renderPlaylistPicker() {
  const listEl = document.getElementById('pl-picker-list');
  const query = (document.getElementById('pl-search-input').value || '').toLowerCase().trim();
  const tokens = query ? query.split(/\s+/) : [];
  listEl.replaceChildren();

  const filtered = allPlaylists.filter(pl => {
    if (tokens.length === 0) return true;
    const hay = `${pl.name || ''} ${pl.schedule || ''} ${pl.date || ''} ${pl.time || ''}`.toLowerCase();
    return tokens.every(tok => hay.includes(tok));
  });

  if (filtered.length === 0) {
    const empty = document.createElement('div');
    empty.className = 'pl-picker-meta';
    empty.textContent = 'Brak pasujących playlist.';
    listEl.appendChild(empty);
    return;
  }

  filtered.forEach(pl => {
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'pl-picker-item' + (pl.filename === activePlaylistFilename ? ' selected' : '');

    const title = document.createElement('div');
    title.className = 'pl-picker-name';
    title.textContent = pl.name || pl.filename;

    const meta = document.createElement('div');
    meta.className = 'pl-picker-meta';
    const count = Array.isArray(pl.songs) ? pl.songs.length : 0;
    meta.textContent = [pl.schedule, `${count} utworów`].filter(Boolean).join(' • ');

    btn.appendChild(title);
    btn.appendChild(meta);
    btn.onclick = () => selectPlaylistByFilename(pl.filename);
    listEl.appendChild(btn);
  });
}

async function selectPlaylistByFilename(filename) {
  try {
    const res = await fetch('/api/playlists/select', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: JSON.stringify({ filename })
    });
    const data = await res.json();
    if (res.ok && data.ok) {
      allPlaylists = data.playlists ? (data.playlists.playlists || allPlaylists) : allPlaylists;
      activePlaylistFilename = filename;
      if (data.state) {
        currentState = data.state;
        renderToolbar(currentState, true);
      }
      closePlaylistSelectModal();
      draw();
    }
  } catch (_) {}
}

async function openPlaylistEditor() {
  if (!currentState || !currentState.canPresent) return;
  songsViewOpen = false;
  playlistEditorOpen = true;
  closePlaylistSelectModal();
  closeDisplayModal();
  closeQrModal();
  updateUiVisibility();
  await Promise.all([loadSongsCatalog(), fetchPlaylistsData()]);
  if (!selectedEditorPlaylistFilename && allPlaylists.length > 0) {
    const initial = allPlaylists.find(p => p.filename === activePlaylistFilename) || allPlaylists[0];
    if (initial) {
      onSelectPlaylistToEdit(initial.filename);
    } else {
      resetPlaylistForm();
    }
  } else {
    renderPlaylistEditorList();
    updateDeletePlaylistButton();
    renderPlaylistSongLists();
  }
  window.scrollTo({ top: 0, behavior: 'auto' });
}

function closePlaylistEditor() {
  playlistEditorOpen = false;
  updateUiVisibility();
  resize();
}

function updateDeletePlaylistButton() {
  const delBtn = document.getElementById('btn-pl-delete');
  if (!delBtn) return;
  const exists = Boolean(
    selectedEditorPlaylistFilename &&
    allPlaylists.some(p => p.filename === selectedEditorPlaylistFilename)
  );
  delBtn.disabled = !exists;
}

function renderPlaylistEditorList() {
  const listEl = document.getElementById('pl-editor-list');
  if (!listEl) return;
  listEl.replaceChildren();

  if (allPlaylists.length === 0) {
    const empty = document.createElement('div');
    empty.className = 'song-item-sub';
    empty.style.padding = '8px';
    empty.textContent = 'Brak zapisanych playlist.';
    listEl.appendChild(empty);
    return;
  }

  allPlaylists.forEach(pl => {
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'song-item' + (pl.filename === selectedEditorPlaylistFilename ? ' selected' : '');

    const titleEl = document.createElement('div');
    titleEl.className = 'song-item-title';
    titleEl.textContent = pl.name || pl.filename;

    const subEl = document.createElement('div');
    subEl.className = 'song-item-sub';
    const count = Array.isArray(pl.songs) ? pl.songs.length : 0;
    subEl.textContent = [pl.schedule, `${count} utworów`].filter(Boolean).join(' • ');

    btn.appendChild(titleEl);
    btn.appendChild(subEl);
    btn.onclick = () => onSelectPlaylistToEdit(pl.filename);
    listEl.appendChild(btn);
  });
}

function onSelectPlaylistToEdit(filename) {
  if (!filename) {
    resetPlaylistForm();
    return;
  }
  const found = allPlaylists.find(p => p.filename === filename);
  if (!found) return;
  selectedEditorPlaylistFilename = found.filename;
  document.getElementById('pl-editor-heading').textContent = 'Edycja playlisty';
  document.getElementById('pl-name').value = found.name || '';
  document.getElementById('pl-date').value = found.date || '';
  document.getElementById('pl-time').value = found.time || '';
  draftPlaylistSongs = Array.isArray(found.songs) ? [...found.songs] : [];
  setPlaylistStatus('', false);
  renderPlaylistEditorList();
  updateDeletePlaylistButton();
  renderPlaylistSongLists();
}

function resetPlaylistForm() {
  selectedEditorPlaylistFilename = '';
  const now = new Date();
  document.getElementById('pl-editor-heading').textContent = 'Tworzenie playlisty';
  document.getElementById('pl-name').value = '';
  document.getElementById('pl-date').value = now.toISOString().slice(0, 10);
  document.getElementById('pl-time').value = now.toTimeString().slice(0, 5);
  draftPlaylistSongs = [];
  setPlaylistStatus('', false);
  renderPlaylistEditorList();
  updateDeletePlaylistButton();
  renderPlaylistSongLists();
}

async function deleteSelectedPlaylist() {
  if (!selectedEditorPlaylistFilename) return;
  const target = allPlaylists.find(p => p.filename === selectedEditorPlaylistFilename);
  const label = (target && target.name) || selectedEditorPlaylistFilename;
  if (!confirm(`Czy na pewno skasować playlistę "${label}"?`)) return;

  try {
    const res = await fetch('/api/playlists/delete', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: JSON.stringify({ filename: selectedEditorPlaylistFilename })
    });
    const rawText = await res.text();
    let data = null;
    try {
      data = JSON.parse(rawText);
    } catch (_) {
      setPlaylistStatus(rawText || 'Błąd odpowiedzi serwera podczas usuwania playlisty.', true);
      return;
    }

    if (!res.ok || !data || !data.ok) {
      setPlaylistStatus((data && data.error) || 'Nie udało się skasować playlisty.', true);
      return;
    }

    allPlaylists = data.playlists ? (data.playlists.playlists || []) : [];
    activePlaylistFilename = data.playlists ? (data.playlists.activeFilename || '') : '';
    if (data.state) {
      currentState = data.state;
      renderToolbar(currentState, true);
    }

    if (allPlaylists.length > 0) {
      onSelectPlaylistToEdit(allPlaylists[0].filename);
    } else {
      resetPlaylistForm();
    }
    setPlaylistStatus(`Skasowano playlistę "${label}".`, false);
  } catch (_) {
    setPlaylistStatus('Brak połączenia z serwerem podczas usuwania playlisty.', true);
  }
}

function renderPlaylistSongLists() {
  const availEl = document.getElementById('pl-available-list');
  const selEl = document.getElementById('pl-selected-list');
  if (!availEl || !selEl) return;

  availEl.replaceChildren();
  catalogSongs.forEach(song => {
    const row = document.createElement('div');
    row.className = 'pl-row';

    const label = document.createElement('span');
    label.textContent = song.title || song.filename;

    const addBtn = document.createElement('button');
    addBtn.type = 'button';
    addBtn.className = 'btn-sm btn-save';
    addBtn.textContent = 'Dodaj';
    addBtn.onclick = () => {
      draftPlaylistSongs.push(song.filename);
      renderPlaylistSongLists();
    };

    row.appendChild(label);
    row.appendChild(addBtn);
    availEl.appendChild(row);
  });

  selEl.replaceChildren();
  if (draftPlaylistSongs.length === 0) {
    const empty = document.createElement('div');
    empty.className = 'song-item-sub';
    empty.textContent = 'Brak utworów w playliście.';
    selEl.appendChild(empty);
    return;
  }

  draftPlaylistSongs.forEach((fname, idx) => {
    const songObj = catalogSongs.find(s => s.filename === fname);
    const row = document.createElement('div');
    row.className = 'pl-row';

    const label = document.createElement('span');
    label.textContent = `${idx + 1}. ${(songObj && songObj.title) || fname}`;

    const rmBtn = document.createElement('button');
    rmBtn.type = 'button';
    rmBtn.className = 'btn-sm';
    rmBtn.textContent = 'Usuń';
    rmBtn.onclick = () => {
      draftPlaylistSongs.splice(idx, 1);
      renderPlaylistSongLists();
    };

    row.appendChild(label);
    row.appendChild(rmBtn);
    selEl.appendChild(row);
  });
}

async function savePlaylist() {
  const payload = {
    name: (document.getElementById('pl-name').value || '').trim(),
    date: (document.getElementById('pl-date').value || '').trim(),
    time: (document.getElementById('pl-time').value || '').trim(),
    songs: draftPlaylistSongs
  };

  try {
    const res = await fetch('/api/playlists/save', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: JSON.stringify(payload)
    });
    const rawText = await res.text();
    let data = null;
    try {
      data = JSON.parse(rawText);
    } catch (_) {
      setPlaylistStatus(rawText || 'Błąd odpowiedzi serwera podczas zapisu playlisty.', true);
      return;
    }

    if (!res.ok || !data || !data.ok) {
      setPlaylistStatus((data && data.error) || 'Nie udało się zapisać playlisty.', true);
      return;
    }
    if (data.playlists) {
      allPlaylists = data.playlists.playlists || allPlaylists;
      activePlaylistFilename = data.playlists.activeFilename || data.filename;
    }
    if (data.state) {
      currentState = data.state;
      renderToolbar(currentState, true);
    }
    selectedEditorPlaylistFilename = data.filename || selectedEditorPlaylistFilename;
    renderPlaylistEditorList();
    updateDeletePlaylistButton();
    setPlaylistStatus(`Zapisano playlistę jako playlists/${data.filename}`, false);
  } catch (_) {
    setPlaylistStatus('Brak połączenia z serwerem (sprawdź czy działa cargo run).', true);
  }
}

function setPlaylistStatus(message, isError) {
  const el = document.getElementById('pl-editor-status');
  if (!el) return;
  el.textContent = message;
  el.className = 'status-msg ' + (message ? (isError ? 'status-err' : 'status-ok') : '');
}

function openQrModal() {
  if (!currentState || !currentState.canPresent) return;
  closeDisplayModal();
  closePlaylistSelectModal();
  const backdrop = document.getElementById('qr-modal-backdrop');
  const urlEl = document.getElementById('qr-modal-url');
  const boxEl = document.getElementById('qr-canvas-box');
  const statusEl = document.getElementById('qr-status-text');
  if (!backdrop || !urlEl || !boxEl || !statusEl) return;

  const fallbackUrl = `http://${currentState.ip || '127.0.0.1'}:${currentState.port || 8080}`;
  urlEl.textContent = fallbackUrl;
  boxEl.innerHTML = '';
  statusEl.textContent = 'Generowanie kodu QR...';
  backdrop.classList.remove('hidden');

  generateQrBelowInModal(fallbackUrl);
}

async function generateQrBelowInModal(fallbackUrl) {
  const urlEl = document.getElementById('qr-modal-url');
  const boxEl = document.getElementById('qr-canvas-box');
  const statusEl = document.getElementById('qr-status-text');
  if (!boxEl || !statusEl) return;

  try {
    const res = await fetch('/api/qr', { cache: 'no-store' });
    const data = await res.json();
    if (urlEl && data.url) {
      urlEl.textContent = data.url;
    }
    if (!res.ok || !data.ok || !data.svg) {
      boxEl.innerHTML = '';
      statusEl.textContent = data.error || `Nie udało się wygenerować kodu QR dla ${fallbackUrl}.`;
      return;
    }
    boxEl.innerHTML = data.svg;
    statusEl.textContent = 'Zeskanuj aparatem w telefonie, aby otworzyć podgląd.';
  } catch (_) {
    boxEl.innerHTML = '';
    statusEl.textContent = 'Błąd połączenia podczas generowania kodu QR.';
  }
}

function closeQrModal() {
  const backdrop = document.getElementById('qr-modal-backdrop');
  const boxEl = document.getElementById('qr-canvas-box');
  const statusEl = document.getElementById('qr-status-text');
  if (backdrop) backdrop.classList.add('hidden');
  if (boxEl) boxEl.innerHTML = '';
  if (statusEl) statusEl.textContent = '';
}

function onQrBackdropClick(event) {
  if (event.target && event.target.id === 'qr-modal-backdrop') {
    closeQrModal();
  }
}

function applyThemeFromState(themeObj) {
  if (!themeObj) return;
  activeTheme = {
    name: themeObj.name || DEFAULT_THEME.name,
    bg: themeObj.bg || DEFAULT_THEME.bg,
    text: themeObj.text || DEFAULT_THEME.text,
    chord: themeObj.chord || DEFAULT_THEME.chord,
    showChords: typeof themeObj.showChords === 'boolean' ? themeObj.showChords : true
  };
  const bgEl = document.getElementById('col-bg');
  const textEl = document.getElementById('col-text');
  const chordEl = document.getElementById('col-chord');
  const chordsChk = document.getElementById('chk-show-chords');
  if (bgEl && document.activeElement !== bgEl) bgEl.value = activeTheme.bg;
  if (textEl && document.activeElement !== textEl) textEl.value = activeTheme.text;
  if (chordEl && document.activeElement !== chordEl) chordEl.value = activeTheme.chord;
  if (chordsChk && document.activeElement !== chordsChk) chordsChk.checked = activeTheme.showChords;
}

async function loadThemesList() {
  if (!currentState || !currentState.canPresent) return;
  try {
    const res = await fetch('/api/themes', { cache: 'no-store' });
    if (!res.ok) return;
    const data = await res.json();
    const sel = document.getElementById('theme-select');
    if (!sel) return;
    sel.replaceChildren();
    const placeholder = document.createElement('option');
    placeholder.value = '';
    placeholder.textContent = 'Wybierz motyw...';
    sel.appendChild(placeholder);

    (data.themes || []).forEach(t => {
      const opt = document.createElement('option');
      opt.value = t.name;
      opt.textContent = t.name;
      if (data.active && data.active.name === t.name) {
        opt.selected = true;
      }
      sel.appendChild(opt);
    });
  } catch (_) {}
}

async function onThemeChange() {
  if (!currentState || !currentState.canPresent) return;
  const bgEl = document.getElementById('col-bg');
  const textEl = document.getElementById('col-text');
  const chordEl = document.getElementById('col-chord');
  const chordsChk = document.getElementById('chk-show-chords');

  const payload = {
    bg: bgEl ? bgEl.value : activeTheme.bg,
    text: textEl ? textEl.value : activeTheme.text,
    chord: chordEl ? chordEl.value : activeTheme.chord,
    showChords: chordsChk ? Boolean(chordsChk.checked) : true
  };

  try {
    const res = await fetch('/api/theme', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: JSON.stringify(payload)
    });
    if (res.ok) {
      currentState = await res.json();
      applyThemeFromState(currentState.theme);
      draw();
    }
  } catch (_) {}
}

async function saveThemePreset() {
  if (!currentState || !currentState.canPresent) return;
  const nameInput = document.getElementById('theme-name');
  const name = (nameInput ? nameInput.value : '').trim();
  if (!name) {
    document.getElementById('meta').textContent = 'Wpisz nazwę motywu przed zapisem.';
    return;
  }
  await onThemeChange();
  try {
    const res = await fetch('/api/themes/save', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: JSON.stringify({ name })
    });
    const data = await res.json();
    if (res.ok && data.ok) {
      if (data.state) {
        currentState = data.state;
        applyThemeFromState(currentState.theme);
      }
      await loadThemesList();
      document.getElementById('meta').textContent = `Zapisano motyw: ${name}`;
      draw();
    }
  } catch (_) {}
}

async function selectThemePreset(name) {
  if (!currentState || !currentState.canPresent || !name) return;
  try {
    const res = await fetch('/api/themes/select', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: JSON.stringify({ name })
    });
    const data = await res.json();
    if (res.ok && data.ok) {
      if (data.state) {
        currentState = data.state;
        applyThemeFromState(currentState.theme);
      }
      const nameInput = document.getElementById('theme-name');
      if (nameInput) nameInput.value = name;
      await loadThemesList();
      draw();
    }
  } catch (_) {}
}

function isServerActive(state, connected = true) {
  if (serverShutdownDone) return false;
  if (!connected || !state) return false;
  if (typeof state.serverRunning === 'boolean') return state.serverRunning;
  return true;
}

function updateServerPowerIcon(connected = true) {
  const pwrBtn = document.getElementById('btn-srv-power');
  if (!pwrBtn) return;
  const online = !serverShutdownDone && connected && Boolean(currentState);
  pwrBtn.classList.toggle('offline', !online);
  pwrBtn.title = online ? 'Serwer działa — kliknij, aby zamknąć' : 'Serwer zatrzymany / zamknięty';
}

function clearSongTransition() {
  songTransitionRemaining = 0;
  songTransitionTotal = 0;
}

function updatePauseButton() {
  const btn = document.getElementById('btn-pause');
  if (!btn) return;
  btn.classList.toggle('paused', playbackPaused);
  btn.textContent = playbackPaused ? '►❚❚' : '❚❚';
  btn.title = playbackPaused ? 'Wznów odtwarzanie / listę' : 'Pauza odtwarzania / listy';
}

function togglePlaybackPause() {
  playbackPaused = !playbackPaused;
  updatePauseButton();
  if (playbackPaused) {
    updateAutoBadge();
  } else if (currentState) {
    if (songTransitionRemaining > 0 || autoRemaining > 0) {
      updateAutoBadge();
    } else {
      resetAutoTimer(currentState, true);
    }
  }
}

function manualStep(direction) {
  playbackPaused = false;
  updatePauseButton();
  clearSongTransition();
  autoRemaining = 0;
  autoSignature = '';
  sendAction(direction, true);
}

function updateBottomBar(state, connected = true) {
  const playerBar = document.getElementById('player-bar');
  const ipEl = document.getElementById('srv-ip');
  const toggleBtn = document.getElementById('btn-srv-toggle');
  updateServerPowerIcon(connected);
  if (!playerBar || !toggleBtn) return;

  const show = Boolean(state && state.canPresent && presenterModeOpen && !isAnySubViewOpen());
  playerBar.classList.toggle('hidden', !show);
  if (!show) return;

  const ip = state.ip || '127.0.0.1';
  const port = state.port || 8080;
  if (ipEl) ipEl.textContent = `http://${ip}:${port}`;

  const active = isServerActive(state, connected);
  if (!connected) {
    toggleBtn.textContent = '● [Start/Stop]';
    toggleBtn.className = 'btn-srv-start';
  } else if (active) {
    toggleBtn.textContent = '● [Stop]';
    toggleBtn.className = 'btn-srv-stop';
  } else {
    toggleBtn.textContent = '● [Start]';
    toggleBtn.className = 'btn-srv-start';
  }
  updatePauseButton();
}

async function copyWifiAddress() {
  if (!currentState) return;
  const url = `http://${currentState.ip}:${currentState.port}`;
  try {
    await navigator.clipboard.writeText(url);
    document.getElementById('meta').textContent = `Skopiowano: ${url}`;
  } catch (_) {
    document.getElementById('meta').textContent = url;
  }
}

async function toggleServerState() {
  if (!currentState || !currentState.canPresent) return;
  try {
    const res = await fetch('/api/server/toggle', { method: 'POST' });
    if (res.ok) {
      currentState = await res.json();
      renderToolbar(currentState);
      draw();
    }
  } catch (_) {
    updateBottomBar(currentState, false);
  }
}

async function shutdownProgram() {
  if (!currentState || !currentState.canPresent) return;
  if (!confirm('Czy na pewno odłączyć wszystkich klientów i całkowicie zamknąć program?')) return;

  try {
    await fetch('/api/shutdown', { method: 'POST' });
  } catch (_) {}

  serverShutdownDone = true;
  closeQrModal();
  closePlaylistSelectModal();
  closeDisplayModal();
  currentState = null;
  updateBottomBar(null, false);
  document.getElementById('meta').textContent = 'Program został zamknięty.';
  const playerBar = document.getElementById('player-bar');
  if (playerBar) playerBar.classList.add('hidden');
}

function getActiveSlideSeconds(state) {
  if (!state) return 0;
  if (Array.isArray(state.slideDurations) && state.slideDurations.length > state.slideIndex) {
    const val = Number(state.slideDurations[state.slideIndex]);
    if (!Number.isNaN(val) && val > 0) return val;
  }
  return Math.max(0, Number(state.slideSeconds || 0));
}

function resetAutoTimer(state, forceReset = false) {
  const slideSec = getActiveSlideSeconds(state);
  if (
    !state ||
    !state.canPresent ||
    !isServerActive(state, true) ||
    !state.automatic ||
    slideSec <= 0 ||
    state.slideCount <= 0
  ) {
    autoRemaining = 0;
    autoSignature = '';
    clearSongTransition();
    updateAutoBadge();
    return;
  }
  const sig = `${state.songIndex}:${state.slideIndex}:${slideSec}`;
  if (forceReset || sig !== autoSignature || (autoRemaining <= 0 && songTransitionRemaining <= 0)) {
    autoSignature = sig;
    autoRemaining = slideSec;
    clearSongTransition();
  }
  updateAutoBadge();
}

function updateAutoBadge() {
  const badge = document.getElementById('auto-indicator');
  if (!badge) return;
  const slideSec = getActiveSlideSeconds(currentState);
  if (
    currentState &&
    currentState.canPresent &&
    isServerActive(currentState, true) &&
    currentState.automatic &&
    slideSec > 0 &&
    currentState.slideCount > 0
  ) {
    badge.classList.remove('hidden');
    if (songTransitionRemaining > 0) {
      badge.textContent = playbackPaused
        ? `PAUZA — KOLEJNY UTWÓR (${songTransitionRemaining}s / ${songTransitionTotal}s)`
        : `KOLEJNY UTWÓR ZA: ${songTransitionRemaining}s / ${songTransitionTotal}s`;
    } else {
      badge.textContent = playbackPaused
        ? `PAUZA (${Math.max(0, autoRemaining)}s / ${slideSec}s)`
        : `AUTO: ${Math.max(0, autoRemaining)}s / ${slideSec}s`;
    }
  } else {
    badge.classList.add('hidden');
  }
}

setInterval(() => {
  if (
    !currentState ||
    !currentState.canPresent ||
    !isServerActive(currentState, true) ||
    playbackPaused ||
    isAnySubViewOpen() ||
    !presenterModeOpen
  ) return;
  const slideSec = getActiveSlideSeconds(currentState);
  if (!currentState.automatic || slideSec <= 0 || currentState.slideCount <= 0) {
    return;
  }

  if (songTransitionRemaining > 0) {
    if (songTransitionRemaining > 1) {
      songTransitionRemaining -= 1;
      updateAutoBadge();
    } else {
      clearSongTransition();
      updateAutoBadge();
      sendAction('next', true);
    }
    return;
  }

  if (autoRemaining > 1) {
    autoRemaining -= 1;
    updateAutoBadge();
  } else {
    const isLastSlide = currentState.slideIndex + 1 >= currentState.slideCount;
    const delaySec = Number(currentState.nextSongDelaySeconds || 0);
    if (isLastSlide && delaySec > 0) {
      songTransitionTotal = delaySec;
      songTransitionRemaining = delaySec;
      autoRemaining = 0;
      updateAutoBadge();
    } else {
      autoRemaining = slideSec;
      updateAutoBadge();
      sendAction('next', true);
    }
  }
}, 1000);

function togglePresenter() {
  presenterModeOpen = !presenterModeOpen;
  updateUiVisibility();
  resize();
}

async function toggleSongsView() {
  if (!currentState || !currentState.canPresent) return;
  playlistEditorOpen = false;
  closeDisplayModal();
  closePlaylistSelectModal();
  closeQrModal();
  songsViewOpen = !songsViewOpen;
  updateUiVisibility();
  if (songsViewOpen) {
    await loadSongsCatalog();
    window.scrollTo({ top: 0, behavior: 'auto' });
  } else {
    resize();
  }
}

function updateUiVisibility() {
  const btnPresenter = document.getElementById('btn-presenter');
  const iconBustOn = document.getElementById('icon-bust-on');
  const iconBustOff = document.getElementById('icon-bust-off');
  const btnSongsView = document.getElementById('btn-songs-view');
  const btnSongsLabel = document.getElementById('btn-songs-label');
  const btnPlSelect = document.getElementById('btn-pl-select');
  const btnDisplay = document.getElementById('btn-display-modal');
  const btnQr = document.getElementById('btn-qr');
  const btnSrvPower = document.getElementById('btn-srv-power');
  const songsView = document.getElementById('songs-view');
  const plEditorView = document.getElementById('playlist-editor-view');

  const isPresenter = Boolean(currentState && currentState.canPresent);
  if (!isPresenter) {
    songsViewOpen = false;
    playlistEditorOpen = false;
    closeQrModal();
    closePlaylistSelectModal();
    closeDisplayModal();
  }

  btnSongsView.classList.toggle('hidden', !isPresenter);
  if (btnSongsLabel) {
    btnSongsLabel.textContent = songsViewOpen ? 'Prezentacja' : 'Piosenki';
  }
  btnSongsView.title = songsViewOpen ? 'Wróć do prezentacji' : 'Piosenki';
  btnSongsView.classList.toggle('active', songsViewOpen);

  canvas.classList.toggle('hidden', isAnySubViewOpen());
  songsView.classList.toggle('hidden', !songsViewOpen);
  plEditorView.classList.toggle('hidden', !playlistEditorOpen);

  if (!currentState) {
    btnPresenter.classList.add('hidden');
    btnPlSelect.classList.add('hidden');
    btnDisplay.classList.add('hidden');
    btnQr.classList.add('hidden');
    btnSrvPower.classList.toggle('hidden', !serverShutdownDone);
    updateBottomBar(null, false);
    return;
  }

  const showNav = isPresenter && !isAnySubViewOpen();
  btnPresenter.classList.toggle('hidden', !showNav);
  btnPresenter.classList.toggle('panel-hidden', !presenterModeOpen);
  if (iconBustOn) iconBustOn.classList.toggle('hidden', !presenterModeOpen);
  if (iconBustOff) iconBustOff.classList.toggle('hidden', presenterModeOpen);
  btnPresenter.title = presenterModeOpen ? 'Ukryj belkę sterowania' : 'Pokaż belkę sterowania';

  btnPlSelect.classList.toggle('hidden', !isPresenter);
  btnDisplay.classList.toggle('hidden', !isPresenter);
  btnQr.classList.toggle('hidden', !isPresenter);
  btnSrvPower.classList.toggle('hidden', !isPresenter);

  if (isPresenter && !themesLoadedOnce) {
    themesLoadedOnce = true;
    loadThemesList();
  }
  updateBottomBar(currentState, true);
  updateAutoBadge();
}

function resize() {
  if (isAnySubViewOpen()) return;
  const rect = canvas.getBoundingClientRect();
  if (!rect.width || !rect.height) return;
  const dpr = window.devicePixelRatio || 1;
  canvas.width = Math.round(rect.width * dpr);
  canvas.height = Math.round(rect.height * dpr);
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  draw();
}

function wrapParagraph(text, maxWidth, isChord) {
  const result = [];
  const words = text.trim().split(/\s+/);
  let line = '';
  for (const word of words) {
    if (!word) continue;
    const candidate = line ? `${line} ${word}` : word;
    if (line && ctx.measureText(candidate).width > maxWidth) {
      result.push({ text: line, isChord });
      line = word;
    } else {
      line = candidate;
    }
  }
  result.push({ text: line, isChord });
  return result.length ? result : [{ text: '', isChord }];
}

function prepareSlideLines(rawSlideText, maxWidth) {
  const lines = [];
  for (const rawLine of (rawSlideText || '').split('\n')) {
    if (rawLine.startsWith(CHORD_PREFIX)) {
      if (!activeTheme.showChords) continue;
      const chordText = rawLine.slice(CHORD_PREFIX.length);
      lines.push(...wrapParagraph(chordText, maxWidth, true));
    } else {
      lines.push(...wrapParagraph(rawLine, maxWidth, false));
    }
  }
  return lines.length ? lines : [{ text: '', isChord: false }];
}

function draw() {
  if (!currentState || isAnySubViewOpen()) return;
  const rect = canvas.getBoundingClientRect();
  const w = rect.width;
  const h = rect.height;
  if (!w || !h) return;

  ctx.clearRect(0, 0, w, h);
  ctx.fillStyle = activeTheme.bg;
  ctx.fillRect(0, 0, w, h);

  const compact = w < 600 || h < 450;
  const sidePadding = Math.max(16, Math.min(48, w * 0.06));
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';

  const titleSize = Math.max(16, Math.min(compact ? 20 : 28, w * 0.045));
  ctx.fillStyle = '#60a5fa';
  ctx.font = `bold ${titleSize}px system-ui, sans-serif`;
  const titleLines = wrapParagraph(currentState.songTitle || '', w - sidePadding * 2, false);
  const titleLineHeight = titleSize * 1.25;
  const titleTop = Math.max(12, h * 0.06);
  titleLines.forEach((line, index) => {
    ctx.fillText(line.text, w / 2, titleTop + index * titleLineHeight, w - sidePadding * 2);
  });

  const footerSize = Math.max(12, Math.min(16, w * 0.035));
  const footerY = h - Math.max(18, footerSize * 1.5);
  ctx.fillStyle = '#9ca3af';
  ctx.font = `${footerSize}px system-ui, sans-serif`;
  const slideTotal = Math.max(0, currentState.slideCount || 0);
  const slideCurr = slideTotal > 0 ? currentState.slideIndex + 1 : 0;
  ctx.fillText(`Slajd ${slideCurr}/${slideTotal}`, w / 2, footerY);

  const slideTop = titleTop + titleLines.length * titleLineHeight + 12;
  const slideBottom = footerY - footerSize * 1.5;
  const availableHeight = Math.max(40, slideBottom - slideTop);

  let fontSize = Math.max(16, Math.min(compact ? 32 : 44, w * 0.075, h * 0.11));
  let lines = [];
  for (; fontSize >= 16; fontSize -= 1) {
    ctx.font = `600 ${fontSize}px system-ui, sans-serif`;
    lines = prepareSlideLines(currentState.slideText || '', w - sidePadding * 2);
    const lineHeight = fontSize * 1.3;
    if (lines.length * lineHeight <= availableHeight) break;
  }

  const lineHeight = fontSize * 1.3;
  const textHeight = lines.length * lineHeight;
  const startY = slideTop + Math.max(0, (availableHeight - textHeight) / 2) + lineHeight / 2;
  lines.forEach((line, index) => {
    ctx.font = line.isChord
      ? `700 ${Math.max(14, Math.round(fontSize * 0.85))}px ui-monospace, SFMono-Regular, Menlo, Consolas, monospace`
      : `600 ${fontSize}px system-ui, sans-serif`;
    ctx.fillStyle = line.isChord ? activeTheme.chord : activeTheme.text;
    ctx.fillText(line.text, w / 2, startY + index * lineHeight, w - sidePadding * 2);
  });
}

function renderToolbar(state, forceTimerReset = false) {
  applyThemeFromState(state.theme);
  updateUiVisibility();
  updateBottomBar(state, true);
  resetAutoTimer(state, forceTimerReset);

  const plLabel = document.getElementById('active-playlist-label');
  if (plLabel) {
    if (state.playlistName) {
      plLabel.textContent = state.playlistSchedule
        ? `${state.playlistName} (${state.playlistSchedule})`
        : state.playlistName;
      plLabel.classList.remove('hidden');
    } else {
      plLabel.textContent = '';
      plLabel.classList.add('hidden');
    }
  }

  const meta = document.getElementById('meta');
  const active = isServerActive(state, true);
  if (state.canPresent) {
    meta.textContent = active ? 'Prezenter aktywny' : 'Prezenter zatrzymany';
  } else {
    meta.textContent = active ? 'Podgląd uczestnika' : 'Prezentacja wstrzymana';
  }
  resize();
}

async function refreshState() {
  if (serverShutdownDone) return;
  try {
    const response = await fetch('/api/state', { cache: 'no-store' });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    currentState = await response.json();
    renderToolbar(currentState, false);
    draw();
  } catch (_) {
    updateBottomBar(currentState, false);
    document.getElementById('meta').textContent = 'Brak połączenia...';
  }
}

async function sendAction(command, forceTimerReset = false) {
  if (!currentState || !currentState.canPresent) return;
  try {
    const response = await fetch('/api/action', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain; charset=utf-8' },
      body: command
    });
    if (!response.ok) return;
    currentState = await response.json();
    renderToolbar(currentState, forceTimerReset);
    draw();
  } catch (_) {
    updateBottomBar(currentState, false);
  }
}

async function loadSongsCatalog() {
  if (!currentState || !currentState.canPresent) return;
  try {
    const res = await fetch('/api/songs', { cache: 'no-store' });
    if (!res.ok) throw new Error('Brak dostępu do listy piosenek');
    catalogSongs = await res.json();
    renderSongCatalog();
    if (playlistEditorOpen) {
      renderPlaylistSongLists();
    }
    if (editingFilename) {
      const found = catalogSongs.find(s => s.filename === editingFilename);
      if (found) fillEditorForm(found);
      else startNewSong();
    } else if (catalogSongs.length > 0 && !document.getElementById('f-title').value) {
      fillEditorForm(catalogSongs[0]);
    }
  } catch (err) {
    setEditorStatus(err.message || 'Błąd wczytywania katalogu.', true);
  }
}

function renderSongCatalog() {
  const listDiv = document.getElementById('song-list');
  const query = (document.getElementById('song-search').value || '').toLowerCase().trim();
  listDiv.replaceChildren();

  const filtered = catalogSongs.filter(s =>
    s.title.toLowerCase().includes(query) ||
    s.author.toLowerCase().includes(query) ||
    s.hymn_number.toLowerCase().includes(query) ||
    s.filename.toLowerCase().includes(query)
  );

  if (filtered.length === 0) {
    const empty = document.createElement('div');
    empty.className = 'song-item-sub';
    empty.style.padding = '8px';
    empty.textContent = 'Brak piosenek spełniających kryteria.';
    listDiv.appendChild(empty);
    return;
  }

  filtered.forEach(song => {
    const item = document.createElement('button');
    item.type = 'button';
    item.className = 'song-item' + (song.filename === editingFilename ? ' selected' : '');

    const titleEl = document.createElement('div');
    titleEl.className = 'song-item-title';
    titleEl.textContent = song.hymn_number ? `${song.hymn_number}. ${song.title}` : song.title;

    const subEl = document.createElement('div');
    subEl.className = 'song-item-sub';
    const parts = [song.author, song.key, song.filename].filter(Boolean);
    subEl.textContent = parts.join(' • ');

    item.appendChild(titleEl);
    item.appendChild(subEl);
    item.onclick = () => fillEditorForm(song);
    listDiv.appendChild(item);
  });
}

function fillEditorForm(song) {
  editingFilename = song.filename;
  document.getElementById('editor-heading').textContent = 'Edycja piosenki';
  document.getElementById('editor-filename').textContent = `songs/${song.filename}`;
  document.getElementById('f-title').value = song.title || '';
  document.getElementById('f-key').value = song.key || '';
  document.getElementById('f-time-sig').value = song.time_sig || '';
  document.getElementById('f-author').value = song.author || '';
  document.getElementById('f-tempo').value = song.tempo || '';
  document.getElementById('f-hymn-number').value = song.hymn_number || '';
  document.getElementById('f-presentation').value = song.presentation || '';
  document.getElementById('f-lyrics').value = song.lyrics || '';
  document.getElementById('f-automatic').checked = Boolean(song.automatic);
  document.getElementById('f-slide-seconds').value = song.slide_seconds ?? 10;
  document.getElementById('f-next-song-delay').value = song.next_song_delay_seconds ?? 3;
  document.getElementById('btn-delete-song').classList.remove('hidden');
  setEditorStatus('', false);
  renderSongCatalog();
}

function startNewSong() {
  editingFilename = null;
  document.getElementById('editor-heading').textContent = 'Nowa piosenka';
  document.getElementById('editor-filename').textContent = 'Nowy plik w katalogu songs/';
  document.getElementById('f-title').value = '';
  document.getElementById('f-key').value = '';
  document.getElementById('f-time-sig').value = '';
  document.getElementById('f-author').value = '';
  document.getElementById('f-tempo').value = '';
  document.getElementById('f-hymn-number').value = '';
  document.getElementById('f-presentation').value = '';
  document.getElementById('f-lyrics').value = '';
  document.getElementById('f-automatic').checked = false;
  document.getElementById('f-slide-seconds').value = 10;
  document.getElementById('f-next-song-delay').value = 3;
  document.getElementById('btn-delete-song').classList.add('hidden');
  setEditorStatus('', false);
  renderSongCatalog();
}

function cancelEditing() {
  if (editingFilename) {
    const existing = catalogSongs.find(s => s.filename === editingFilename);
    if (existing) {
      fillEditorForm(existing);
      setEditorStatus('Przywrócono zapisaną wersję.', false);
      return;
    }
  }
  startNewSong();
}

async function saveCurrentSong() {
  const delayRaw = document.getElementById('f-next-song-delay').value;
  const parsedDelay = parseInt(delayRaw, 10);
  const nextSongDelay = Number.isNaN(parsedDelay) ? 3 : Math.max(0, parsedDelay);

  const payload = {
    filename: editingFilename || '',
    title: document.getElementById('f-title').value.trim(),
    key: document.getElementById('f-key').value.trim(),
    time_sig: document.getElementById('f-time-sig').value.trim(),
    author: document.getElementById('f-author').value.trim(),
    tempo: document.getElementById('f-tempo').value.trim(),
    hymn_number: document.getElementById('f-hymn-number').value.trim(),
    presentation: document.getElementById('f-presentation').value.trim(),
    lyrics: document.getElementById('f-lyrics').value,
    automatic: document.getElementById('f-automatic').checked,
    slide_seconds: Math.max(0, parseInt(document.getElementById('f-slide-seconds').value || '0', 10) || 0),
    next_song_delay_seconds: nextSongDelay
  };

  try {
    const res = await fetch('/api/songs/save', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: JSON.stringify(payload)
    });
    const data = await res.json();
    if (!res.ok || !data.ok) {
      setEditorStatus(data.error || 'Błąd podczas zapisywania piosenki.', true);
      return;
    }
    catalogSongs = data.songs || [];
    if (data.state) {
      currentState = data.state;
      renderToolbar(currentState, true);
    }
    const saved = catalogSongs.find(s => s.filename === data.filename);
    if (saved) fillEditorForm(saved);
    setEditorStatus(`Zapisano pomyślnie jako songs/${data.filename}`, false);
  } catch (_) {
    setEditorStatus('Błąd połączenia z serwerem podczas zapisu.', true);
  }
}

async function deleteCurrentSong() {
  if (!editingFilename) return;
  if (!confirm(`Czy na pewno usunąć plik songs/${editingFilename}?`)) return;

  try {
    const res = await fetch('/api/songs/delete', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: JSON.stringify({ filename: editingFilename })
    });
    const data = await res.json();
    if (!res.ok || !data.ok) {
      setEditorStatus(data.error || 'Nie udało się usunąć piosenki.', true);
      return;
    }
    catalogSongs = data.songs || [];
    if (data.state) {
      currentState = data.state;
      renderToolbar(currentState, true);
    }
    startNewSong();
    setEditorStatus('Piosenka została usunięta.', false);
  } catch (_) {
    setEditorStatus('Błąd połączenia podczas usuwania.', true);
  }
}

function setEditorStatus(message, isError) {
  const el = document.getElementById('editor-status');
  el.textContent = message;
  el.className = 'status-msg ' + (message ? (isError ? 'status-err' : 'status-ok') : '');
}

canvas.addEventListener('touchstart', event => {
  if (!currentState || !currentState.canPresent || isAnySubViewOpen()) return;
  if (event.touches.length !== 1) return;
  touchStartX = event.touches[0].clientX;
  touchStartY = event.touches[0].clientY;
}, { passive: true });

canvas.addEventListener('touchend', event => {
  if (touchStartX === null || touchStartY === null) return;
  if (!currentState || !currentState.canPresent || isAnySubViewOpen()) {
    touchStartX = null;
    touchStartY = null;
    return;
  }
  const changed = event.changedTouches[0];
  if (!changed) {
    touchStartX = null;
    touchStartY = null;
    return;
  }
  const dx = changed.clientX - touchStartX;
  const dy = changed.clientY - touchStartY;
  touchStartX = null;
  touchStartY = null;

  if (Math.abs(dx) > 48 && Math.abs(dx) > Math.abs(dy) * 1.4) {
    if (dx < 0) {
      manualStep('next');
    } else {
      manualStep('prev');
    }
  }
}, { passive: true });

window.addEventListener('keydown', event => {
  if (event.key === 'Escape') {
    closeQrModal();
    closePlaylistSelectModal();
    closeDisplayModal();
    return;
  }
  if (!currentState || !currentState.canPresent || isAnySubViewOpen()) return;
  if (event.key === 'ArrowRight' || event.key === ' ') {
    event.preventDefault();
    manualStep('next');
  } else if (event.key === 'ArrowLeft') {
    event.preventDefault();
    manualStep('prev');
  }
});

window.addEventListener('resize', resize);
if ('ResizeObserver' in window) {
  resizeObserver = new ResizeObserver(resize);
  resizeObserver.observe(canvas);
}

refreshState();
setInterval(refreshState, 1000);
</script>
</body>
</html>
"##;