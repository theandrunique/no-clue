```
src/
├── app.html
├── features/                          # Фичи (по домену)
│   ├── settings/
│   │   ├── AudioSettings.svelte
│   │   ├── ProviderSettings.svelte
│   │   └── index.ts
│   ├── system-prompts/
│   │   ├── SystemPromptsList.svelte
│   │   ├── CreateSystemPromptModal.svelte
│   │   ├── EditSystemPromptModal.svelte
│   │   └── index.ts
│   ├── conversation-list/
│   │   ├── ConversationList.svelte
│   │   └── index.ts
│   ├── llm-chat/
│   │   ├── LlmChatMessagesList.svelte
│   │   ├── LlmChatInput.svelte
│   │   ├── LlmChatQuickActions.svelte
│   │   └── index.ts
│   ├── overlay-menu/
│   │   ├── OverlayMenu.svelte
│   │   └── index.ts
│   └── transcript-list/
│       ├── TranscriptList.svelte
│       └── index.ts
├── lib/
│   ├── api/                           # HTTP-вызовы
│   ├── components/                    # Переиспользуемые компоненты
│   │   ├── form/                      # SelectField, TextField
│   │   └── ui/                        # Button, Card, Dialog/, Select/, Tabs/
│   ├── queries/                       # TanStack Query-хуки
│   ├── services/                      # .svelte.ts сервисы
│   ├── types/                         # Типы
│   └── utils/                         # Утилиты
├── routes/                            # SvelteKit-роуты
│   ├── (dashboard)/
│   │   ├── conversations/ # Список диалогов и возможность их открывать и работать в них
│   │   ├── overview/ # Пока не придумал, что тут разместить, но как минимум можно отображать недавние диалоги
│   │   ├── settings/ # Страница настроек, отображаем все разделы настроек audio, providers
│   │   └── system-prompts/ # Управление системными промптами, создание, изменение и выбор активного
│   └── overlay/
│       └── [conversationId]/
└── services/
    ├── system-prompts/
    │   ├── activePrompt.svelte.ts # Хранилище активного промпта
    │   ├── createSystemPromptModal.svelte.ts # Стор для вызова модалки создания
    │   └── editSystemPromptModal.svelte.ts # Стор для вызова модалки изменения
    ├── overlay/
    │   ├── OverlayContext.svelte # Создает и предостовляет контекст оверлея
    │   └── context.ts # Контекст оверлея
    ├── llm-chat/
    │   ├── LlmChatContext.svelte # Принимает conversationId и создает контекст для текущего диалога
    │   ├── service.svelte.ts # Сервис чата для диалога
    │   └── context.ts
    ├── transcriptions/
    │   ├── TranscriptionsContext.svelte  # Принимает conversationId и создает контекст для текущего диалога
    │   ├── service.svelte.ts
    │   └── context.ts
    └── settings/ # Через это место задаются настроки, управлется и сохраняются в localStorage
        ├── audioSettings.svelte.ts
        └── providerSettings.svelte.ts
```
