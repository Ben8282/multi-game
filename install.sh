#!/bin/sh
set -eu

INSTALL_DIR="$HOME/.local/share/multi-game"

download(){
    echo "Downloading $1"
}

has(){
    command -v "$1" >/dev/null 2>&1
}

main(){
    echo "installing multi-game into $INSTALL_DIR"

    os=$(uname -s)

    case "$os" in
        Linux)
            echo "Linux detected"
            ;;
        Darwin)
            echo "MacOS detected"
            ;;
        FreeBSD|DragonFly)
            echo "you are on $os"
            ;;
        OpenBSD)
            echo "OpenBSD detected"
            ;;
        NetBSD)
            echo "NetBSD detected"
            ;;
         *)
            echo "Unsupported OS: $os"
            exit 1
            ;;
    esac
    if has pacman; then
        PM="pacman"
    elif has apt-get; then
        PM="apt-get"
    elif has dnf; then
        PM="dnf"
    elif has zypper; then
        PM="zypper"
    elif has apk; then
        PM="apk"
    elif has xbps-install; then
        PM="xbps-install"
    elif has emerge; then
        PM="emerge"
    elif has eopkg; then
        PM="eopkg"
    elif has pkg; then
        PM="pkg"
    elif has pkg_add; then
        PM="pkg_add"
    elif has pkgin; then
        PM="pkgin"
    elif has nix-shell; then
        PM="nix"
    elif has guix; then
        PM="guix"
    else
        log "could not find a supported package manager"
        log "please install these yourself: git, python3, pygame, java, rust"
        log "then run this installer again"
        exit 1
    fi
    
    if has git; then
        echo "git is installed, cloning repo"
    else
        echo "git is not installed"
    fi
}