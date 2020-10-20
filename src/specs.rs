use std::env;
use std::fs;
use std::process::{Command, Stdio};

#[warn(dead_code)]
pub fn get_product() -> String {
    match fs::read_to_string("/sys/devices/virtual/dmi/id/product_version") {
        Ok(product) => format!("{} {}", "💻 ", product.trim().to_string()),
        Err(_) => String::new(),
    }
}

pub fn get_uptime() -> String {
    let mut uptime = match fs::read_to_string("/proc/uptime") {
        Ok(mut uptime) => {
            uptime = uptime.split('.').collect::<Vec<&str>>()[0].to_string();
            uptime.parse::<u64>().unwrap()
        }
        Err(_) => 0,
    };

    let day = uptime / (24 * 3600);
    uptime %= 24 * 3600;
    format!("{} {} days, {} hours", "  ", day, uptime / 3600)
}

pub fn get_cpu() -> String {
    let cpu = "/proc/cpuinfo";
    let mut model = String::new();
    let data = fs::read_to_string(cpu).unwrap();
    for l in data.split('\n') {
        let cpuinfo = l.split(':').map(|i| i.trim()).collect::<Vec<&str>>();
        if cpuinfo[0] == "model name" {
            model = cpuinfo[1].to_string()
        }
    }
    format!("{} {}", "  ", model)
}

pub fn get_gpu() -> String {
    let cmd = "lspci -mm | grep VGA";
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .output()
        .expect("Error");
    let result = String::from_utf8_lossy(&cmd.stdout);
    format!(
        "{} {}",
        "  ",
        result.split('"').collect::<Vec<&str>>()[5].to_string()
    )
}

pub fn get_disk() -> String {
    let cmd = "df /home -h";
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .output()
        .expect("Error");
    let result = String::from_utf8_lossy(&cmd.stdout);
    let result = result.split('\n').collect::<Vec<&str>>()[1].to_string();
    let mut result = result.split(' ').collect::<Vec<&str>>();
    result.retain(|x| !x.is_empty());
    format!("{} {} / {} ({})", "  ", result[2], result[1], result[4])
}

pub fn get_memory() -> String {
    let mem = "/proc/meminfo";
    let mut used = 0.0;
    let mut total = 0.0;
    let data = fs::read_to_string(mem).unwrap();
    for l in data.split('\n') {
        let meminfo = l.split(':').map(|i| i.trim()).collect::<Vec<&str>>();
        if meminfo.len() != 2 {
            continue;
        }
        let key = meminfo[0];
        let val = meminfo[1].replace(" kB", "").parse::<f64>().unwrap();
        match key {
            "MemTotal" => {
                used += val;
                total = val;
            }
            "Shmem" => used += val,
            "SReclaimable" | "Buffers" | "Cached" | "MemFree" => used -= val,
            _ => (),
        }
    }
    format!(
        "{} {:.0}mb / {:.0}mb",
        "  ",
        used / 1024.0,
        total / 1024.0
    )
}

pub fn get_resolution() -> String {
    let cmd = "xrandr --nograb --current";
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .output()
        .expect("Error");
    let result = String::from_utf8_lossy(&cmd.stdout);
    let result = result.split(',').collect::<Vec<&str>>()[1];
    format!(
        "{} {}",
        "  ",
        result.replace("current ", "").trim().to_string()
    )
}

pub fn get_kernel() -> String {
    let cmd = "uname -r";
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .output()
        .expect("Error");
    format!(
        "{} {}",
        "  ",
        String::from_utf8_lossy(&cmd.stdout).trim().to_string()
    )
}

pub fn get_distro() -> String {
    format!(
        "{} {}",
        "  ",
        fs::read_to_string("/etc/os-release")
            .unwrap()
            .split('"')
            .collect::<Vec<&str>>()[1]
            .to_string()
    )
}

pub fn get_wmde() -> String {
    let mut result = String::new();
    let de = env::var("XDG_DESKTOP_SESSION")
        .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
        .or_else(|_| env::var("DESKTOP_SESSION"));
    match de {
        Ok(de) => result = de,
        Err(_) => {
            let loc = env::var("HOME").unwrap() + "/.xinitrc";
            match fs::read_to_string(loc) {
                Ok(wm) => {
                    let mut wm = wm.split('\n').collect::<Vec<&str>>();
                    wm.retain(|x| !x.is_empty());
                    let wm = wm.last().unwrap();
                    result = wm.replace("exec ", "").to_string();
                }
                Err(e) => println!("Error {}", e),
            }
        }
    }
    format!("{} {}", "  ", result)
}

pub fn get_theme() -> [String; 4] {
    let loc = env::var("HOME").unwrap() + "/.config/gtk-3.0/settings.ini";
    let mut theme = String::new();
    let mut icons = String::new();
    let mut font = String::new();
    let mut cursor = String::new();
    for i in fs::read_to_string(loc).unwrap().split('\n') {
        let i = i.split('=').collect::<Vec<&str>>();
        if i.len() != 2 {
            continue;
        }
        let key = i[0];
        let val = i[1].to_string();
        match key {
            "gtk-theme-name" => theme = val,
            "gtk-icon-theme-name" => icons = val,
            "gtk-font-name" => font = val,
            "gtk-cursor-theme-name" => cursor = val,
            _ => (),
        }
    }
    return [
        format!("{} {}", "  ", theme),
        format!("{} {}", "  ", icons),
        format!("{} {}", "  ", font),
        format!("{} {}", "  ", cursor),
    ];
}

pub fn get_packages() -> String {
    let cmd = "pacman -Q | wc -l";
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .output()
        .expect("Error");
    format!(
        "{} {} (pacman)",
        "  ",
        String::from_utf8_lossy(&cmd.stdout).trim().to_string()
    )
}

pub fn get_music() -> String {
    let cmd = "playerctl metadata --format \"{{ artist }} - {{ title }}\"";
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .output()
        .expect("Error");
    let song = String::from_utf8_lossy(&cmd.stdout).trim().to_string();
    if song.is_empty() {
        String::new()
    } else {
        format!("{} {}", "ﱘ  ", song)
    }
}
