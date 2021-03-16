use crate::logo::{ARCH, VOID};
use cmd_lib::run_fun;
use colored::{Color, Colorize};
use regex::Regex;
use std::{env, fs, path::Path};

pub struct Specs {
    pub logo: String,
    pub os: Option<String>,
    pub kernel: Option<String>,
    pub uptime: Option<String>,
    pub packages: Option<String>,
    pub shell: Option<String>,
    pub monitors: Option<String>,
    pub wmde: Option<String>,
    pub theme: [Option<String>; 4],
    pub terminal: Option<String>,
    pub cpu: Option<String>,
    pub gpu: Option<String>,
    pub memory: Option<String>,
    pub disk: Option<String>,
    pub music: Option<String>,
    pub colours: [String; 2],
    pub shade: Color,
}

impl Specs {
    pub fn new() -> Self {
        Self {
            logo: "".to_string(),
            os: None,
            kernel: None,
            uptime: None,
            packages: None,
            shell: None,
            monitors: None,
            wmde: None,
            theme: [None, None, None, None],
            terminal: None,
            cpu: None,
            gpu: None,
            memory: None,
            disk: None,
            music: None,
            colours: ["".to_string(), "".to_string()],
            shade: Color::White,
        }
    }

    pub fn get(&mut self) {
        self.os = Self::get_os();
        if let Some(os) = &self.os {
            let (logo, shade) = match os.as_str() {
                "Void Linux" => (VOID, Color::Green),
                "Arch Linux" => (ARCH, Color::Blue),
                _ => ("", Color::White),
            };
            self.logo = logo.to_string();
            self.shade = shade;
        }
        self.kernel = Self::get_kernel();
        self.uptime = Self::get_uptime();
        self.packages = Self::get_packages();
        self.shell = Self::get_shell();
        self.monitors = Self::get_monitors();
        self.wmde = Self::get_wmde();
        self.theme = Self::get_theme();
        self.terminal = Self::get_terminal();
        self.cpu = Self::get_cpu();
        self.gpu = Self::get_gpu();
        self.memory = Self::get_memory();
        self.disk = Self::get_disk();
        self.music = Self::get_music();
        self.colours = Self::get_colours();
    }

    pub fn get_os() -> Option<String> {
        let raw = fs::read_to_string("/etc/os-release").ok()?;
        let regex = Regex::new("(?m)^ID=\"?([^\"\n]*)\"?$").unwrap();
        let id = regex.captures(&raw)?.get(1)?;
        Some(
            match id.as_str() {
                "arch" => "Arch Linux",
                "void" => "Void Linux",
                _ => "Linux",
            }
            .to_string(),
        )
    }

    pub fn get_kernel() -> Option<String> {
        run_fun!(uname -rms).ok()
    }

    pub fn get_uptime() -> Option<String> {
        Some(run_fun!(uptime -p).ok()?.chars().skip(3).collect())
    }

    pub fn get_packages() -> Option<String> {
        let mut out = String::new();
        if let Ok(n) = run_fun!(xbps-query -l | wc -l) {
            if Path::new("/usr/bin/flatpak").exists() == true{
                if let Ok(x) = run_fun!(flatpak list | wc -l){
                    out.push_str(&format!("{} (xbps) {} (flatpak)",n,x))
                }
            }else{
                out.push_str(&format!("{} (xbps)", n))
            }
        }
        if let Ok(n) = run_fun!(pacman -Q | wc -l) {
            if Path::new("/usr/bin/flatpak").exists() == true{
                if let Ok(x) = run_fun!(flatpak list | wc -l){
                    out.push_str(&format!("{} (pacman) {} (flatpak)",n,x))
                }
            }else{
                out.push_str(&format!("{} (pacman)", n))
            }
        }
        Some(out)
    }

    pub fn get_shell() -> Option<String> {
        let raw = env::var("SHELL").ok()?;
        let path = Path::new(&raw);
        Some(path.file_name()?.to_str()?.to_string())
    }

    pub fn get_monitors() -> Option<String> {
        let raw = run_fun!(xrandr --listactivemonitors).ok()?;
        let res = Regex::new(r"(\d+)/\d+x(\d+)").unwrap();
        let mut out = vec![];
        for i in raw.split('\n').skip(1) {
            let data = res.captures(i)?;
            let (w, h) = (data.get(1)?.as_str(), data.get(2)?.as_str());
            out.push(format!("{}x{}", w, h));
        }
        Some(out.join(", "))
    }

    pub fn get_wmde() -> Option<String> {
        let de = env::var("XDG_DESKTOP_SESSION")
            .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
            .or_else(|_| env::var("DESKTOP_SESSION"));
        if let Ok(de) = de {
            Some(de)
        } else {
            let path = env::var("HOME").ok()? + "/.xinitrc";
            let raw = fs::read_to_string(path).ok()?;
            let wm = Regex::new("(?m)^exec [^\n]* ([^\n]*)$").unwrap();
            Some(wm.captures(&raw)?.get(1)?.as_str().to_string())
        }
    }

    pub fn get_theme() -> [Option<String>; 4] {
        Self::do_theme().unwrap_or([None, None, None, None])
    }

    fn do_theme() -> Option<[Option<String>; 4]> {
        let loc = env::var("HOME").ok()? + "/.config/gtk-3.0/settings.ini";
        let mut theme = None;
        let mut icons = None;
        let mut font = None;
        let mut cursor = None;
        for i in fs::read_to_string(loc).ok()?.split('\n') {
            let i = i.split('=');
            match i.collect::<Vec<_>>().as_slice() {
                ["gtk-theme-name", name] => theme = Some((*name).to_string()),
                ["gtk-icon-theme-name", name] => icons = Some((*name).to_string()),
                ["gtk-font-name", name] => font = Some((*name).to_string()),
                ["gtk-cursor-theme-name", name] => cursor = Some((*name).to_string()),
                _ => (),
            }
        }
        Some([theme, icons, font, cursor])
    }

    pub fn get_terminal() -> Option<String> {
        let window = env::var("WINDOWID").ok()?;
        let proc = run_fun!(xwininfo -id $window).ok()?;
        let s = Regex::new("\"(.*)\"").unwrap();
        Some(s.captures(&proc)?.get(1)?.as_str().to_string())
    }

    pub fn get_cpu() -> Option<String> {
        let raw = fs::read_to_string("/proc/cpuinfo").ok()?;
        let model = Regex::new(r"(?m)model name\s*:\s*([^\n]*)$").unwrap();
        let cores = Regex::new(r"(?m)siblings\s*:\s*([^\n]*)$").unwrap();
        Some(format!(
            "{} (x{})",
            model.captures(&raw)?.get(1)?.as_str(),
            cores.captures(&raw)?.get(1)?.as_str(),
        ))
    }

    pub fn get_gpu() -> Option<String> {
        Some(
            run_fun!(lspci -mm | grep VGA)
                .ok()?
                .split('"')
                .nth(5)?
                .to_string(),
        )
    }

    pub fn get_memory() -> Option<String> {
        let raw = run_fun!(free -m).ok()?.split('\n').nth(1)?.to_string();
        let total: usize = raw.split_whitespace().nth(1)?.parse().ok()?;
        let used: usize = raw.split_whitespace().nth(2)?.parse().ok()?;
        Some(format!("{}mb / {}mb", used, total))
    }

    pub fn get_disk() -> Option<String> {
        let raw = run_fun!(df /home).ok()?.split('\n').nth(1)?.to_string();
        let used: usize = raw.split_whitespace().nth(2)?.parse().ok()?;
        let available: usize = raw.split_whitespace().nth(3)?.parse().ok()?;
        let percent = raw.split_whitespace().nth(4)?;
        let total = used + available;
        Some(format!(
            "{}gb / {}gb ({} used)",
            used / 1_048_576,
            total / 1_048_576,
            percent
        ))
    }

    pub fn get_music() -> Option<String> {
        run_fun!(playerctl metadata -f "{{ artist }} - {{ title }}").ok()
    }

    pub fn get_colours() -> [String; 2] {
        let dark_scheme = format!(
            "{}{}{}{}{}{}{}{}",
            "   ".on_black(),
            "   ".on_red(),
            "   ".on_green(),
            "   ".on_blue(),
            "   ".on_yellow(),
            "   ".on_magenta(),
            "   ".on_cyan(),
            "   ".on_white(),
        );
        let light_scheme = format!(
            "{}{}{}{}{}{}{}{}",
            "   ".on_bright_black(),
            "   ".on_bright_red(),
            "   ".on_bright_green(),
            "   ".on_bright_blue(),
            "   ".on_bright_yellow(),
            "   ".on_bright_magenta(),
            "   ".on_bright_cyan(),
            "   ".on_bright_white(),
        );
        [dark_scheme, light_scheme]
    }
}
