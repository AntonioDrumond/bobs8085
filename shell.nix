{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustc
    rustfmt
    cargo
    clippy
    gcc
    expat
    fontconfig
    freetype
    freetype.dev
    libGL
    pkg-config
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    wayland
    libxkbcommon
  ];

  LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;

  shellHook = ''
    echo ""
    echo "Packages loaded: gcc, cargo, rustc, rustfmt, clippy"
  '';

}
