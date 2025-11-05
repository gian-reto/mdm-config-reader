{
  description = "MDM Config Reader";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};

        # Corepack derivation for pnpm support.
        corepack = pkgs.stdenv.mkDerivation {
          name = "corepack";
          buildInputs = [pkgs.nodejs_22];
          phases = ["installPhase"];
          installPhase = ''
            mkdir -p $out/bin
            corepack enable --install-directory=$out/bin
          '';
        };
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Node.js and package manager.
            nodejs_22
            corepack

            # Rust toolchain.
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer

            # Build essentials for NAPI-RS.
            pkg-config

            # Cross-compilation tools.
            llvmPackages_latest.llvm
            cargo-xwin
            cargo-zigbuild
          ];

          shellHook = ''
            export COREPACK_ENABLE_AUTO_PIN=0

            echo "Environment is ready:"
            echo ""
            echo "Node.js: $(node -v)"
            echo "pnpm: $(pnpm -v)"
            echo "Cargo: $(cargo --version)"
            echo "Rust: $(rustc --version)"
            echo "LLVM: $(llvm-config --version)"
          '';
        };
      }
    );
}
