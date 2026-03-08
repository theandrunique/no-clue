# Agent Guidelines for no-clue

This document provides guidelines for agents working on this codebase.

## Project Overview

This is a Tauri v2 desktop application using Vue 3 + TypeScript for the frontend and Rust for the backend.

## Build Commands

### Frontend (Vue/TypeScript)

| Command | Description |
|---------|-------------|
| `npm run dev` | Start Vite dev server (port 1420) |
| `npm run build` | Type check with `vue-tsc --noEmit`, then build for production |
| `npm run preview` | Preview the production build |

### Tauri Commands

| Command | Description |
|---------|-------------|
| `npm run tauri dev` | Run Tauri in development mode |
| `npm run tauri build` | Build production Tauri app |
| `npm run tauri build -- --debug` | Build Tauri app with debug symbols |

### Backend (Rust)

Run from `src-tauri/` directory:

| Command | Description |
|---------|-------------|
| `cargo build` | Compile the Rust code |
| `cargo run` | Run the Rust backend |
| `cargo test` | Run Rust tests |
| `cargo clippy` | Run linting |
| `cargo fmt` | Format Rust code |

### Single Test Commands

- **Vue/TypeScript**: No test framework is currently configured
- **Rust**: `cargo test -- <test_name>` (run from `src-tauri/`)

## Code Style Guidelines

### General

- Use 2 spaces for indentation
- No trailing whitespace
- Use semicolons in TypeScript

### TypeScript

- Strict mode is enabled in `tsconfig.json`
- All strict checks must pass (`noUnusedLocals`, `noUnusedParameters`, `noFallthroughCasesInSwitch`)
- Use explicit types for function parameters and return types when not inferrable
- Use the Composition API with `<script setup lang="ts">`

### Vue Components

- Use `<script setup lang="ts">` syntax
- Use `ref()` and `reactive()` for reactive state
- Use TypeScript for prop types and emits

Example:
```vue
<script setup lang="ts">
import { ref, computed } from "vue";

interface Props {
  title: string;
}

const props = defineProps<Props>();
const count = ref(0);

const doubled = computed(() => count.value * 2);

function increment() {
  count.value++;
}
</script>
```

### Imports

- Use absolute imports from package names (e.g., `@tauri-apps/api/core`)
- Use relative imports for local files (e.g., `./components/Foo.vue`)
- Group imports: external packages, then relative imports
- Sort alphabetically within groups

### Rust

- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Use `#[tauri::command]` for Tauri commands
- Follow standard Rust conventions (snake_case for functions/variables, PascalCase for types)

### Error Handling

- In Rust: Use `Result` types and `?` operator for propagating errors
- In Vue/TypeScript: Use try/catch for async operations, handle errors gracefully
- Never leave `.expect()` calls in production code (except for Tauri context generation)

### Tauri API Usage

- Use `@tauri-apps/api/core` for invoking commands
- Use `invoke<T>(command: string, args?: object): Promise<T>` for calling Rust commands

Example:
```typescript
import { invoke } from "@tauri-apps/api/core";

const result = await invoke<string>("greet", { name: "World" });
```

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Vue components | PascalCase | `UserProfile.vue` |
| TypeScript files | kebab-case | `user-service.ts` |
| Composables | camelCase, start with `use` | `useCounter.ts` |
| Rust functions | snake_case | `fn greet_user()` |
| Rust types | PascalCase | `struct UserConfig` |
| Rust modules | snake_case | `mod user_auth` |
