{ pkgs, rustPlatform, browser ? "chromium" }:
with pkgs;
let
  browser-tools = {
    chromium = {
      deps = [ chromium ];
      install-cmd = '' chromium --pack-extension=pkg --no-message-box '';
    };
  };
in
with browser-tools."${browser}"; rustPlatform.buildRustPackage {
  pname = "traffql";
  version = "0.0.1";
  src = ./.;
  cargoHash = "sha256-vzSfdH1jjKtj0K9Uxoon2ys44CBmHlvmC2ydH/b7+5o=";
  buildPhase = ''
    wasm-pack build --target=no-modules
  '';
  installPhase = ''
    ./build.sh
    cp -rf ./pkg $out/pkg
    cd $out && ${install-cmd}
    rm -r pkg
  '';
  nativeBuildInputs = [
    wasm-pack
    wasm-bindgen-cli
  ] ++ deps;
}
