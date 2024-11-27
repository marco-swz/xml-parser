{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = {nixpkgs, flake-utils, ... }: 
    flake-utils.lib.eachSystem flake-utils.lib.allSystems (system:
    let
        pkgs = import nixpkgs {
            inherit system;
        };
    in {
        devShell = (pkgs.buildFHSUserEnv {
            name = "elixir-env";
            targetPkgs = pkgs: (with pkgs; [
                clang
                elixir
                next-ls
                xmlstarlet
            ]);
            profile = "export ELIXIR_LS=${pkgs.elixir-ls}/lib";
            runScript = "bash";
        }).env;
    });
}
