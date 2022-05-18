default: install-dev
set windows-powershell := true
# export RUSTFLAGS="-Ctarget-feature=+crt-static -Ctarget-cpu=native"
# Build for macOS Intel and macOS Apple Silicon
build-mac:
    rm -f out/useragent-macos-x64.zip out/useragent-macos-arm.zip
    mkdir -p out
    cargo build --target=x86_64-apple-darwin --release
    cargo build --target=aarch64-apple-darwin --release
    zip -r out/useragent-macos-x64.zip -j target/x86_64-apple-darwin/release/useragent
    zip -r out/useragent-macos-arm.zip -j target/aarch64-apple-darwin/release/useragent

# Build for Windows MSVC
build-win:
    if (Test-Path -Path ".\out\useragent-windows-msvc.zip") { Remove-Item -Path ".\out\useragent-windows-msvc.zip" }
    if (-Not (Test-Path -Path ".\out")) { New-Item -Name "out" -ItemType Directory }
    set RUSTFLAGS="-Ctarget-feature=+crt-static"
    cargo build --target=x86_64-pc-windows-msvc --release
    Compress-Archive -Path "target\x86_64-pc-windows-msvc\release\useragent.exe" -DestinationPath "out\useragent-windows-msvc.zip"

# Build for GNU Linux and GNU Windows (e.g. cygwin)
build-linux:
    rm -f out/useragent-linux-gnu.zip out/useragent-windows-gnu.zip
    mkdir -p out
    rustup target add x86_64-pc-windows-gnu
    cargo build --target=x86_64-pc-windows-gnu --release
    cargo build --target=x86_64-unknown-linux-gnu --release
    zip -r out/useragent-linux-gnu.zip -j target/x86_64-unknown-linux-gnu/release/useragent
    zip -r out/useragent-windows-gnu.zip -j target/x86_64-pc-windows-gnu/release/useragent.exe

# Run clippy lints
lint:
    cargo clippy -- \
        -D clippy::all \
        -D clippy::perf \
        -D clippy::style \
        -D clippy::cargo \
        -D clippy::suspicious \
        -D clippy::complexity \
        -W clippy::nursery \
        -W clippy::pedantic \
        -A clippy::too-many-lines \
        -A clippy::non-ascii-literal \
        -A clippy::single-match-else \
        -A clippy::let-underscore-drop \
        -A clippy::multiple-crate-versions

# Install useragent to cargo's binary folder
install:
    cargo install --force --path .

# Install useragent to cargo's binary folder but debug
install-dev:
    cargo install --debug --force --path .

clean:
    cargo clean
    rm -rf out
    rm -rf tests/mods
    rm -rf tests/configs/running
