{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
	let
	system = "x86_64-linux";
	pkgs = nixpkgs.legacyPackages.${system};
	in {
		devShells.${system}.c = pkgs.mkShell {
			buildInputs = [
				pkgs.gcc
				pkgs.gnumake
			];

			shellHook = ''
			'';
		};
  };
}
