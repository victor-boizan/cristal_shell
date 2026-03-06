{
	description = "cristal_shell";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
	};

	outputs = {self, nixpkgs}: let
		system = "x86_64-linux";
		pkgs = nixpkgs.legacyPackages.${system};

		buildInputs = with pkgs; [
			wayland
			libxkbcommon
			
			expat
			fontconfig
			freetype
			freetype.dev
			libGL
			xorg.libX11
			xorg.libXcursor
			xorg.libXrandr
			vulkan-loader
		];

		nativeBuildInputs = with pkgs; [
			pkg-config
			rustc
			cargo

			makeWrapper
		];

	in {
		packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
			name = "cristal_shell";
			src = ./.;
			buildInputs = buildInputs;
			nativeBuildInputs = nativeBuildInputs;
			cargoLock.lockFile = ./Cargo.lock;

			postInstall = ''
				wrapProgram $out/bin/cristal_shell \
					--prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs}
			'';
		};
		devShells.${system}.default = pkgs.mkShell {
			buildInputs = buildInputs ++ (with pkgs; [
				cargo
				rustc
				rustfmt
				clippy
				rust-analyzer
			]);

			nativeBuildInputs = nativeBuildInputs;

			env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
			LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
		};
	};
}
