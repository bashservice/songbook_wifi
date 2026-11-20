pub const HTML_PAGE: &str = r##"<!doctype html>
<html lang="pl">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
<title>songbookclear</title>
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
  gap: 8px;
  padding: 8px 12px;
  background: #1c2026;
  align-items: center;
  flex-wrap: wrap;
  border-bottom: 1px solid #2b323d;
}
.theme-bar {
  display: flex;
  flex: 0 0 auto;
  gap: 8px;
  padding: 6px 12px;
  background: #191d24;
  align-items: center;
  flex-wrap: wrap;
  border-bottom: 1px solid #2b323d;
  font-size: 12px;
}
.theme-item {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  color: #cbd5e1;
}
.theme-item input[type="color"] {
  width: 24px;
  height: 22px;
  padding: 0;
  border: 1px solid #3d4756;
  border-radius: 4px;
  background: transparent;
  cursor: pointer;
}
.theme-item input[type="checkbox"] {
  cursor: pointer;
}
.theme-preset-input {
  width: 120px;
  background: #14181e;
  color: #f3f4f6;
  border: 1px solid #3d4756;
  border-radius: 5px;
  padding: 4px 8px;
  font-size: 12px;
}
.theme-preset-select {
  background: #14181e;
  color: #f3f4f6;
  border: 1px solid #3d4756;
  border-radius: 5px;
  padding: 4px 8px;
  font-size: 12px;
}
.player-bar {
  display: flex;
  flex: 0 0 auto;
  gap: 8px;
  padding: 6px 12px;
  background: #181c22;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  border-top: 1px solid #2b323d;
}
.player-left {
  display: flex;
  align-items: center;
  gap: 8px;
  min-height: 28px;
}
.bottom-bar {
  display: flex;
  flex: 0 0 auto;
  gap: 10px;
  padding: 7px 12px;
  background: #14181e;
  align-items: center;
  flex-wrap: wrap;
  border-top: 1px solid #2b323d;
  font-size: 13px;
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
}
button:hover { background: #374151; }
button.active {
  border-color: #60a5fa;
  background: #1e3a5f;
}
button.presenter-toggle {
  padding: 6px 9px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: #2563eb;
  border-color: #3b82f6;
  color: #ffffff;
}
button.presenter-toggle:hover {
  background: #1d4ed8;
}
button.presenter-toggle.panel-hidden {
  background: #1f242d;
  border-color: #4b5563;
  color: #9ca3af;
}
button.presenter-toggle.panel-hidden:hover {
  background: #2b323d;
  color: #e5e7eb;
}
button.nav-songs {
  background: #0f766e;
  border-color: #14b8a6;
  font-weight: 600;
}
button.btn-save {
  background: #16a34a;
  border-color: #22c55e;
  font-weight: 600;
}
button.btn-danger-right {
  background: #450a0a;
  border-color: #dc2626;
  color: #fecaca;
  font-weight: 700;
  margin-left: auto;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}
button.btn-danger-right:hover {
  background: #7f1d1d;
}
.shutdown-icon {
  color: #ef4444;
  font-size: 15px;
  line-height: 1;
  font-weight: 900;
}
button.btn-sm {
  padding: 5px 10px;
  font-size: 12px;
}
button.btn-transport {
  font-weight: 800;
  font-size: 14px;
  line-height: 1;
  padding: 6px 11px;
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
  display: inline-flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}
button.btn-srv-stop {
  background: #7f1d1d;
  border-color: #ef4444;
  font-weight: 700;
  padding: 6px 10px;
  font-size: 12px;
}
button.btn-srv-start {
  background: #14532d;
  border-color: #22c55e;
  font-weight: 700;
  padding: 6px 10px;
  font-size: 12px;
}
#presenter-controls {
  display: flex;
  gap: 6px;
  align-items: center;
  flex-wrap: wrap;
}
#songs {
  display: flex;
  flex-direction: row;
  gap: 6px;
  flex-wrap: wrap;
  align-items: center;
}
.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  display: inline-block;
  flex: 0 0 auto;
}
.dot-ok {
  background: #22c55e;
  box-shadow: 0 0 8px rgba(34, 197, 94, 0.7);
}
.dot-stop {
  background: #ef4444;
  box-shadow: 0 0 8px rgba(239, 68, 68, 0.7);
}
.ip-label {
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  color: #e2e8f0;
}
.auto-badge {
  font-size: 12px;
  padding: 4px 9px;
  border-radius: 999px;
  background: #1e293b;
  border: 1px solid #38bdf8;
  color: #7dd3fc;
  font-weight: 700;
}
.info {
  margin-left: auto;
  font-size: 12px;
  color: #9ca3af;
}
#stage {
  flex: 1 1 auto;
  display: block;
  width: 100%;
  min-height: 0;
  touch-action: manipulation;
}

#songs-view {
  flex: 1 1 auto;
  display: grid;
  grid-template-columns: 320px 1fr;
  min-height: 0;
  overflow: hidden;
}
.songs-sidebar {
  background: #161a20;
  border-right: 1px solid #2b323d;
  display: flex;
  flex-direction: column;
  padding: 12px;
  gap: 10px;
  min-height: 0;
}
.sidebar-top {
  display: flex;
  gap: 8px;
}
.sidebar-top input {
  flex: 1;
  min-width: 0;
}
#song-list {
  flex: 1 1 auto;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
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
  padding: 16px 20px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}
.editor-header h2 {
  margin: 0;
  font-size: 18px;
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
.span-2 { grid-column: span 2; }
.span-3 { grid-column: span 3; }
label.field {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 12px;
  color: #cbd5e1;
}
input[type="text"], input[type="number"], textarea {
  width: 100%;
  background: #1c2026;
  color: #f3f4f6;
  border: 1px solid #3d4756;
  border-radius: 6px;
  padding: 8px 10px;
  font-size: 14px;
  font-family: inherit;
}
textarea {
  min-height: 240px;
  resize: vertical;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  line-height: 1.45;
}
input:focus, textarea:focus {
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
  gap: 10px;
  align-items: center;
  flex-wrap: wrap;
}
.status-msg {
  font-size: 13px;
  min-height: 18px;
}
.status-ok { color: #4ade80; }
.status-err { color: #f87171; }

@media (max-width: 820px) {
  #songs-view {
    grid-template-columns: 1fr;
    grid-template-rows: 220px 1fr;
  }
  .songs-sidebar {
    border-right: none;
    border-bottom: 1px solid #2b323d;
  }
  .form-grid {
    grid-template-columns: 1fr;
  }
  .span-2, .span-3 { grid-column: span 1; }
}
@media (max-width: 600px) {
  .bar, .theme-bar, .player-bar, .bottom-bar { padding: 7px 8px; gap: 6px; }
  button { padding: 6px 10px; }
  .info {
    width: 100%;
    margin-left: 0;
    overflow-wrap: anywhere;
  }
}
</style>
</head>
<body>
<div class="bar" id="topbar">
  <button id="btn-presenter" class="presenter-toggle hidden" title="Ukryj panel prezentera" onclick="togglePresenter()">
    <svg id="icon-bust-on" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
      <circle cx="12" cy="8" r="4"/>
      <path d="M4 20c0-4 3.6-7 8-7s8 3 8 7H4z"/>
    </svg>
    <svg id="icon-bust-off" class="hidden" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
      <circle cx="12" cy="8" r="4"/>
      <path d="M4 20c0-4 3.6-7 8-7s8 3 8 7"/>
    </svg>
  </button>
  <button id="btn-songs-view" class="nav-songs hidden" onclick="toggleSongsView()">
    Piosenki
  </button>
  <div id="presenter-controls" class="hidden">
    <div id="songs"></div>
  </div>
  <div class="info" id="meta">Łączenie...</div>
</div>

<div id="theme-bar" class="theme-bar hidden">
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
    <input type="checkbox" id="chk-show-chords" checked onchange="onThemeChange()" />
    <span>Pokaż akordy</span>
  </label>
  <input type="text" id="theme-name" class="theme-preset-input" placeholder="Nazwa motywu..." />
  <button type="button" class="btn-sm btn-save" onclick="saveThemePreset()">Zapisz motyw</button>
  <select id="theme-select" class="theme-preset-select" onchange="selectThemePreset(this.value)">
    <option value="">Wybierz motyw...</option>
  </select>
</div>

<canvas id="stage"></canvas>

<div id="player-bar" class="player-bar hidden">
  <div class="player-left">
    <span id="auto-indicator" class="auto-badge hidden"></span>
  </div>
  <div class="transport-group" aria-label="Sterowanie prezentacją">
    <button type="button" id="btn-prev" class="btn-transport" title="Wstecz" onclick="manualStep('prev')">|◄</button>
    <button type="button" id="btn-pause" class="btn-transport" title="Pauza / Wznów automat i listę" onclick="togglePlaybackPause()">❚❚</button>
    <button type="button" id="btn-next" class="btn-transport" title="Dalej" onclick="manualStep('next')">►</button>
    <button type="button" id="btn-srv-toggle" class="btn-srv-stop" onclick="toggleServerState()">● [Stop]</button>
  </div>
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
      <button id="btn-delete-song" class="btn-danger-right hidden" onclick="deleteCurrentSong()">Usuń</button>
    </div>
  </section>
</div>

<div id="bottom-bar" class="bottom-bar hidden">
  <span id="srv-dot" class="status-dot dot-ok"></span>
  <span id="srv-ip" class="ip-label">http://192.168.43.14:8080</span>
  <button type="button" class="btn-sm" onclick="copyWifiAddress()">Kopiuj Wi-Fi</button>
  <button type="button" class="btn-sm btn-danger-right" title="Zamknij program" onclick="shutdownProgram()">
    <span class="shutdown-icon">◉</span> Zamknij
  </button>
</div>

<script>
const canvas = document.getElementById('stage');
const ctx = canvas.getContext('2d');
let currentState = null;
let presenterModeOpen = true;
let songsViewOpen = false;
let catalogSongs = [];
let editingFilename = null;
let resizeObserver = null;
let themesLoadedOnce = false;

let autoRemaining = 0;
let autoSignature = '';
let playbackPaused = false;
let songTransitionRemaining = 0;
let songTransitionTotal = 0;

const CHORD_PREFIX = '\x01';

const DEFAULT_THEME = {
  name: 'Ciemny klasyczny',
  bg: '#121417',
  text: '#f9fafb',
  chord: '#fbbf24',
  showChords: true
};

let activeTheme = { ...DEFAULT_THEME };

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
  if (!connected || !state) return false;
  if (typeof state.serverRunning === 'boolean') return state.serverRunning;
  return true;
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
    if (songTransitionRemaining > 0) {
      updateAutoBadge();
    } else if (autoRemaining > 0) {
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
  const bar = document.getElementById('bottom-bar');
  const dot = document.getElementById('srv-dot');
  const ipEl = document.getElementById('srv-ip');
  const toggleBtn = document.getElementById('btn-srv-toggle');
  if (!playerBar || !bar || !dot || !ipEl || !toggleBtn) return;

  const show = Boolean(state && state.canPresent && presenterModeOpen && !songsViewOpen);
  playerBar.classList.toggle('hidden', !show);
  bar.classList.toggle('hidden', !show);
  if (!show) return;

  const ip = state.ip || '127.0.0.1';
  const port = state.port || 8080;
  ipEl.textContent = `http://${ip}:${port}`;

  const active = isServerActive(state, connected);
  dot.className = 'status-dot ' + (active ? 'dot-ok' : 'dot-stop');

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
  } catch (_) {
    // Połączenie zostaje przerwane w momencie zamykania procesu
  }

  currentState = null;
  updateBottomBar(null, false);
  document.getElementById('meta').textContent = 'Program został zamknięty.';
  const themeBar = document.getElementById('theme-bar');
  if (themeBar) themeBar.classList.add('hidden');
  const playerBar = document.getElementById('player-bar');
  if (playerBar) playerBar.classList.add('hidden');
  const bar = document.getElementById('bottom-bar');
  if (bar) bar.classList.add('hidden');
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
    songsViewOpen ||
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
  songsViewOpen = !songsViewOpen;
  updateUiVisibility();
  if (songsViewOpen) {
    await loadSongsCatalog();
  } else {
    resize();
  }
}

function updateUiVisibility() {
  const btnPresenter = document.getElementById('btn-presenter');
  const iconBustOn = document.getElementById('icon-bust-on');
  const iconBustOff = document.getElementById('icon-bust-off');
  const btnSongsView = document.getElementById('btn-songs-view');
  const controls = document.getElementById('presenter-controls');
  const themeBar = document.getElementById('theme-bar');
  const songsView = document.getElementById('songs-view');

  const isPresenter = Boolean(currentState && currentState.canPresent);
  if (!isPresenter) {
    songsViewOpen = false;
  }

  btnSongsView.classList.toggle('hidden', !isPresenter);
  btnSongsView.textContent = songsViewOpen ? '← Prezentacja' : 'Piosenki';
  btnSongsView.classList.toggle('active', songsViewOpen);

  canvas.classList.toggle('hidden', songsViewOpen);
  songsView.classList.toggle('hidden', !songsViewOpen);

  if (!currentState) return;

  if (isPresenter && !songsViewOpen) {
    btnPresenter.classList.remove('hidden');
    btnPresenter.classList.toggle('panel-hidden', !presenterModeOpen);
    if (iconBustOn) iconBustOn.classList.toggle('hidden', !presenterModeOpen);
    if (iconBustOff) iconBustOff.classList.toggle('hidden', presenterModeOpen);
    btnPresenter.title = presenterModeOpen ? 'Ukryj panel prezentera' : 'Pokaż panel prezentera';
    controls.classList.toggle('hidden', !presenterModeOpen);
    if (themeBar) themeBar.classList.toggle('hidden', !presenterModeOpen);
    if (!themesLoadedOnce) {
      themesLoadedOnce = true;
      loadThemesList();
    }
  } else {
    btnPresenter.classList.add('hidden');
    controls.classList.add('hidden');
    if (themeBar) themeBar.classList.add('hidden');
  }
  updateBottomBar(currentState, true);
  updateAutoBadge();
}

function resize() {
  if (songsViewOpen) return;
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
  if (!currentState || songsViewOpen) return;
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
  const meta = document.getElementById('meta');
  const active = isServerActive(state, true);
  if (state.canPresent) {
    meta.textContent = active ? 'Prezenter aktywny' : 'Prezenter zatrzymany';
  } else {
    meta.textContent = active ? 'Podgląd uczestnika' : 'Prezentacja wstrzymana';
  }
  resize();
  if (!state.canPresent) return;

  const songsDiv = document.getElementById('songs');
  songsDiv.replaceChildren();
  const autoSongs = state.songs.filter(s => Boolean(s.automatic));
  autoSongs.forEach(song => {
    const button = document.createElement('button');
    button.textContent = song.title;
    if (song.index === state.songIndex) {
      button.classList.add('active');
    }
    button.onclick = () => manualStep(`song:${song.index}`);
    songsDiv.appendChild(button);
  });
  resize();
}

async function refreshState() {
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

window.addEventListener('keydown', event => {
  if (!currentState || !currentState.canPresent || songsViewOpen) return;
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
