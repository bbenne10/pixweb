{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.05";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rustOverlay.url = "github:oxalica/rust-overlay";

    pre-commit = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.flake-utils.follows = "flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { nixpkgs, flake-utils, crane, rustOverlay, pre-commit, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        wasmTarget = "wasm32-unknown-unknown";
        overlays = [ (import rustOverlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustPkg = pkgs.rust-bin.stable.latest.default.override {
          targets = [ wasmTarget ];
        };
        craneLib = (crane.mkLib pkgs).overrideScope' (_final: _prev: {
          rustc = rustPkg;
          cargo = rustPkg;
          rustfmt = rustPkg;
        });
        check = import ./checks/default.nix {
          inherit system nixpkgs pre-commit rustPkg;
        };
        buildInputs = with pkgs; [
          rustPkg
          libiconv
          trunk
          wasm-bindgen-cli
          binaryen
        ];
        commonDepsArgs = {
          name = "pixweb-deps";
          src = ./.;
        };
      in rec {
        packages = rec {
          cargoArtifacts = craneLib.buildDepsOnly (commonDepsArgs);
          cargoArtifactsWasm = craneLib.buildDepsOnly
            (commonDepsArgs // { CARGO_BUILD_TARGET = wasmTarget; });
          ui = craneLib.buildPackage {
            name = "pixweb";
            version = "0.1";
            src = ./.;
            cargoArtifacts = cargoArtifactsWasm;
            cargoBuildCommand = "cargo build --workspace --release -p ui";
            CARGO_BUILD_TARGET = wasmTarget;
            postInstall = ''
              mkdir -p $out
              ui_hash=$(echo $out | cut -c12-43)
              substitute \
                ui/index.html \
                $out/index.html \
                --subst-var "ui_hash" \
                --replace "<!--" "" \
                --replace "-->" ""

              cp -r ui/assets/ $out

              ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
                --target=web \
                --out-dir=$out/ \
                --out-name=ui-$ui_hash \
                target/${wasmTarget}/release/ui.wasm \
                --no-typescript

              rm -r $out/bin
            '';
          };
          ui_dev_server = pkgs.writeShellApplication {
            name = "run_dev_server";
            runtimeInputs = buildInputs;
            text = ''
              printf ">> trunk serve index.html\n"
              ${pkgs.trunk}/bin/trunk serve index.html
            '';
          };
        };
        devShell = pkgs.mkShell {
          buildInputs = buildInputs
            ++ [ packages.ui_dev_server pkgs.rust-analyzer ];
          shellHook = ''
            # pre-commit
            ${check.shellHook}
          '';
        };
      });
}
