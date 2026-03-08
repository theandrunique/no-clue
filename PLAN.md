# No-Clue - Подробный план реализации

## 1. Технологический стек

| Компонент | Технология |
|-----------|------------|
| Framework | Tauri 2.x |
| Frontend | Vue 3 + TypeScript |
| UI | Radix UI + TailwindCSS |
| State | Pinia |
| Markdown | streamdown (streaming + анимация) |
| Storage | @tauri-apps/plugin-sql (SQLite) |
| Screen Capture | xcap (Rust) |

---

## 2. Архитектура окон

### Overlay (Основное окно)
- **URL**: `/` (корень)
- **Параметры**:
  - `label`: "overlay"
  - `width`: 600, `height`: 54
  - `decorations`: false
  - `transparent`: true
  - `alwaysOnTop`: true
  - `resizable`: false
  - `visibleOnAllWorkspaces`: true
  - `skipTaskbar`: true
  - `contentProtected`: true (скрытие от OBS/скриншотов)
  - `focus`: false
  - `acceptFirstMouse`: true
  - `shadow`: false

### Dashboard (Окно настроек)
- **URL**: `/dashboard`
- **Создание**: Динамически через Rust команду `open_dashboard`
- **Параметры**:
  - `label`: "dashboard"
  - `width`: 900, `height`: 700
  - `decorations`: true
  - `center`: true
  - `contentProtected`: true

---

## 3. Идентификатор приложения

- **Identifier**: `com.theandru.noclue`
- **ProductName**: `No-Clue`

---

## 4. UI/UX Архитектура

**Компоненты Overlay**:
- **Chevron кнопка** - раскрывает/скрывает popover
- **Mic toggle** - включение/выключение прослушки
- **Dashboard кнопка** - открыть dashboard
- **Draggable handle** - область для перемещения окна

---

## 5. Структура проекта

```
no-clue/
├── src/
│   ├── main.ts                    # Vue app entry
│   ├── App.vue                    # Root component
│   ├── router/
│   │   └── index.ts               # Vue Router config
│   ├── views/
│   │   ├── overlay/
│   │   │   ├── OverlayView.vue    # Main overlay window
│   │   │   └── components/        # Components only for overlay
│   │   │       ├── OverlayToolbar.vue
│   │   │       ├── OverlayPopover.vue
│   │   │       ├── ChatTab.vue
│   │   │       ├── TranscriptTab.vue
│   │   │       ├── QuickActions.vue
│   │   │       └── ClearButton.vue  # Очистка / начать новую сессию
│   │   └── dashboard/
│   │       ├── DashboardView.vue  # Dashboard container
│   │       └── components/        # Components only for dashboard
│   │           ├── DashboardSidebar.vue
│   │           ├── ChatsPanel.vue
│   │           ├── PromptsPanel.vue
│   │           ├── SettingsPanel.vue
│   │           ├── AudioPanel.vue
│   │           ├── ShortcutsPanel.vue
│   │           └── ProvidersPanel.vue
│   ├── components/
│   │   └── ui/                    # Radix UI + Tailwind components
│   ├── composables/               # Shared Vue composables 
│   │   ├── useTranscription.ts    # start/stop listening, transcripts, currentTranscript
│   │   ├── useChat.ts             # send message, streaming response
│   │   ├── useMediaRecorder.ts    # audio recording
│   │   ├── useSpeechRecognition.ts # speech to text
│   │   └── useShortcuts.ts        # global shortcuts
│   ├── stores/
│   │   ├── overlay.ts             # Overlay state: expanded, mic on, capture_screenshot, currentConversationId
│   │   ├── chat.ts                # Chat messages, history
│   │   ├── settings.ts            # User settings
│   │   ├── providers.ts           # AI providers config
│   │   └── audio.ts               # Audio devices, transcription state
│   ├── lib/
│   │   ├── ai.ts                  # AI request logic
│   │   ├── transcription.ts       # Deepgram integration
│   │   ├── screenshot.ts          # Screen capture
│   │   └── markdown.ts            # Marked streaming renderer
│   └── styles/
│       └── main.css               # Tailwind + custom styles
├── src-tauri/
│   ├── src/
│   │   ├── main.rs                # Tauri entry point
│   │   ├── lib.rs                 # Commands
│   │   └── windows.rs             # Window management
│   ├── tauri.conf.json            # Tauri config
│   └── capabilities/
│       └── default.json           # Permissions
└── package.json
```

---

## 6. Dashboard Вкладки

### 6.1 Chats (История чатов)
- Список всех чатов с датами
- Просмотр отдельного чата
- Удаление чатов
- Экспорт чатов

### 6.2 System Prompts
- CRUD операции для system prompts
- Выбор активного prompt
- Поля: name, content (текст)

### 6.3 Settings
- **Прозрачность** (opacity): 0-100%
- **Always On Top**: true/false
- **Stealth Mode**: (всегда включен, через contentProtected)

### 6.4 Audio
- Выбор **input device** (микрофон)
- Выбор **system audio** (виртуальный кабель)
- Тест аудио

### 6.5 Shortcuts
- Управление глобальными шорткатами
- Список по умолчанию:
  - `Ctrl+Enter` - Спросить об экране
  - `Ctrl+Shift+Up` - Скролл чата вверх
  - `Ctrl+Shift+Down` - Скролл чата вниз

### 6.6 Providers (AI + Transcription)

#### AI Providers
```typescript
export const AI_PROVIDERS = [
  {
    id: "openrouter",
    name: "OpenRouter",
    fields: ["API_KEY", "MODEL"],
    curl: `curl https://openrouter.ai/api/v1/chat/completions ...`,
    responseContentPath: "choices[0].message.content",
    streaming: true,
  },
  // Другие провайдеры...
]
```

Пользователь заполняет `{{API_KEY}}`, `{{MODEL}}`.
Автоматически подставляются: `{{TEXT}}`, `{{IMAGE}}`, `{{SYSTEM_PROMPT}}`

#### Transcription Providers
- **Deepgram** (по умолчанию)
- Настройка API ключа
- Выбор модели

---

## 6b. Conversation (Сессия) Use-case

### Поток:
1. Приложение открыто → overlay пустой
2. Пользователь включает транскрипцию → **создается НОВЫЙ conversation** (т.к. транскрипции должны куда-то сохраняться)
3. Транскрипции пишутся в этот conversation
4. Пользователь отправляет сообщение → продолжается тот же conversation
5. Пользователь нажал "очистку" (ClearButton):
   - Текущий conversation уже сохранен в БД (все транскрипции + сообщения)
   - Если транскрипция ВКЛ → создается новый conversation, currentConversationId = newId
   - Если транскрипция ВЫКЛ → currentConversationId = null (просто очищаем UI)
6. В Dashboard → можно просмотреть всю историю любых conversation

### Синхронизация (overlay.ts store):
```typescript
interface OverlayState {
  currentConversationId: string | null  // null = нет активной сессии
  isTranscriptionEnabled: boolean
  captureScreenshot: boolean
}
```

**Включение транскрипции:**
- If `currentConversationId == null` → `create_conversation()` → сохраняем id в store

**Очистка:**
- Текущий conversation уже в БД
- If `isTranscriptionEnabled == true` → `create_conversation()` → новый id
- Else → `currentConversationId = null`

### Терминология:
- **Conversation** = сессия = один разговор (транскрипции + сообщения LLM + скриншоты)
- **Очистка** = завершить текущий conversation, начать новый (если транскрипция включена)

---

## 7. Функциональные требования

### 7.1 AI Chat
- [ ] Отправка сообщения с контекстом:
  - System prompt (из настроек)
  - История чата
  - Транскрипция
  - Скриншот экрана
- [ ] Streaming ответ (отображать по чанкам)
- [ ] Markdown рендеринг с подсветкой кода
- [ ] Quick actions (заготовленные фразы)

### 7.2 Transcription
**Полностью на Rust**, Vue только слушает events и сохраняет в SQLite.

- [ ] Rust: подключение к Deepgram WebSocket (микрофон + системное аудио)
- [ ] Rust: эмиттит events `transcription-result`, `transcription-started`, `transcription-stopped`, `transcription-error`
- [ ] Vue `useTranscription()`: слушает events
- [ ] При получении `transcription-result` (is_final=true): Rust сам сохраняет в SQLite (текущая сессия уже известна на бэкенде)
- [ ] Отображение: текущая (нефинальная) + история финальных транскрипций

### 7.3 Overlay
- [ ] Перемещение по drag handle
- [ ] Перемещение через шорткаты (Ctrl+arrows)
- [ ] Expand/collapse popover
- [ ] Toggle microphone
- [ ] Open dashboard

### 7.4 Dashboard
- [ ] Sidebar навигация
- [ ] Все вкладки из раздела 6
- [ ] Сохранение настроек в SQLite

---

## 8. Rust Команды (src-tauri)

```rust
// lib.rs

// Window management
#[tauri::command]
fn open_dashboard(app: AppHandle) -> Result<(), String>

#[tauri::command]
fn move_overlay(app: AppHandle, direction: String, step: i32) -> Result<(), String>

#[tauri::command]
fn set_overlay_height(window: WebviewWindow, height: u32) -> Result<(), String>

#[tauri::command]
fn set_overlay_visible(window: WebviewWindow, visible: bool) -> Result<(), String>

// LLM Proxy - Rust reads settings from SQLite, API keys never exposed to frontend
#[tauri::command]
async fn chat_completion(
    conversation_id: String,
    user_message: String,
    capture_screenshot: bool,  // Rust сам делает скриншот если true (xcap)
) -> Result<StreamReader, String>
// Rust самостоятельно:
// 1. Если capture_screenshot=true → xcap делает скриншот → base64
// 2. Читает из SQLite: active provider, api_key, model, system_prompt
// 3. Читает историю сообщений для conversation_id
// 4. Формирует запрос к LLM и возвращает стрим

// Transcription (Deepgram WebSocket) - полностью на Rust
#[tauri::command]
async fn start_transcription(config: DeepgramConfig) -> Result<(), String>
// - Подключается к Deepgram WebSocket
// - Слушает аудио (микрофон + системное)
// - Эмиттит events: transcription-started, transcription-result, transcription-stopped, transcription-error
// - Сам сохраняет транскрипции в SQLite (текущая сессия известна)

#[tauri::command]
async fn stop_transcription() -> Result<(), String>

// Conversation management
#[tauri::command]
fn create_conversation(app: AppHandle) -> Result<Conversation, String>  // Returns full conversation object

#[tauri::command]
fn get_conversations(app: AppHandle) -> Result<Vec<ConversationSummary>, String>  // For dashboard list

#[tauri::command]
fn get_conversation(id: String) -> Result<ConversationDetail, String>  // Full with messages + transcripts

// Transcription session management
#[tauri::command]
fn update_transcription_session(conversation_id: String) -> Result<(), String>  // Обновить текущую сессию для транскрипций

// Database initialization
#[tauri::command]
fn init_database(app: AppHandle) -> Result<(), String>

// AI Provider Settings
#[tauri::command]
fn save_provider_settings(provider: String, api_key: String, model: String) -> Result<(), String>

#[tauri::command]
fn get_provider_settings(provider: String) -> Result<ProviderSettings, String>

#[tauri::command]
fn get_all_providers() -> Result<Vec<ProviderInfo>, String>  // Список доступных провайдеров

// STT Settings (Deepgram)
#[tauri::command]
fn save_stt_settings(api_key: String, model: String, language: String) -> Result<(), String>

#[tauri::command]
fn get_stt_settings() -> Result<SttSettings, String>

// App Settings
#[tauri::command]
fn save_app_settings(key: String, value: String) -> Result<(), String>

#[tauri::command]
fn get_app_settings(key: String) -> Result<String, String>
```

---

## 8b. Tauri Events (транскрипция)

| Event | Payload | Описание |
|-------|---------|----------|
| `transcription-connected` | - | Подключились к Deepgram |
| `transcription-started` | - | Транскрипция началась |
| `transcription-result` | `{text: string, is_final: boolean, confidence: number?}` | Результат (промежуточный или финальный) |
| `transcription-stopped` | - | Транскрипция остановлена |
| `transcription-error` | `{message: string, error_type: string}` | Ошибка |

---

## 9. База данных (SQLite)

```sql
-- Conversations table
CREATE TABLE IF NOT EXISTS conversations (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Messages table (LLM чат)
CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('user', 'assistant', 'system')),
    content TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
);

-- Transcripts table (транскрипции)
CREATE TABLE IF NOT EXISTS transcripts (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    speaker TEXT NOT NULL CHECK(speaker IN ('user', 'system')),  -- user = микрофон, system = системное аудио
    text TEXT NOT NULL,
    is_final INTEGER NOT NULL DEFAULT 1,  -- 1 = final, 0 = interim (не сохраняем)
    confidence REAL,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
);

-- System prompts table
CREATE TABLE IF NOT EXISTS system_prompts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    prompt TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now')) NOT NULL,
    updated_at TEXT DEFAULT (datetime('now')) NOT NULL
);

-- Provider settings (AI providers - OpenRouter etc)
CREATE TABLE IF NOT EXISTS provider_settings (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL UNIQUE,
    api_key TEXT NOT NULL,
    model TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- STT settings (Deepgram)
CREATE TABLE IF NOT EXISTS stt_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),  -- Only one row
    api_key TEXT NOT NULL,
    model TEXT DEFAULT 'nova-3',
    language TEXT DEFAULT 'ru',
    updated_at INTEGER NOT NULL
);

-- App settings
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_conversations_updated_at ON conversations(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_messages_conversation_timestamp ON messages(conversation_id, timestamp ASC);

-- Triggers for auto-updating conversation timestamp
CREATE TRIGGER IF NOT EXISTS update_conversation_timestamp_on_message_insert
AFTER INSERT ON messages
FOR EACH ROW
BEGIN
    UPDATE conversations SET updated_at = NEW.timestamp WHERE id = NEW.conversation_id;
END;
```

---

## 9. Tauri Permissions

```json
{
  "permissions": [
    "core:default",
    "core:window:default",
    "core:window:allow-close",
    "core:window:allow-show",
    "core:window:allow-hide",
    "core:window:allow-set-position",
    "core:window:allow-set-size",
    "core:window:allow-set-always-on-top",
    "core:window:allow-set-skip-taskbar",
    "core:window:allow-set-decorations",
    "core:window:allow-set-focus",
    "core:window:allow-start-dragging",
    "core:window:allow-is-visible",
    "core:tray:default",
    "global-shortcut:default",
    "sql:default",
    "shell:default"
  ]
}
```

---

## 10. Глобальные шорткаты (план)

| Шорткат | Действие |
|---------|----------|
| `Ctrl+Enter` | Спросить об экране и аудио |
| `Ctrl+Up/Down/Left/Right` | Двигать overlay |
| `Ctrl+Shift+Up` | Скролл чата вверх |
| `Ctrl+Shift+Down` | Скролл чата вниз |

---

## 11. Этапы реализации

### Этап 1: Базовая структура
- [x] Tauri + Vue 3 проект
- [ ] Обновить tauri.conf.json
- [ ] Настроить capabilities
- [ ] Vue Router
- [ ] Pinia stores

### Этап 2: Overlay окно
- [ ] OverlayView.vue - базовая структура
- [ ] Draggable handle
- [ ] Кнопки (chevron, mic, dashboard)
- [ ] Popover с табами

### Этап 3: Dashboard окно
- [ ] Rust команда open_dashboard
- [ ] DashboardView.vue
- [ ] Sidebar навигация

### Этап 4: AI Providers
- [ ] UI для выбора провайдера
- [ ] Форма настроек API
- [ ] AI request логика

### Этап 5: Chat
- [ ] ChatStore
- [ ] ChatTab.vue
- [ ] Markdown рендеринг
- [ ] Quick actions
- [ ] Screenshot capture

### Этап 6: Transcription
- [ ] Интеграция Deepgram
- [ ] TranscriptTab.vue
- [ ] Audio device selection

### Этап 7: Shortcuts
- [ ] Global shortcuts plugin
- [ ] ShortcutsPanel.vue
- [ ] Реализация перемещения overlay

### Этап 8: Persistence
- [ ] SQLite database
- [ ] Сохранение чатов
- [ ] Сохранение настроек

---

## 12. Вопросы/Уточнения

1. **Транскрипция** - будет импортирована из другого проекта
2. **Stealth Mode** - реализован через `contentProtected: true`
3. **Identifier** - `com.theandru.noclue`
4. **LLM архитектура**:
   - Frontend → Rust (только conversation_id, message, image_base64)
   - Rust сам читает из SQLite: active provider, api_key, model, system_prompt
   - Rust формирует запрос → LLM API → стрим обратно во Frontend
   - API ключи НИКОГДА не передаются во Frontend
5. **Скриншоты** - Rust (xcap) → base64 → отправка в LLM
6. **Streamdown** - для рендеринга markdown со стримингом

