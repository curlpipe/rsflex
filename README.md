# Rsflex (Rust Flex)

<img src="https://i.postimg.cc/Kckc2kG2/image.png">

System information tool like neofetch but it's much faster.

## Configuration
I chose to use a suckless method of configuration
in the file `src/main.rs` you'll see an easy macro api that allows quite a lot of customisation

If you want to add a new logo, you can do so in the `src/logo.rs` file

## Installation

### A) AUR

Rsflex is available on the Arhc User Repository as `rsflex-git`:

```
yay -S rsflex-git
```

### B) Cargo

Dependencies:

 - `rustup` - Modern installation of Rust.
 - Arch Or Void Linux (these are the only ones supported)
 - `ttf-nerd-fonts-symbols` - Nerd fonts to render the fancy icons
 - `lspci` - Command via `pciutils`
 - `df` - Command via `coreutils`
 - `xrandr` - Command via `xorg-xrandr`
 - `uname` - Command via `coreutils`
 - `playerctl` - To read MPRIS status

On Arch Linux, these can all be installed with:
```
yay -S pciutils coreutils xorg-xrandr playerctl ttf-nerd-fonts-symbols cargo
```

**Note: These dependencies are only needed if installed via Cargo, they are included automatically with the AUR package above.**

```
cargo install rsflex
```

## Compiling

Building:
```
git clone https://github.com/curlpipe/rsflex
cd rsflex
cargo build --release
```

Running:
```
cargo run --release
```

Install as a binary:
```
cargo install --path .
```

Have fun! :)
