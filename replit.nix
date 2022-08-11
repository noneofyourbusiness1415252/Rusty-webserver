{ pkgs }: {
	deps = with pkgs; [
		nodejs-18_x
        nodePackages.typescript-language-server
        yarn
        replitPackages.jest
		cargo
		rustc
		rustfmt
		rust-analyzer
	];
}