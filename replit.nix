{ pkgs }: {
    deps = [
        # Compiler
        pkgs.rustc
        # Project / library manager
        pkgs.cargo
        # Formats your code
        pkgs.rustfmt
        # Lints your code
        pkgs.clippy

        # Allows Replit package manager to work
        # pkgs.cargo-edit
    ];
}
