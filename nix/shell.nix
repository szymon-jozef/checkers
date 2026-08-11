{
  perSystem = { pkgs, config, ... }: {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [ ];

      inputsFrom = [
        (config.packages.default)
      ];
    };
  };
}
