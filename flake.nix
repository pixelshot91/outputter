{
  description = "Outputter: show a program output by splitting stdout and stderr";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        app = pkgs.rustPlatform.buildRustPackage {
          pname = "outputter";
          version = "0.0.1";
          src = ./.;
          # cargoBuildFlags = "-p app";

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.pkg-config ];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            rust-bin.stable.latest.default
          ];

          shellHook = ''
          '';
        };
        packages.default = app;
      }
    );
}
