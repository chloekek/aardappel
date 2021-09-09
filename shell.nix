{ nixpkgs ? import nix/nixpkgs }:

nixpkgs.mkShell {
    nativeBuildInputs = [
        nixpkgs.rust_1_52.packages.stable.cargo
    ];
}
