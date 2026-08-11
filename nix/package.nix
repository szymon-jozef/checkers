{ metadata, ... }:
{
  perSystem = { pkgs, lib, ... }: {
    packages.default = (
      pkgs.rustPlatform.buildRustPackage {
        pname = metadata.package.name;
        version = metadata.package.version;

        src = lib.cleanSource ./..;

        cargoLock.lockFile = ./../Cargo.lock;

        # build deps
        nativeBuildInputs = with pkgs; [
          # cargo
          # rustc
          # already provided by buildRustPackage :p
          pkg-config
        ];

        # runtime deps
        buildInputs = with pkgs; [

        ];

        meta = {
          description = metadata.package.description;
          homepage = metadata.package.repository;
          downloadPage = "https://example.com";
          license = lib.licenses."${metadata.package.license}";
          # platforms = lib.platforms.linux;
          mainProgram = metadata.package.name;
        };
      }
    );
  };
}
