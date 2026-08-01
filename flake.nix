{
  description = "A convenient way to build std::io::Readers and std::io::Writers by chaining transformations";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, fenix, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        
        # Updated with the correct hash for the 1.96.0 manifest
        rustToolchain = fenix.packages.${system}.toolchainOf {
          channel = "1.96.0";
          sha256 = "sha256-mvUGEOHYJpn3ikC5hckneuGixaC+yGrkMM/liDIDgoU=";
        };

        naersk-lib = pkgs.callPackage naersk {
          cargo = rustToolchain.cargo;
          rustc = rustToolchain.rustc;
        };
      in
      {
        # build with: nix build
        packages.default = naersk-lib.buildPackage {
          src = ./.;
          # Enables all features including wincode, chacha20, salsa20, and flate2
          cargoBuildOptions = x: x ++ [ "--all-features" ];
          buildInputs = with pkgs; [ pkg-config openssl ];
        };

        # activate with: nix develop
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            (fenix.packages.${system}.combine [
              rustToolchain.rustc
              rustToolchain.cargo
              rustToolchain.rustfmt
              rustToolchain.clippy
              rustToolchain.rust-src
              rustToolchain.llvm-tools-preview
            ])
            pkg-config
            openssl
            # Quality control tools from your development workflow
            grcov
            cargo-all-features
            cargo-deadlinks
          ];

          shellHook = ''
            echo "🦀 rw-builder development environment (Rust 1.96.0) loaded"
            echo "Quality tools available: grcov, cargo-all-features, clippy"
          '';
        };
      });
}