# Rsflex (Rust Flex)

<img src="https://i.postimg.cc/DyPtQ50g/image.png">

System information tool like neofetch or rsfetch with a fast, beautiful CLI.

## Configuration
Rsflex aims to be so simple that you can easily edit the source code!
In the `src/main.rs` file, you can look towards the bottom of the file and rearrange the functions to your hearts content!

## Installation

### A) AUR

Rsflex is available on the Arhc User Repository as `rsflex-git`:

```
yay -S rsflex-git
```

### B) Cargo

Dependencies:

 - `rustup` - Modern installation of Rust.
 - `Arch Linux` - Linux Distribution, only Arch supported right now
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
