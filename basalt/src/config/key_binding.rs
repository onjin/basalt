use std::fmt;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

use crate::{
    app::{Message, ScrollAmount},
    explorer, help_modal, outline, splash_modal, vault_selector_modal,
};
use crate::{config::ConfigError, note_editor};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub(crate) struct KeyBinding {
    pub key: Key,
    pub command: Command,
}

impl From<(Key, Command)> for KeyBinding {
    fn from((key, command): (Key, Command)) -> Self {
        Self::new(key, command)
    }
}

impl KeyBinding {
    pub const fn new(key: Key, command: Command) -> Self {
        Self { key, command }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Key {
    pub modifiers: KeyModifiers,
    pub code: KeyCode,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = self.code.to_string().to_lowercase().replace(" ", "_");

        if self.modifiers.is_empty() {
            write!(f, "{code}")
        } else {
            let modifiers = self.modifiers.to_string().to_ascii_lowercase();
            write!(f, "{modifiers}-{code}")
        }
    }
}

impl Key {
    pub const CTRL_C: Key = Key::new(KeyCode::Char('c'), KeyModifiers::CONTROL);

    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

impl From<char> for Key {
    fn from(value: char) -> Self {
        Self::new(KeyCode::Char(value), KeyModifiers::NONE)
    }
}

impl From<(KeyCode, KeyModifiers)> for Key {
    fn from((code, modifiers): (KeyCode, KeyModifiers)) -> Self {
        Self::new(code, modifiers)
    }
}

impl From<(char, KeyModifiers)> for Key {
    fn from((char, modifiers): (char, KeyModifiers)) -> Self {
        Self::new(KeyCode::Char(char), modifiers)
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(KeyVisitor)
    }
}

struct KeyVisitor;

impl Visitor<'_> for KeyVisitor {
    type Value = Key;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string whose format is either 'key' or 'modifier+key'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = value.to_lowercase();
        let mut parts = value.split('+');
        // Does not panic if the str is empty
        let code = parts.by_ref().next_back().unwrap();
        let modifiers = parts
            .map(parse_modifiers)
            .collect::<Result<Vec<KeyModifiers>, ConfigError>>()
            .map_err(de::Error::custom)?
            .into_iter()
            .reduce(|acc, modifiers| acc.union(modifiers))
            .unwrap_or(KeyModifiers::NONE);

        Ok(Key {
            modifiers,
            code: parse_code(code).map_err(de::Error::custom)?,
        })
    }
}

fn parse_modifiers(modifiers: &str) -> Result<KeyModifiers, ConfigError> {
    match modifiers {
        "" => Ok(KeyModifiers::NONE),
        "alt" => Ok(KeyModifiers::ALT),
        "ctrl" | "control" => Ok(KeyModifiers::CONTROL),
        "hyper" => Ok(KeyModifiers::HYPER),
        "meta" => Ok(KeyModifiers::META),
        "shift" => Ok(KeyModifiers::SHIFT),
        "super" => Ok(KeyModifiers::SUPER),
        _ => Err(ConfigError::UnknownKeyModifiers(modifiers.to_string())),
    }
}
fn parse_code(code: &str) -> Result<KeyCode, ConfigError> {
    match code.len() {
        0 => Some(KeyCode::Null),
        1 => Some(KeyCode::Char(code.chars().next().unwrap())),
        _ => code
            .strip_prefix('f')
            .and_then(|n| n.parse::<u8>().map(KeyCode::F).ok())
            .or(match code {
                "esc" => Some(KeyCode::Esc),
                "space" => Some(KeyCode::Char(' ')),
                "backspace" => Some(KeyCode::Backspace),
                "backtab" => Some(KeyCode::BackTab),
                "delete" => Some(KeyCode::Delete),
                "down" => Some(KeyCode::Down),
                "end" => Some(KeyCode::End),
                "enter" => Some(KeyCode::Enter),
                "home" => Some(KeyCode::Home),
                "insert" => Some(KeyCode::Insert),
                "left" => Some(KeyCode::Left),
                "page_down" => Some(KeyCode::PageDown),
                "page_up" => Some(KeyCode::PageUp),
                "right" => Some(KeyCode::Right),
                "tab" => Some(KeyCode::Tab),
                "up" => Some(KeyCode::Up),
                _ => None,
            }),
    }
    .ok_or(ConfigError::UnknownKeyCode(code.to_string()))
}

impl de::Error for ConfigError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        ConfigError::InvalidKeybinding(msg.to_string())
    }
}

impl From<&KeyEvent> for Key {
    fn from(event: &KeyEvent) -> Self {
        Self {
            code: event.code,
            modifiers: event.modifiers,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Command {
    Quit,

    SplashUp,
    SplashDown,
    SplashOpen,

    ExplorerUp,
    ExplorerDown,
    ExplorerOpen,
    ExplorerSort,
    ExplorerToggle,
    ExplorerToggleOutline,
    ExplorerSwitchPaneNext,
    ExplorerSwitchPanePrevious,
    ExplorerScrollUpOne,
    ExplorerScrollDownOne,
    ExplorerScrollUpHalfPage,
    ExplorerScrollDownHalfPage,

    OutlineUp,
    OutlineDown,
    OutlineSelect,
    OutlineExpand,
    OutlineToggle,
    OutlineToggleExplorer,
    OutlineSwitchPaneNext,
    OutlineSwitchPanePrevious,

    HelpModalScrollUpOne,
    HelpModalScrollDownOne,
    HelpModalScrollUpHalfPage,
    HelpModalScrollDownHalfPage,
    HelpModalToggle,
    HelpModalClose,

    NoteEditorScrollUpOne,
    NoteEditorScrollDownOne,
    NoteEditorScrollUpHalfPage,
    NoteEditorScrollDownHalfPage,
    NoteEditorSwitchPaneNext,
    NoteEditorSwitchPanePrevious,
    NoteEditorToggleExplorer,
    NoteEditorToggleOutline,
    NoteEditorCursorUp,
    NoteEditorCursorDown,

    // # Experimental editor
    NoteEditorExperimentalCursorWordForward,
    NoteEditorExperimentalCursorWordBackward,
    NoteEditorExperimentalSetEditMode,
    NoteEditorExperimentalSetReadMode,
    NoteEditorExperimentalSave,
    NoteEditorExperimentalExitMode,
    NoteEditorExperimentalCursorLeft,
    NoteEditorExperimentalCursorRight,

    VaultSelectorModalUp,
    VaultSelectorModalDown,
    VaultSelectorModalClose,
    VaultSelectorModalOpen,
    VaultSelectorModalToggle,
}

impl From<Command> for Message<'_> {
    fn from(value: Command) -> Self {
        match value {
            Command::Quit => Message::Quit,

            Command::SplashUp => Message::Splash(splash_modal::Message::Up),
            Command::SplashDown => Message::Splash(splash_modal::Message::Down),
            Command::SplashOpen => Message::Splash(splash_modal::Message::Open),

            Command::ExplorerUp => Message::Explorer(explorer::Message::Up),
            Command::ExplorerDown => Message::Explorer(explorer::Message::Down),
            Command::ExplorerOpen => Message::Explorer(explorer::Message::Open),
            Command::ExplorerSort => Message::Explorer(explorer::Message::Sort),
            Command::ExplorerToggle => Message::Explorer(explorer::Message::Toggle),
            Command::ExplorerToggleOutline => Message::Explorer(explorer::Message::ToggleOutline),
            Command::ExplorerSwitchPaneNext => Message::Explorer(explorer::Message::SwitchPaneNext),
            Command::ExplorerSwitchPanePrevious => {
                Message::Explorer(explorer::Message::SwitchPanePrevious)
            }
            Command::ExplorerScrollUpOne => {
                Message::Explorer(explorer::Message::ScrollUp(ScrollAmount::One))
            }
            Command::ExplorerScrollDownOne => {
                Message::Explorer(explorer::Message::ScrollDown(ScrollAmount::One))
            }
            Command::ExplorerScrollUpHalfPage => {
                Message::Explorer(explorer::Message::ScrollUp(ScrollAmount::HalfPage))
            }
            Command::ExplorerScrollDownHalfPage => {
                Message::Explorer(explorer::Message::ScrollDown(ScrollAmount::HalfPage))
            }

            Command::OutlineUp => Message::Outline(outline::Message::Up),
            Command::OutlineDown => Message::Outline(outline::Message::Down),
            Command::OutlineSelect => Message::Outline(outline::Message::Select),
            Command::OutlineExpand => Message::Outline(outline::Message::Expand),
            Command::OutlineToggle => Message::Outline(outline::Message::Toggle),
            Command::OutlineToggleExplorer => Message::Outline(outline::Message::ToggleExplorer),
            Command::OutlineSwitchPaneNext => Message::Outline(outline::Message::SwitchPaneNext),
            Command::OutlineSwitchPanePrevious => {
                Message::Outline(outline::Message::SwitchPanePrevious)
            }

            Command::HelpModalScrollUpOne => {
                Message::HelpModal(help_modal::Message::ScrollUp(ScrollAmount::One))
            }
            Command::HelpModalScrollDownOne => {
                Message::HelpModal(help_modal::Message::ScrollDown(ScrollAmount::One))
            }
            Command::HelpModalScrollUpHalfPage => {
                Message::HelpModal(help_modal::Message::ScrollUp(ScrollAmount::HalfPage))
            }
            Command::HelpModalScrollDownHalfPage => {
                Message::HelpModal(help_modal::Message::ScrollDown(ScrollAmount::HalfPage))
            }
            Command::HelpModalToggle => Message::HelpModal(help_modal::Message::Toggle),
            Command::HelpModalClose => Message::HelpModal(help_modal::Message::Close),

            Command::NoteEditorScrollUpOne => {
                Message::NoteEditor(note_editor::Message::ScrollUp(ScrollAmount::One))
            }
            Command::NoteEditorScrollDownOne => {
                Message::NoteEditor(note_editor::Message::ScrollDown(ScrollAmount::One))
            }
            Command::NoteEditorScrollUpHalfPage => {
                Message::NoteEditor(note_editor::Message::ScrollUp(ScrollAmount::HalfPage))
            }
            Command::NoteEditorScrollDownHalfPage => {
                Message::NoteEditor(note_editor::Message::ScrollDown(ScrollAmount::HalfPage))
            }
            Command::NoteEditorSwitchPaneNext => {
                Message::NoteEditor(note_editor::Message::SwitchPaneNext)
            }
            Command::NoteEditorSwitchPanePrevious => {
                Message::NoteEditor(note_editor::Message::SwitchPanePrevious)
            }
            Command::NoteEditorCursorUp => Message::NoteEditor(note_editor::Message::CursorUp),
            Command::NoteEditorCursorDown => Message::NoteEditor(note_editor::Message::CursorDown),
            Command::NoteEditorToggleExplorer => {
                Message::NoteEditor(note_editor::Message::ToggleExplorer)
            }
            Command::NoteEditorToggleOutline => {
                Message::NoteEditor(note_editor::Message::ToggleOutline)
            }
            // Experimental
            Command::NoteEditorExperimentalSetEditMode => {
                Message::NoteEditor(note_editor::Message::EditMode)
            }
            Command::NoteEditorExperimentalSetReadMode => {
                Message::NoteEditor(note_editor::Message::ReadMode)
            }
            Command::NoteEditorExperimentalSave => Message::NoteEditor(note_editor::Message::Save),
            Command::NoteEditorExperimentalExitMode => {
                Message::NoteEditor(note_editor::Message::ExitMode)
            }
            Command::NoteEditorExperimentalCursorWordForward => {
                Message::NoteEditor(note_editor::Message::CursorWordForward)
            }
            Command::NoteEditorExperimentalCursorWordBackward => {
                Message::NoteEditor(note_editor::Message::CursorWordBackward)
            }
            Command::NoteEditorExperimentalCursorLeft => {
                Message::NoteEditor(note_editor::Message::CursorLeft)
            }
            Command::NoteEditorExperimentalCursorRight => {
                Message::NoteEditor(note_editor::Message::CursorRight)
            }
            Command::VaultSelectorModalClose => {
                Message::VaultSelectorModal(vault_selector_modal::Message::Close)
            }
            Command::VaultSelectorModalToggle => {
                Message::VaultSelectorModal(vault_selector_modal::Message::Toggle)
            }
            Command::VaultSelectorModalUp => {
                Message::VaultSelectorModal(vault_selector_modal::Message::Up)
            }
            Command::VaultSelectorModalDown => {
                Message::VaultSelectorModal(vault_selector_modal::Message::Down)
            }
            Command::VaultSelectorModalOpen => {
                Message::VaultSelectorModal(vault_selector_modal::Message::Select)
            }
        }
    }
}
