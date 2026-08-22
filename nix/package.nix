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
          makeWrapper
        ];

        # runtime deps
        buildInputs = with pkgs; [
          wayland
          wayland-protocols
          libGL
          alsa-lib

          libX11.out
          libXi.out
          libXcursor.out
          libXrandr.out
          libxkbcommon.out
          libxcb
          libXinerama
          libXxf86vm

          fontconfig
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
