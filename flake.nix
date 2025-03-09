{
  description = "Dev environment for Plinth";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default =
        pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustup
            cargo
          ];
        };
    };
}
