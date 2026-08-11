{
  perSystem = { pkgs, config, ... }: {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [
        rustfmt
      ];

      inputsFrom = [
        (config.packages.default)
      ];
    };
  };
}
