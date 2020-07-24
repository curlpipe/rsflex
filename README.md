# Rsflex (Rust Flex)

<img src="https://i.postimg.cc/DyPtQ50g/image.png">

A tool similar to that of neofetch and rsfetch that gets system information in the blink of an eye.

# Configuration
Rsflex aims to be so simple that you can easily edit the source code!
In the `src/main.rs` file, you can look towards the bottom of the file and rearrange the functions to your hearts content!

# Installation

Here is what you have to have if you wish to have the full experience of rsflex:

 - A modern installation of the rust compiler and cargo `rustup`
 - A linux distrobution at the moment, only Arch Linux is supported :(
 - Nerd fonts to render the fancy icons `ttf-nerd-fonts-symbols`
 - the `lspci` command via `pciutils`
 - the `df` command via `coreutils`
 - the `xrandr` command via `xorg-xrandr`
 - the `uname` command via `coreutils`
 - `playerctl` to read MPRIS status

Here's a handy command to ensure you have everything you need!
```
yay -S pciutils coreutils xorg-xrandr playerctl ttf-nerd-fonts-symbols curl git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

And when you want to build it:
```
git clone https://github.com/curlpipe/rsflex
cd rsflex
cargo build --release
```

To test that it works:
```
cargo run --release
```

If everything is in order, then you can go ahead and copy it over to your other binaries!
```
sudo cp ./target/release/rsflex /usr/bin/rsflex
```

You should now be able to execute `rsflex` wherever you are in your shell!

Have fun! :)
