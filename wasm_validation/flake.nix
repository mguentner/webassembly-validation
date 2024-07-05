 {
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";   
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, rust-overlay }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {  inherit system; overlays = [ rust-overlay.overlays.default ]; };
    
    rustc-wasm = pkgs.rust-bin.stable.latest.default.override {
      targets = [ "wasm32-unknown-unknown" ];
      extensions = [ "rust-src" "rust-analyzer" "clippy" ];
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      name = "default";
      buildInputs = with pkgs; [
        wasm-pack cargo llvmPackages.bintools rustc-wasm
      ];
    };
  };
}
