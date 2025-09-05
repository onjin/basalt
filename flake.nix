{
  description = "Basalt TUI application for Obsidian notes.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naerskLib = pkgs.callPackage naersk { };
        # Convenience helpers
        commonBuildInputs = [ ] ++ (if pkgs.stdenv.isDarwin then
          [ pkgs.libiconv ] # many Rust crates need this on macOS
        else
          [ pkgs.ncurses ]); # common TUI dep on Linux

        commonNativeInputs = [ pkgs.pkg-config ]
          ++ (if pkgs.stdenv.isDarwin then
            [ ]
          else [
            pkgs.openssl
            pkgs.openssl.dev
          ]); # openssl-sys, etc.
      in {
        # nix build .  /  nix run .
        packages.default = naerskLib.buildPackage {
          pname = "basalt-tui";
          src = ./.;

          # Carry typical native deps for crates like openssl-sys, cursive, tui, etc.
          nativeBuildInputs = commonNativeInputs;
          buildInputs = commonBuildInputs;

          # So `nix run` knows what to execute if used via apps.* or fallback
          meta.mainProgram = "basalt";
        };

        # nix run .
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/basalt";
        };

        # nix develop  (full Rust toolchain + common native deps)
        devShells.default = pkgs.mkShell {
          # Tooling for development
          packages = with pkgs;
            [ rustc cargo clippy rustfmt rust-analyzer pkg-config ]
            ++ commonNativeInputs ++ commonBuildInputs;

          # Optional: keep Cargo target dir inside the repo for reproducible builds
          # and faster rebuilds across shells
          CARGO_TARGET_DIR = ".cargo-target";

          shellHook = ''
            echo "# Basalt dev shell started:"
            echo " - rustc: $(rustc --version)"
            echo " - cargo:  $(cargo --version)"
          '';
        };
      });
}

