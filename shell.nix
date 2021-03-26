{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    openssl.out
    openssl.dev
  ];

  OPENSSL_DIR=openssl.dev;
  OPENSSL_LIB_DIR=openssl.out + "/lib";
}
