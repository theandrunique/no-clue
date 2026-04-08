use crate::models::shortcut::{PlatformDefaults, ShortcutDef, ValidationRules};

pub fn get_shortcut_definitions() -> Vec<ShortcutDef> {
    vec![
        ShortcutDef {
            id: "move_window_up",
            name: "Move Window Up",
            description: "Moves the overlay window up.",
            defaults: PlatformDefaults {
                windows: "Ctrl+ArrowUp",
                macos: "Cmd+ArrowUp",
                linux: "Ctrl+ArrowUp",
            },
            rules: ValidationRules {
                requires_modifier: true,
            },
        },
        ShortcutDef {
            id: "move_window_down",
            name: "Move Window Down",
            description: "Moves the overlay window down.",
            defaults: PlatformDefaults {
                windows: "Ctrl+ArrowDown",
                macos: "Cmd+ArrowDown",
                linux: "Ctrl+ArrowDown",
            },
            rules: ValidationRules {
                requires_modifier: true,
            },
        },
        ShortcutDef {
            id: "move_window_right",
            name: "Move Window Right",
            description: "Moves the overlay window right.",
            defaults: PlatformDefaults {
                windows: "Ctrl+ArrowRight",
                macos: "Cmd+ArrowRight",
                linux: "Ctrl+ArrowRight",
            },
            rules: ValidationRules {
                requires_modifier: true,
            },
        },
        ShortcutDef {
            id: "move_window_left",
            name: "Move Window Left",
            description: "Moves the overlay window left.",
            defaults: PlatformDefaults {
                windows: "Ctrl+ArrowLeft",
                macos: "Cmd+ArrowLeft",
                linux: "Ctrl+ArrowLeft",
            },
            rules: ValidationRules {
                requires_modifier: true,
            },
        },
        ShortcutDef {
            id: "toggle_overlay_visibility",
            name: "Toggle Overlay",
            description: "Toggles the visibility of the overlay window.",
            defaults: PlatformDefaults {
                windows: "Ctrl+Backslash",
                macos: "Cmd+Backslash",
                linux: "Ctrl+Backslash",
            },
            rules: ValidationRules {
                requires_modifier: true,
            },
        },
        ShortcutDef {
            id: "ask_for_help",
            name: "Ask For Help",
            description: "Triggers the AI assistant to help with the current context.",
            defaults: PlatformDefaults {
                windows: "Ctrl+Enter",
                macos: "Cmd+Enter",
                linux: "Ctrl+Enter",
            },
            rules: ValidationRules {
                requires_modifier: true,
            },
        },
        ShortcutDef {
            id: "scroll_chat_up",
            name: "Scroll Chat Up",
            description: "Scrolls the chat history up.",
            defaults: PlatformDefaults {
                windows: "Ctrl+Shift+ArrowUp",
                macos: "Cmd+Shift+ArrowUp",
                linux: "Ctrl+Shift+ArrowUp",
            },
            rules: ValidationRules {
                requires_modifier: true,
            },
        },
        ShortcutDef {
            id: "scroll_chat_down",
            name: "Scroll Chat Down",
            description: "Scrolls the chat history down.",
            defaults: PlatformDefaults {
                windows: "Ctrl+Shift+ArrowDown",
                macos: "Cmd+Shift+ArrowDown",
                linux: "Ctrl+Shift+ArrowDown",
            },
            rules: ValidationRules {
                requires_modifier: true,
            },
        },
    ]
}

#[cfg(target_os = "macos")]
pub fn get_default_for_platform(def: &ShortcutDef) -> &'static str {
    def.defaults.macos
}

#[cfg(target_os = "windows")]
pub fn get_default_for_platform(def: &ShortcutDef) -> &'static str {
    def.defaults.windows
}

#[cfg(target_os = "linux")]
pub fn get_default_for_platform(def: &ShortcutDef) -> &'static str {
    def.defaults.linux
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn get_default_for_platform(def: &ShortcutDef) -> &'static str {
    def.defaults.windows
}
