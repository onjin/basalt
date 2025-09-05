<img align="left" width="125px" src="https://raw.githubusercontent.com/erikjuhani/basalt/refs/heads/main/assets/basalt.png?raw=true"><h3>Basalt&nbsp;&nbsp;</h3>
<p>TUI Application to manage Obsidian notes&nbsp;&nbsp;&nbsp;&nbsp;</p>

<hr>

TUI Application to manage Obsidian vaults and notes directly from the terminal ✨.

![Demo](https://raw.githubusercontent.com/erikjuhani/basalt/refs/heads/main/assets/basalt_demo.gif)

Basalt is a TUI (Terminal User Interface) application to manage Obsidian vaults and notes from the terminal. Basalt is cross-platform and can be installed and run in the major operating systems on Windows, macOS; and Linux.

Basalt is not a complete or comprehensive replacement for Obsidian, but instead a minimalist approach for note management in terminal with a readable markdown rendering and [WYSIWYG](https://en.wikipedia.org/wiki/WYSIWYG) experience.

## Vision

- Basalt functions as a companion app for Obsidian that enables quick note editing without interrupting the terminal flow
- Basalt enables text editing in a familiar way (Obsidian, vim) without having to rely on external editors
- Basalt is a terminal based [WYSIWYG](https://en.wikipedia.org/wiki/WYSIWYG) markdown editor
- Basalt works as a CLI for finding / deleting / creating notes and works with the rest of the unix tooling
- Basalt is a standalone terminal note managing application that works seamlessly with Obsidian

## Installation

Install basalt using cargo:

```sh
cargo install basalt-tui
```

Or use the precompiled binaries from the latest basalt release.

Run from `nix` manager:

```sh
nix run github:erikjuhani/basalt
```

## Background

This is something that has been brewing in my head for quite some time. There has been different incarnations over the years, however, nothing as substantial as this.

I have been using Neovim and the official Obsidian app. However, I wanted to have something dedicated that offers the same writing experience as Neovim, but has more WYSIWYG experience as in the official Obsidian app. I'm fully aware of [obsidian.nvim](https://github.com/epwalsh/obsidian.nvim), which many people use and find more than sufficient. However, I want to see images, beautified text, note graphs, etc. I want it to be a bit more.

The problem for me personally is that when I leave the terminal, my flow breaks, especially if I'm writing. Using an entirely different app disrupts that flow, and it _annoys_ me. So here I am, building a TUI for Obsidian.

The goal of basalt is not to replace the Obsidian app. Basalt is to fill and cater a need to have a terminal view to the selection of notes and vaults, providing quick access from anywhere in the terminal with a simple command.

## Configuration

[[Basalt]] features and functionality can be customized using a user-defined configuration file. The configuration file should be located in one of the following directories:

**macOS and Unix:**

- `$HOME/.basalt.toml`
- `$XDG_CONFIG_HOME/basalt/config.toml`

**Windows:**

- `%USERPROFILE%\.basalt.toml`
- `%APPDATA%\basalt\config.toml`

If configuration files exist in multiple locations, only the first one found will be used, with the home directory configuration taking precedence. 

> [!WARNING]
>
> This behavior may change in future versions to merge all found configurations instead.

### Key Mappings

Basalt key mappings can be modified or extended by defining key mappings in the user configuration file.

Each key mapping is associated with a specific 'pane' and becomes active when that pane has focus. The global section applies to all panes and is evaluated first.

### Default configuration

```toml
# The corresponding pane needs to be _active_ in order for the keybindings to
# be read and the attached command activated.
#
# Global commands:
#
# quit: exits the application
# vault_selector_modal_toggle: toggles vault selector modal (not available in splash screen)
# help_modal_toggle: toggles help modal
# spawn: <command> spawns a new process without blocking. This is for opening external applications or URLs.
# exec: <command> runs a command in the current shell environment.
#
# Splash commands:
#
# splash_up: moves selector up
# splash_down: moves selector down
# splash_open: opens the selected vault
#
# Explorer commands:
#
# explorer_up: moves selector up
# explorer_down: moves selector down
# explorer_open: opens the selected note in note viewer
# explorer_sort: toggles note and folder sorting between A-z and Z-a 
# explorer_toggle: toggles explorer pane
# explorer_toggle_outline: toggles outline pane
# explorer_switch_pane_next: switches focus to next pane
# explorer_switch_pane_previous: switches focus to previous pane
# explorer_scroll_up_one: scrolls the selector up by one
# explorer_scroll_down_one: scrolls the selector down by one
# explorer_scroll_up_half_page: scrolls the selector up half a page
# explorer_scroll_down_half_page: scrolls the selector down half a page
#
# Outline commands:
#
# outline_up: moves selector up
# outline_down: moves selector down
# outline_toggle: toggles outline panel
# outline_toggle_explorer: toggles explorer pane
# outline_switch_pane_next: switches focus to next pane
# outline_switch_pane_previous: switches focus to previous pane
# outline_expand: expands or collapses headings
# outline_select: select heading and move note editor cursor to heading location
#
# Note editor commands:
#
# note_editor_scroll_up_one: scrolls up by one
# note_editor_scroll_down_one: scrolls down by one
# note_editor_scroll_up_half_page: scrolls up by half page
# note_editor_scroll_down_half_page: scrolls down by half page
# note_editor_toggle_explorer: toggles explorer pane
# note_editor_toggle_outline: toggles outline pane
# note_editor_switch_pane_next: switches focus to next pane
# note_editor_switch_pane_previous: switches focus to previous pane
#
# Help modal commands:
#
# help_modal_toggle: toggles help modal
# help_modal_close: closes help modal
# help_modal_scroll_up_one: scrolls up by one
# help_modal_scroll_down_one: scrolls down by one
# help_modal_scroll_up_half_page: scrolls up by half page
# help_modal_scroll_down_half_page: scrolls down by half page
#
# Vault selector modal commands:
#
# vault_selector_modal_up: moves selector up
# vault_selector_modal_down: moves selector down
# vault_selector_modal_close: closes vault selector modal
# vault_selector_modal_open: opens the selected vault 
# vault_selector_modal_toggle: toggles vault selector modal

# Editor is experimental
experimental_editor = false

[global]
key_bindings = [
 { key = "q", command = "quit" },
 { key = "ctrl+g", command = "vault_selector_modal_toggle" },
 { key = "?", command = "help_modal_toggle" },
 { key = "ctrl+alt+e", command = "exec:vi %note_path" },
 { key = "ctrl+alt+o", command = "spawn:open obsidian://open?vault=%vault&file=%note" },
]

[splash]
key_bindings = [
 { key = "k", command = "splash_up" },
 { key = "j", command = "splash_down" },
 { key = "up", command = "splash_up" },
 { key = "down", command = "splash_down" },
 { key = "enter", command = "splash_open" },
]

[explorer]
key_bindings = [
 { key = "k", command = "explorer_up" },
 { key = "j", command = "explorer_down" },
 { key = "up", command = "explorer_up" },
 { key = "down", command = "explorer_down" },
 { key = "t", command = "explorer_toggle" },
 { key = "s", command = "explorer_sort" },
 { key = "tab", command = "explorer_switch_pane_next" },
 { key = "shift+backtab", command = "explorer_switch_pane_previous" },
 { key = "enter", command = "explorer_open" },
 { key = "ctrl+b", command = "explorer_toggle" },
 { key = "ctrl+u", command = "explorer_scroll_up_half_page" },
 { key = "ctrl+d", command = "explorer_scroll_down_half_page" },
 { key = "ctrl+o", command = "explorer_toggle_outline" },
]

[outline]
key_bindings = [
 { key = "k", command = "outline_up" },
 { key = "j", command = "outline_down" },
 { key = "up", command = "outline_up" },
 { key = "down", command = "outline_down" },
 { key = "ctrl+o", command = "outline_toggle" },
 { key = "ctrl+b", command = "outline_toggle_explorer" },
 { key = "t", command = "outline_toggle_explorer" },
 { key = "tab", command = "outline_switch_pane_next" },
 { key = "shift+backtab", command = "outline_switch_pane_previous" },
 { key = "enter", command = "outline_expand" },
 { key = "g", command = "outline_select" },
]

[note_editor]
key_bindings = [
 { key = "k", command = "note_editor_cursor_up" },
 { key = "j", command = "note_editor_cursor_down" },
 { key = "up", command = "note_editor_cursor_up" },
 { key = "down", command = "note_editor_cursor_down" },
 { key = "t", command = "note_editor_toggle_explorer" },
 { key = "tab", command = "note_editor_switch_pane_next" },
 { key = "shift+backtab", command = "note_editor_switch_pane_previous" },
 { key = "ctrl+b", command = "note_editor_toggle_explorer" },
 { key = "ctrl+u", command = "note_editor_scroll_up_half_page" },
 { key = "ctrl+d", command = "note_editor_scroll_down_half_page" },
 { key = "ctrl+o", command = "note_editor_toggle_outline" },

 # Experimental editor 
 { key = "i", command = "note_editor_experimental_set_edit_mode" },
 { key = "shift+r", command = "note_editor_experimental_set_read_mode" },
 { key = "ctrl+x", command = "note_editor_experimental_save" },
 { key = "esc", command = "note_editor_experimental_exit_mode" },
 { key = "h", command = "note_editor_experimental_cursor_left" },
 { key = "l", command = "note_editor_experimental_cursor_right" },
 { key = "left", command = "note_editor_experimental_cursor_left" },
 { key = "right", command = "note_editor_experimental_cursor_right" },
 # 'f' translates to arrow key right
 { key = "alt+f", command = "note_editor_experimental_cursor_word_forward" },
 # 'b' translates to arrow key left
 { key = "alt+b", command = "note_editor_experimental_cursor_word_backward" },
]

[help_modal]
key_bindings = [
 { key = "esc", command = "help_modal_close" },
 { key = "k", command = "help_modal_scroll_up_one" },
 { key = "j", command = "help_modal_scroll_down_one" },
 { key = "up", command = "help_modal_scroll_up_one" },
 { key = "down", command = "help_modal_scroll_down_one" },
 { key = "ctrl+u", command = "help_modal_scroll_up_half_page" },
 { key = "ctrl+d", command = "help_modal_scroll_down_half_page" },
]

[vault_selector_modal]
key_bindings = [
 { key = "k", command = "vault_selector_modal_up" },
 { key = "j", command = "vault_selector_modal_down" },
 { key = "up", command = "vault_selector_modal_up" },
 { key = "down", command = "vault_selector_modal_down" },
 { key = "enter", command = "vault_selector_modal_open" },
 { key = "esc", command = "vault_selector_modal_close" },
]
```

## Contributing to Basalt

[[Basalt]] is ***open for code contributions***, but _primarily_ for bug fixes. Why? Feature work can bring long-term maintenance overhead, and I'd like to keep that to a minimum. One big reason for limiting feature work is that I want to build features myself, as this is a _fun_ side project alongside work, and I would like to keep it that way—to an extent.

However, I do realize that open source projects usually flourish with multiple contributors. Thus, I won't say no if you would like to contribute feature work, but please open an issue first so we can discuss it. This way we can avoid unnecessary effort or bikeshedding over architectural or stylistic choices. I have my own opinions and ideas on how certain things should be written in this project.

> [!info]
> I want this project to feel low-barrier, so don't be discouraged from opening an issue, whether it's about existing features, ideas, or anything else!

### What you can do right now

#### Found a typo?

Open a PR directly with the correction!

#### Found a bug and know how to fix it?

Open a PR with the fix!

#### Found a bug but not sure how to fix it or don't want to do it yourself?

Open an issue with steps to reproduce!

#### Want to contribute a feature?

Open an issue first so we can chat about the feature work or claim an existing issue for yourself!

### How to make your contribution

1. Fork the `basalt` repository
2. Create a branch
3. Open a pull request against basalt's main branch with your changes
4. I'll review your pull request as soon as possible and either leave comments or merge it

If you find mistakes in the documentation or need simple code fixes, please go ahead and open a pull request with the changes!

### Git Pre-push Hook

There's a useful pre-push git hook under `scripts`, which you can enable by running the following command:

```sh
cp scripts/pre-push .git/hooks/
```

The script runs the same test commands as in the `test.yml` workflow.

### CI

> [!missing]
>
> This section is unfinished. It should explain roughly what is being run in the CI and what is required for CI to actually run on a PR opened from a fork.

---

_I will create proper contribution guidelines later, with more details on certain operational aspects of this project._
