use basalt_core::obsidian::{Note, Vault};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect, Size},
    widgets::{StatefulWidget, StatefulWidgetRef},
    DefaultTerminal,
};

use std::{cell::RefCell, fmt::Debug, io::Result};

use crate::{
    config::{self, Config},
    explorer::{Explorer, ExplorerState},
    help_modal::{HelpModal, HelpModalState},
    note_editor::{Editor, EditorState, Mode},
    outline::{Outline, OutlineState},
    splash_modal::{SplashModal, SplashModalState},
    statusbar::{StatusBar, StatusBarState},
    stylized_text::{self, FontStyle},
    text_counts::{CharCount, WordCount},
    vault_selector_modal::{VaultSelectorModal, VaultSelectorModalState},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = include_str!("./help.txt");

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ScrollAmount {
    #[default]
    One,
    HalfPage,
}

fn calc_scroll_amount(scroll_amount: ScrollAmount, height: usize) -> usize {
    match scroll_amount {
        ScrollAmount::One => 1,
        ScrollAmount::HalfPage => height / 2,
    }
}

#[derive(Default, Clone)]
pub struct AppState<'a> {
    screen_size: Size,
    is_running: bool,

    active_pane: ActivePane,
    explorer: ExplorerState<'a>,
    note_editor: EditorState<'a>,
    outline: OutlineState,
    selected_note: Option<SelectedNote>,

    splash_modal: SplashModalState<'a>,
    help_modal: HelpModalState,
    vault_selector_modal: VaultSelectorModalState<'a>,
}

fn modal_area_height(size: Size) -> usize {
    let vertical = Layout::vertical([Constraint::Percentage(50)]).flex(Flex::Center);
    let [area] = vertical.areas(Rect::new(0, 0, size.width, size.height.saturating_sub(3)));
    area.height.into()
}

impl<'a> AppState<'a> {
    pub fn active_component(&self) -> ActivePane {
        if self.help_modal.visible {
            return ActivePane::HelpModal;
        }

        if self.vault_selector_modal.visible {
            return ActivePane::VaultSelectorModal;
        }

        if self.splash_modal.visible {
            return ActivePane::Splash;
        }

        self.active_pane
    }

    pub fn set_running(&self, is_running: bool) -> Self {
        Self {
            is_running,
            ..self.clone()
        }
    }
}

pub mod splash {
    use crate::splash_modal::SplashModalState;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Message {
        Up,
        Down,
        Open,
    }

    pub fn update(message: Message, state: SplashModalState) -> SplashModalState {
        match message {
            Message::Up => state.previous(),
            Message::Down => state.next(),
            Message::Open => state.select(),
        }
    }
}

pub mod explorer {
    use crate::explorer::ExplorerState;

    use super::ScrollAmount;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Message {
        Up,
        Down,
        Open,
        Sort,
        Toggle,
        ToggleOutline,
        SwitchPaneNext,
        SwitchPanePrevious,
        ScrollUp(ScrollAmount),
        ScrollDown(ScrollAmount),
    }

    pub fn update(message: Message, state: ExplorerState) -> ExplorerState {
        match message {
            Message::Up => state.previous(1),
            Message::Down => state.next(1),
            Message::Sort => state.sort(),
            Message::Open => state.select(),
            Message::Toggle => state.toggle(),
            Message::SwitchPaneNext | Message::SwitchPanePrevious => {
                if state.active {
                    state.set_active(false)
                } else {
                    state.set_active(true)
                }
            }
            _ => state,
        }
    }
}

pub mod outline {
    use crate::outline::OutlineState;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Message {
        Up,
        Down,
        Select,
        Expand,
        Toggle,
        ToggleExplorer,
        SwitchPaneNext,
        SwitchPanePrevious,
    }

    pub fn update(message: Message, state: OutlineState) -> OutlineState {
        match message {
            Message::Up => state.previous(1),
            Message::Down => state.next(1),
            Message::Toggle => state.toggle(),
            Message::SwitchPaneNext | Message::SwitchPanePrevious => {
                if state.active {
                    state.set_active(false)
                } else {
                    state.set_active(true)
                }
            }
            _ => state,
        }
    }
}

pub mod note_editor {
    use ratatui::crossterm::event::{KeyCode, KeyEvent};

    use super::ScrollAmount;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Message {
        Save,
        SwitchPaneNext,
        SwitchPanePrevious,
        ToggleExplorer,
        ToggleOutline,
        EditMode,
        ExitMode,
        ReadMode,
        KeyEvent(KeyEvent),
        CursorUp,
        CursorLeft,
        CursorRight,
        CursorWordForward,
        CursorWordBackward,
        CursorDown,
        ScrollUp(ScrollAmount),
        ScrollDown(ScrollAmount),
        Delete,
    }

    pub fn handle_editing_event(key: &KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Up => Some(Message::CursorUp),
            KeyCode::Down => Some(Message::CursorDown),
            KeyCode::Esc => Some(Message::ExitMode),
            KeyCode::Backspace => Some(Message::Delete),
            _ => Some(Message::KeyEvent(*key)),
        }
    }
}

pub mod help_modal {
    use crate::help_modal::HelpModalState;

    use super::ScrollAmount;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Message {
        Toggle,
        Close,
        ScrollUp(ScrollAmount),
        ScrollDown(ScrollAmount),
    }

    pub fn update(message: Message, state: HelpModalState) -> HelpModalState {
        match message {
            Message::Toggle => state.toggle_visibility(),
            Message::Close => state.hide(),
            _ => state,
        }
    }
}

pub mod vault_selector_modal {
    use crate::vault_selector_modal::VaultSelectorModalState;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Message {
        Toggle,
        Up,
        Down,
        Select,
        Close,
    }

    pub fn update(message: Message, state: VaultSelectorModalState) -> VaultSelectorModalState {
        match message {
            Message::Up => state.previous(),
            Message::Down => state.next(),
            Message::Toggle => state.toggle_visibility(),
            Message::Select => state.select(),
            Message::Close => state.hide(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    Quit,
    Resize(Size),

    Splash(splash::Message),
    Explorer(explorer::Message),
    NoteEditor(note_editor::Message),
    Outline(outline::Message),
    HelpModal(help_modal::Message),
    VaultSelectorModal(vault_selector_modal::Message),
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ActivePane {
    #[default]
    Splash,
    Explorer,
    NoteEditor,
    Outline,
    HelpModal,
    VaultSelectorModal,
}

impl From<ActivePane> for &str {
    fn from(value: ActivePane) -> Self {
        match value {
            ActivePane::Splash => "Splash",
            ActivePane::Explorer => "Explorer",
            ActivePane::NoteEditor => "Note Editor",
            ActivePane::Outline => "Outline",
            ActivePane::HelpModal => "Help",
            ActivePane::VaultSelectorModal => "Vault Selector",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SelectedNote {
    name: String,
    path: String,
    content: String,
}

impl From<Note> for SelectedNote {
    fn from(value: Note) -> Self {
        Self {
            name: value.name.clone(),
            path: value.path.to_string_lossy().to_string(),
            content: Note::read_to_string(&value).unwrap_or_default(),
        }
    }
}

fn help_text(version: &str) -> String {
    HELP_TEXT.replace("%version-notice", version)
}

pub struct App<'a> {
    state: AppState<'a>,
    config: Config,
    terminal: RefCell<DefaultTerminal>,
}

impl<'a> App<'a> {
    pub fn new(state: AppState<'a>, terminal: DefaultTerminal) -> Self {
        Self {
            state,
            // TODO: Surface toast if read config returns error
            config: config::load().unwrap(),
            terminal: RefCell::new(terminal),
        }
    }

    pub fn start(terminal: DefaultTerminal, vaults: Vec<&Vault>) -> Result<()> {
        let version = stylized_text::stylize(&format!("{VERSION}~beta"), FontStyle::Script);
        let size = terminal.size()?;

        let state = AppState {
            screen_size: size,
            help_modal: HelpModalState::new(&help_text(&version)),
            vault_selector_modal: VaultSelectorModalState::new(vaults.clone()),
            splash_modal: SplashModalState::new(&version, vaults, true),
            ..Default::default()
        };

        App::new(state, terminal).run()
    }

    fn run(&'a mut self) -> Result<()> {
        self.state.is_running = true;

        while self.state.is_running {
            self.draw(&mut self.state.clone())?;
            let event = event::read()?;
            let action = self.handle_event(&event);
            self.state = self.update(&self.state, action);
        }

        Ok(())
    }

    fn draw(&self, state: &'a mut AppState<'a>) -> Result<()> {
        let mut terminal = self.terminal.borrow_mut();

        terminal.draw(move |frame| {
            let area = frame.area();
            let buf = frame.buffer_mut();
            self.render_ref(area, buf, state);
        })?;

        Ok(())
    }

    fn handle_event(&self, event: &Event) -> Option<Message> {
        match event {
            Event::Resize(cols, rows) => Some(Message::Resize(Size::new(*cols, *rows))),
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => None,
        }
    }

    #[rustfmt::skip]
    fn handle_active_component_event(&self, key: &KeyEvent, active_component: ActivePane) -> Option<Message> {
        match active_component {
            ActivePane::Splash => self.config.splash.key_to_message(key.into()),
            ActivePane::Explorer => self.config.explorer.key_to_message(key.into()),
            ActivePane::NoteEditor => {
                    if self.state.note_editor.is_editing() {
                        note_editor::handle_editing_event(key).map(Message::NoteEditor)
                    } else {
                        self.config.note_editor.key_to_message(key.into())
                }
            },
            ActivePane::Outline => self.config.outline.key_to_message(key.into()),
            ActivePane::HelpModal => self.config.help_modal.key_to_message(key.into()),
            ActivePane::VaultSelectorModal => self.config.vault_selector_modal.key_to_message(key.into()),
        }
    }

    fn handle_key_event(&self, key: &KeyEvent) -> Option<Message> {
        let global_message = self.config.global.key_to_message(key.into());

        let is_editing = self.state.note_editor.is_editing();

        if global_message.is_some() && !is_editing {
            return global_message;
        }

        let active_component = self.state.active_component();
        self.handle_active_component_event(key, active_component)
    }

    fn update(&self, state: &AppState<'a>, message: Option<Message>) -> AppState<'a> {
        let state = state.clone();
        let Some(message) = message else {
            return state;
        };

        match message {
            Message::Quit => state.set_running(false),
            Message::Resize(size) => AppState {
                screen_size: size,
                ..state
            },
            Message::HelpModal(message) => {
                let help_modal = help_modal::update(message.clone(), state.help_modal.clone());

                match message {
                    help_modal::Message::ScrollDown(scroll_amount) => AppState {
                        help_modal: help_modal.scroll_down(calc_scroll_amount(
                            scroll_amount,
                            modal_area_height(state.screen_size),
                        )),
                        ..state
                    },
                    help_modal::Message::ScrollUp(scroll_amount) => AppState {
                        help_modal: help_modal.scroll_up(calc_scroll_amount(
                            scroll_amount,
                            modal_area_height(state.screen_size),
                        )),
                        ..state
                    },
                    _ => AppState {
                        help_modal,
                        ..state
                    },
                }
            }
            Message::VaultSelectorModal(message) => {
                let vault_selector_modal = vault_selector_modal::update(
                    message.clone(),
                    state.vault_selector_modal.clone(),
                );

                match message {
                    vault_selector_modal::Message::Select => vault_selector_modal
                        .selected()
                        .and_then(|index| vault_selector_modal.clone().get_item(index))
                        .map(|vault| AppState {
                            active_pane: ActivePane::Explorer,
                            explorer: ExplorerState::new(&vault.name, vault.entries())
                                .set_active(true),
                            ..Default::default()
                        })
                        .unwrap_or(state),
                    _ => AppState {
                        vault_selector_modal,
                        ..state
                    },
                }
            }
            Message::Splash(message) => {
                let splash_modal = splash::update(message.clone(), state.splash_modal.clone());

                match message {
                    splash::Message::Open => splash_modal
                        .selected()
                        .and_then(|index| splash_modal.clone().get_item(index))
                        .map(|vault| AppState {
                            active_pane: ActivePane::Explorer,
                            explorer: ExplorerState::new(&vault.name, vault.entries())
                                .set_active(true),
                            splash_modal: SplashModalState::default(),
                            ..state.clone()
                        })
                        .unwrap_or(state),
                    _ => AppState {
                        splash_modal,
                        ..state
                    },
                }
            }
            Message::Explorer(message) => {
                let explorer = explorer::update(message.clone(), state.explorer.clone());

                match message {
                    explorer::Message::SwitchPaneNext => AppState {
                        active_pane: ActivePane::NoteEditor,
                        note_editor: state.note_editor.set_active(true),
                        explorer,
                        ..state
                    },
                    explorer::Message::SwitchPanePrevious => AppState {
                        active_pane: ActivePane::Outline,
                        outline: state.outline.set_active(true),
                        explorer,
                        ..state
                    },
                    explorer::Message::ScrollUp(scroll_amount) => AppState {
                        explorer: explorer.previous(calc_scroll_amount(
                            scroll_amount,
                            state.screen_size.height.into(),
                        )),
                        ..state
                    },
                    explorer::Message::ScrollDown(scroll_amount) => AppState {
                        explorer: explorer.next(calc_scroll_amount(
                            scroll_amount,
                            state.screen_size.height.into(),
                        )),
                        ..state
                    },
                    explorer::Message::Toggle => match explorer.open {
                        true => AppState { explorer, ..state },
                        false => AppState {
                            active_pane: ActivePane::NoteEditor,
                            explorer: explorer.set_active(false),
                            note_editor: state.note_editor.set_active(true),
                            ..state
                        },
                    },
                    explorer::Message::ToggleOutline => AppState {
                        outline: state.outline.toggle(),
                        ..state
                    },
                    explorer::Message::Open => {
                        let selected_note = explorer.selected_note.clone().map(SelectedNote::from);

                        let note_editor = selected_note
                            .clone()
                            .map(|note| {
                                EditorState::default()
                                    .set_mode(if self.config.experimental_editor {
                                        state.note_editor.mode
                                    } else {
                                        Mode::Read
                                    })
                                    .set_content(&note.content)
                                    .set_path(note.path.into())
                            })
                            .unwrap_or_default();

                        let outline = OutlineState::new(
                            note_editor.nodes(),
                            note_editor.current_row,
                            state.outline.is_open(),
                        );

                        AppState {
                            explorer,
                            outline,
                            note_editor,
                            selected_note,
                            ..state
                        }
                    }
                    _ => AppState { explorer, ..state },
                }
            }
            Message::Outline(message) => {
                let outline = outline::update(message.clone(), state.outline.clone());

                match message {
                    outline::Message::SwitchPaneNext => AppState {
                        active_pane: ActivePane::Explorer,
                        explorer: state.explorer.set_active(true),
                        outline,
                        ..state
                    },
                    outline::Message::SwitchPanePrevious => AppState {
                        active_pane: ActivePane::NoteEditor,
                        note_editor: state.note_editor.set_active(true),
                        outline,
                        ..state
                    },
                    outline::Message::Toggle => match outline.open {
                        true => AppState { outline, ..state },
                        false => AppState {
                            active_pane: ActivePane::NoteEditor,
                            outline: outline.set_active(false),
                            note_editor: state.note_editor.set_active(true),
                            ..state
                        },
                    },
                    outline::Message::Expand => AppState {
                        outline: state.outline.toggle_item(),
                        ..state
                    },
                    outline::Message::Select => AppState {
                        note_editor: state.note_editor.set_row(
                            outline
                                .selected()
                                .map(|item| item.get_range().start)
                                .unwrap_or_default(),
                        ),
                        ..state
                    },

                    _ => AppState { outline, ..state },
                }
            }
            Message::NoteEditor(message) => {
                let mode = &state.note_editor.mode();

                let editor_enabled = self.config.experimental_editor;

                if editor_enabled {
                    match message {
                        note_editor::Message::KeyEvent(key) if *mode == Mode::Edit => {
                            let note_editor = state.note_editor.edit(key.into());
                            let selected_note = state.selected_note.map(|note| SelectedNote {
                                content: note_editor.content().to_string(),
                                ..note
                            });

                            return AppState {
                                note_editor,
                                selected_note,
                                ..state
                            };
                        }
                        note_editor::Message::CursorLeft => {
                            return AppState {
                                note_editor: state.note_editor.cursor_left(),
                                ..state
                            }
                        }
                        note_editor::Message::CursorRight => {
                            return AppState {
                                note_editor: state.note_editor.cursor_right(),
                                ..state
                            }
                        }
                        note_editor::Message::CursorWordForward => {
                            return AppState {
                                note_editor: state.note_editor.cursor_word_forward(),
                                ..state
                            }
                        }
                        note_editor::Message::CursorWordBackward => {
                            return AppState {
                                note_editor: state.note_editor.cursor_word_backward(),
                                ..state
                            }
                        }
                        note_editor::Message::Delete => {
                            return AppState {
                                note_editor: state.note_editor.delete_char(),
                                ..state
                            }
                        }
                        note_editor::Message::EditMode if *mode != Mode::Edit => {
                            if let Some(selected_note) = &state.selected_note {
                                return AppState {
                                    active_pane: ActivePane::NoteEditor,
                                    note_editor: state
                                        .note_editor
                                        .clone()
                                        .set_content(&selected_note.content)
                                        .set_mode(Mode::Edit),
                                    ..state
                                };
                            } else {
                                return state;
                            }
                        }
                        note_editor::Message::ReadMode if *mode != Mode::Read => {
                            return AppState {
                                note_editor: state.note_editor.set_mode(Mode::Read),
                                ..state
                            }
                        }
                        note_editor::Message::ExitMode if *mode == Mode::Read => {
                            return AppState {
                                note_editor: state.note_editor.set_mode(Mode::View),
                                ..state
                            }
                        }
                        note_editor::Message::ExitMode if *mode == Mode::Edit => {
                            let note_editor = state.note_editor.exit_insert();
                            let outline = state.outline.set_nodes(note_editor.nodes());

                            let selected_note = state
                                .selected_note
                                .map(|note| SelectedNote {
                                    content: note_editor.content().to_string(),
                                    ..note
                                })
                                .clone();

                            return AppState {
                                note_editor: note_editor.set_mode(Mode::View),
                                outline,
                                selected_note,
                                ..state
                            };
                        }
                        note_editor::Message::Save => {
                            let note_editor = state.note_editor.save();
                            let selected_note = state.selected_note.map(|note| SelectedNote {
                                content: note_editor.content().to_string(),
                                ..note
                            });

                            return AppState {
                                selected_note,
                                note_editor,
                                ..state
                            };
                        }
                        _ => {}
                    }
                }

                match message {
                    note_editor::Message::CursorUp => {
                        let note_editor = state.note_editor.cursor_up();
                        let outline = state.outline.select_at(note_editor.current_row);

                        AppState {
                            note_editor,
                            outline,
                            ..state
                        }
                    }
                    note_editor::Message::CursorDown => {
                        let note_editor = state.note_editor.cursor_down();
                        let outline = state.outline.select_at(note_editor.current_row);

                        AppState {
                            note_editor,
                            outline,
                            ..state
                        }
                    }
                    note_editor::Message::ScrollUp(scroll_amount) if *mode != Mode::Edit => {
                        AppState {
                            note_editor: state.note_editor.scroll_up(calc_scroll_amount(
                                scroll_amount,
                                state.screen_size.height.into(),
                            )),
                            ..state
                        }
                    }
                    note_editor::Message::ScrollDown(scroll_amount) if *mode != Mode::Edit => {
                        AppState {
                            note_editor: state.note_editor.scroll_down(calc_scroll_amount(
                                scroll_amount,
                                state.screen_size.height.into(),
                            )),
                            ..state
                        }
                    }
                    note_editor::Message::ToggleExplorer if *mode != Mode::Edit => {
                        match state.explorer.open {
                            true => AppState {
                                explorer: state.explorer.toggle(),
                                ..state
                            },
                            false => AppState {
                                active_pane: ActivePane::Explorer,
                                explorer: state.explorer.toggle().set_active(true),
                                note_editor: state.note_editor.set_active(false),
                                ..state
                            },
                        }
                    }
                    note_editor::Message::ToggleOutline if *mode != Mode::Edit => {
                        match state.outline.open {
                            true => AppState {
                                outline: state.outline.toggle(),
                                ..state
                            },
                            false => AppState {
                                active_pane: ActivePane::Outline,
                                outline: state.outline.toggle().set_active(true),
                                note_editor: state.note_editor.set_active(false),
                                ..state
                            },
                        }
                    }
                    note_editor::Message::SwitchPaneNext => AppState {
                        active_pane: ActivePane::Outline,
                        note_editor: state.note_editor.set_active(false),
                        outline: state.outline.set_active(true),
                        ..state
                    },
                    note_editor::Message::SwitchPanePrevious => AppState {
                        active_pane: ActivePane::Explorer,
                        note_editor: state.note_editor.set_active(false),
                        explorer: state.explorer.set_active(true),
                        ..state
                    },
                    note_editor::Message::ScrollUp(_) if *mode == Mode::Edit => AppState {
                        note_editor: state.note_editor.cursor_up(),
                        ..state
                    },
                    note_editor::Message::ScrollDown(_) if *mode == Mode::Edit => AppState {
                        note_editor: state.note_editor.cursor_down(),
                        ..state
                    },
                    _ => state,
                }
            }
        }
    }

    fn render_splash(&self, area: Rect, buf: &mut Buffer, state: &mut SplashModalState<'a>) {
        SplashModal::default().render_ref(area, buf, state)
    }

    fn render_main(&self, area: Rect, buf: &mut Buffer, state: &mut AppState<'a>) {
        let [content, statusbar] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
            .horizontal_margin(1)
            .areas(area);

        let (left, right) = if state.explorer.open {
            (Constraint::Length(35), Constraint::Fill(1))
        } else {
            (Constraint::Length(5), Constraint::Fill(1))
        };

        let [explorer_pane, note, outline] = Layout::horizontal([
            left,
            right,
            if state.outline.is_open() {
                Constraint::Length(35)
            } else {
                Constraint::Length(4)
            },
        ])
        .areas(content);

        Explorer::new().render(explorer_pane, buf, &mut state.explorer);
        Editor::default().render(note, buf, &mut state.note_editor);
        Outline.render(outline, buf, &mut state.outline);

        let (_, counts) = state
            .selected_note
            .clone()
            .map(|note| {
                let content = note.content.as_str();
                (
                    note.name,
                    (WordCount::from(content), CharCount::from(content)),
                )
            })
            .unzip();

        let (word_count, char_count) = counts.unwrap_or_default();

        let mut status_bar_state = StatusBarState::new(
            state.active_pane.into(),
            word_count.into(),
            char_count.into(),
        );

        let status_bar = StatusBar::default();
        status_bar.render_ref(statusbar, buf, &mut status_bar_state);

        self.render_modals(area, buf, state)
    }

    fn render_modals(&self, area: Rect, buf: &mut Buffer, state: &mut AppState<'a>) {
        if state.splash_modal.visible {
            self.render_splash(area, buf, &mut state.splash_modal);
        }

        if state.vault_selector_modal.visible {
            VaultSelectorModal::default().render(area, buf, &mut state.vault_selector_modal);
        }

        if state.help_modal.visible {
            HelpModal.render(area, buf, &mut state.help_modal);
        }
    }
}

impl<'a> StatefulWidgetRef for App<'a> {
    type State = AppState<'a>;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_main(area, buf, state);
    }
}
