#![warn(clippy::all, clippy::pedantic)]
mod logo;
mod macros;
mod specs;

use specs::Specs;
use std::cmp;
use unicode_width::UnicodeWidthStr as uws;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[allow(clippy::vec_init_then_push)]
fn main() {
    // Read the system specifications
    let mut system = Specs::new();
    system.get();
    let mut info: Vec<String> = vec![];
    // Customise your fetch here:
    show!(info); // Seperator
    show!(info); // Seperator
    show!("ﲾ   ", system.os, info);
    show!("   ", system.kernel, info);
    //show!("ﲊ   ", system.uptime, info);
    show!("   ", system.packages, info);
    show!("   ", system.shell, info);
    //show!("   ", system.cpu, info);
    //show!("   ", system.gpu, info);
    show!("   ", system.memory, info);
    show!("﫭  ", system.disk, info);
    show!(info); // Seperator
    show!("   ", system.monitors, info);
    show!("缾  ", system.wmde, info);
    //show!("   ", &system.theme[0], info);
    //show!("   ", &system.theme[1], info);
    //show!("   ", &system.theme[2], info);
    //show!("   ", &system.theme[3], info);
    show!("   ", system.terminal, info);
    show!(info); // Seperator
    show!("   ", system.music, info);
    show!(info); // Seperator
                 // Print Colours
    show!(system.colours, info);
    // Render
    render!(system, info);
}
