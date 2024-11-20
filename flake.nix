{
  description = "Flake for tinymist package";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = { self, nixpkgs }: 
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
    in {
      packages.x86_64-linux.tinymist = pkgs.rustPlatform.buildRustPackage {
        name = "tinymist";
        pname = "tinymist";
        version = "0.1.0";

        src = ./.;

        nativeBuildInputs = [
          pkgs.openssl.dev # For OpenSSL headers
          pkgs.pkg-config  # For pkg-config tool
        ];

        buildInputs = [
          pkgs.openssl.out # For OpenSSL libraries
        ];

        cargoLock.lockFile = ./Cargo.lock;
				cargoLock.outputHashes = {
					"typst-0.12.0" = "sha256-E2wSVHqY3SymCwKgbLsASJYaWfrbF8acH15B2STEBF8=";
					"typst-syntax-0.7.0" = "sha256-yrtOmlFAKOqAmhCP7n0HQCOQpU3DWyms5foCdUb9QTg=";
					"typstfmt_lib-0.2.7" = "sha256-LBYsTCjZ+U+lgd7Z3H1sBcWwseoHsuepPd66bWgfvhI=";
				};

        meta = with pkgs.lib; {
          description = "Tinymist tool from GitHub";
          homepage = "https://github.com/Myriad-Dreamin/tinymist";
          license = licenses.asl20;
				};
      };
    };
}
