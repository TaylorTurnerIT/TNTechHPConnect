{ pkgs, ... }:
{
  # System packages the CLI shells out to (or needs to build):
  # - gcc: linker for `cargo build` (rustc alone is not enough)
  # - openssh: ssh / scp clients used for every cluster operation
  # - sshpass: applies the keychain-stored password non-interactively
  # - dbus + pkg-config: headers for building the Secret Service backend
  #   (keyring sync-secret-service -> libdbus-sys)
  packages = with pkgs; [
    gcc
    openssh
    sshpass
    dbus
    pkg-config
  ];

  # Prefer the user's rustup toolchain when present, so cargo/rustc versions
  # stay pinned by rustup instead of nixpkgs.
  enterShell = ''
    for d in "$HOME"/.rustup/toolchains/*/bin; do
      if [ -d "$d" ]; then
        export PATH="$d:$PATH"
      fi
    done
  '';

  # `devenv test` verifies the environment provides everything.
  enterTest = ''
    command -v gcc
    command -v ssh
    command -v scp
    command -v sshpass
    command -v cargo
    pkg-config --exists dbus-1
  '';
}
