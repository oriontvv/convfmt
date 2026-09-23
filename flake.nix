{
  description = "cli tool which can convert different formats";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      inherit (nixpkgs) lib;

      # name/version/description are kept in Cargo.toml only
      manifest = (lib.importTOML ./Cargo.toml).package;

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = f: lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: rec {
        convfmt = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          inherit (manifest) version;

          # just the crate itself: web/ is a separate workspace with its own lock file
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./src
            ];
          };

          cargoLock.lockFile = ./Cargo.lock;

          meta = {
            inherit (manifest) description homepage;
            license = lib.licenses.asl20;
            mainProgram = "convfmt";
          };
        };

        default = convfmt;
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
          ];
        };
      });

      # `nix flake check` builds the package and runs `cargo test` inside it
      checks = forAllSystems (pkgs: { inherit (self.packages.${pkgs.system}) convfmt; });
    };
}
