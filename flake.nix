{
  description = "Rust devshell from github:oxalica";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            android_sdk.accept_license = true;
            allowUnfree = true;
          };
        };

        androidComposition = pkgs.androidenv.composeAndroidPackages {
          platformVersions = [ "33" ];
          includeNDK = true;
        };

        rustWithTargets = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "aarch64-linux-android" ];
        };

        android-sdk = androidComposition.androidsdk;
        android-home = "${android-sdk}/libexec/android-sdk";
      in {
        devShells.default = with pkgs; mkShell {
          ANDROID_HOME = android-home;
          ANDROID_NDK_HOME="${android-home}/ndk/";
          ANDROID_NDK_ROOT="${android-home}/ndk/";

          packages = [
            rustWithTargets
            android-sdk
            cargo-ndk
          ];

          shellHook = ''
            echo "Entered Rust dev environment"
          '';
        };
      }
    );
}
