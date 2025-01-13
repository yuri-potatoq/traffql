{
  description = "Rust Project Template.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, rust }:
    utils.lib.eachDefaultSystem
      (
        system:
        let
          project-name = "traffi-ql";
          rust-channel = "nightly";
          rust-version = "2024-06-07"; # 1.80.0
          rust-overlay = import rust;

          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay ];
          };

          rust-toolchain = pkgs.rust-bin."${rust-channel}"."${rust-version}".default.override {
            extensions = [
              "rust-std"
              "rust-src"
            ];
            targets = [ "wasm32-unknown-unknown" ];
          };

          rustPlatform = pkgs.makeRustPlatform {
            cargo = rust-toolchain;
            rustc = rust-toolchain;
          };
          
          sqlite = pkgs.sqlite;
        in
        rec {
          # `nix build`
          packages.default = rustPlatform.buildRustPackage {
            pname = "wwasm";
            version = "0.0.1";
            src = ./.;
            cargoHash = "sha256-XjvXQQUvWtA6HNZOlSBuGJW0uFUIT4CQqaalxG7JmGc=";            
            buildPhase = ''
              wasm-pack build --target=no-modules
            '';
            installPhase = ''
              ./build.sh
              cp -rf ./pkg $out/pkg
              cd $out && chromium --pack-extension=pkg --no-message-box
              rm -r pkg
            '';
            nativeBuildInputs = with pkgs; [
              wasm-pack
              wasm-bindgen-cli
              chromium
            ];
          };

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [
              rust-toolchain
              rust-analyzer
              wasm-pack
              perl
              openssl
            ];
          };
        }
      );
}
