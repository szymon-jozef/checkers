{
  perSystem = { pkgs, config, ... }: {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [
        rustfmt
      ];

      RUST_LOG = "debug";

      inputsFrom = [
        (config.packages.default)
      ];
    };
  };
}
