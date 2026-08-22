{
  perSystem =
    {
      pkgs,
      config,
      ...
    }:
    {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          rustfmt
          clippy
        ];

        RUST_LOG = "debug";

        LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
          "${pkgs.libX11}/lib"
          "${pkgs.libXi}/lib"
          "${pkgs.libGL}/lib"
          "${pkgs.libxkbcommon}/lib"
        ];

        inputsFrom = [
          (config.packages.default)
        ];
      };
    };
}
