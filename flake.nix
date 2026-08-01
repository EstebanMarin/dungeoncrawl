{
  description = "second (bracket-lib: freetype + OpenGL/X11/Wayland)";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      # dlopen'd at runtime by the OpenGL windowing backend
      runtimeLibs = with pkgs; [
        libGL
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        libxkbcommon
        wayland
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          cmake       # freetype-sys vendored-build fallback
          pkg-config
        ];

        # system libs via pkg-config let the *-sys crates skip vendored cmake builds
        buildInputs = with pkgs; [ freetype expat fontconfig ] ++ runtimeLibs;

        shellHook = ''
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeLibs}:''${LD_LIBRARY_PATH:-}"
          # fallback for any crate whose vendored CMakeLists still asks for cmake < 3.5
          export CMAKE_POLICY_VERSION_MINIMUM=3.5
          # bracket-lib's winit 0.27 closes its window instantly on Wayland;
          # force the X11/XWayland backend so the window stays open.
          export WINIT_UNIX_BACKEND=x11
        '';
      };
    };
}
