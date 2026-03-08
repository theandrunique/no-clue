# No-Clue - Подробный план реализации

## 1. Технологический стек

| Компонент | Технология |
|-----------|------------|
| Framework | Tauri 2.x |
| Frontend | Vue 3 + TypeScript |
| UI | Radix UI + TailwindCSS |
| State | Pinia |
| Markdown | marked (с поддержкой стриминга) |
| Storage | @tauri-apps/plugin-store (персистентность) |
| Database | @tauri-apps/plugin-sql (SQLite) |

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

### Overlay Window

```
┌─────────────────────────────────────────────────────────┐
│  ┌────┐  ┌────┐  ┌────┐                                │
│  │ ◀ ▶ │  │ 🎤 │  │ ⚙ │    [draggable area]           │
│  └────┘  └────┘  └────┘                                │
└─────────────────────────────────────────────────────────┘
                    │
                    ▼ (при нажатии на ◀▶)
┌─────────────────────────────────────────────────────────┐
│  [Chat] [Transcript]                                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Content area (Chat or Transcript)                     │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Компоненты Overlay**:
- **Chevron кнопка** - раскрывает/скрывает popover
- **Mic toggle** - включение/выключение прослушки
- **Dashboard кнопка** - открыть dashboard
- **Draggable handle** - область для перемещения окна

### Popover (Chat Tab)

```
┌─────────────────────────────────────────────────────────┐
│  [Chat] [Transcript]                                    │
├─────────────────────────────────────────────────────────┤
│  Quick Actions: [Что на экране?] [Помоги с этим] ...   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Messages:                                              │
│  ┌─────────────────────────────────────────────────┐   │
│  │ User: Что видишь на экране?                      │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ AI: (streaming response...)                     │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
├─────────────────────────────────────────────────────────┤
│  [Input message...]                          [Send]    │
└─────────────────────────────────────────────────────────┘
```

### Popover (Transcript Tab)

```
┌─────────────────────────────────────────────────────────┐
│  [Chat] [Transcript]                                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 🎤 User: текст с микрофона                        │   │
│  ├─────────────────────────────────────────────────┤   │
│  │ 🔊 System: текст системных звуков                │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Dashboard Window

```
┌─────────────────────────────────────────────────────────┐
│  [Sidebar]  │  [Content Area]                           │
│             │                                           │
│  Chats      │  Tab content (Chats/Settings/Providers)  │
│  ─────────  │                                           │
│  Prompts    │                                           │
│  ─────────  │                                           │
│  Settings   │                                           │
│  ─────────  │                                           │
│  Audio      │                                           │
│  ─────────  │                                           │
│  Shortcuts  │                                           │
│  ─────────  │                                           │
│  Providers  │                                           │
└─────────────────────────────────────────────────────────┘
```

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
│   │   ├── OverlayView.vue        # Main overlay window
│   │   └── DashboardView.vue      # Dashboard container
│   ├── components/
│   │   ├── overlay/
│   │   │   ├── OverlayToolbar.vue # Top bar with buttons
│   │   │   ├── OverlayPopover.vue # Expandable popover
│   │   │   ├── ChatTab.vue        # Chat with LLM
│   │   │   ├── TranscriptTab.vue  # Live transcription
│   │   │   └── QuickActions.vue    # Quick action buttons
│   │   ├── dashboard/
│   │   │   ├── DashboardSidebar.vue
│   │   │   ├── ChatsPanel.vue     # Chat history
│   │   │   ├── PromptsPanel.vue   # System prompts
│   │   │   ├── SettingsPanel.vue  # App settings
│   │   │   ├── AudioPanel.vue     # Audio devices
│   │   │   ├── ShortcutsPanel.vue # Keyboard shortcuts
│   │   │   └── ProvidersPanel.vue  # AI + Transcription providers
│   │   └── ui/                    # Radix UI + Tailwind components
│   ├── stores/
│   │   ├── overlay.ts             # Overlay state (expanded, mic on)
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
- [ ] Захват аудио с микрофона
- [ ] Захват системных звуков (через виртуальный кабель)
- [ ] Realtime transcription (Deepgram)
- [ ] Отображение диалога: User vs System

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
#[tauri::command]
fn open_dashboard(app: AppHandle) -> Result<(), String>

#[tauri::command]
fn move_overlay(app: AppHandle, direction: String, step: i32) -> Result<(), String>

#[tauri::command]
fn set_overlay_height(window: WebviewWindow, height: u32) -> Result<(), String>

#[tauri::command]
fn set_overlay_visible(window: WebviewWindow, visible: bool) -> Result<(), String>
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
    "store:default",
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
4. **Markdown** - используем `marked` для стриминга

