Basalt works in all major desktop platforms, Windows, Linux; and macOS.

Currently, the recommended way to install `basalt` is to use `cargo`.

## Cargo

```
cargo install basalt-tui
```

## Pre-compiled binaries

Every release is pre-compiled into platform specific binaries and archive files. To install basalt using the pre-compiled binary you must first:

1. Download the appropriate target system and architecture archive file from [GitHub release assets](https://github.com/erikjuhani/basalt/releases)
2. Extract the archive file contents
3. Move the included basalt binary file to appropriate location where it can be located by the System PATH or run it directly by invoking the binary.

## Nix

```
nix run github:erikjuhani/basalt
```
