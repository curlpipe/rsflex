mod specs;

use termion::color;
use specs::*;

fn main() {
    let dark_scheme = format!("{}   {}   {}   {}   {}   {}   {}   {}   {}", 
                               color::Bg(color::Black),
                               color::Bg(color::Red),
                               color::Bg(color::Green),
                               color::Bg(color::Blue),
                               color::Bg(color::Yellow),
                               color::Bg(color::Magenta),
                               color::Bg(color::Cyan),
                               color::Bg(color::White),
                               color::Bg(color::Reset));
    let light_scheme = format!("{}   {}   {}   {}   {}   {}   {}   {}   {}", 
                               color::Bg(color::LightBlack),
                               color::Bg(color::LightRed),
                               color::Bg(color::LightGreen),
                               color::Bg(color::LightBlue),
                               color::Bg(color::LightYellow),
                               color::Bg(color::LightMagenta),
                               color::Bg(color::LightCyan),
                               color::Bg(color::LightWhite),
                               color::Bg(color::Reset));

    print!("{}", color::Fg(color::Blue));

    println!("{20}                   ▄                     {19}{}\r
{20}                  ▟█▙                    {19}{}\r
{20}                 ▟███▙                   {19}{}\r
{20}                ▟█████▙                  {19}{}\r
{20}               ▟███████▙                 {19}{}\r
{20}              ▂▔▀▜██████▙                {19}{}\r
{20}             ▟██▅▂▝▜█████▙               {19}{}\r
{20}            ▟█████████████▙              {19}{}\r
{20}           ▟███████████████▙             {19}{}\r
{20}          ▟█████████████████▙            {19}{}\r
{20}         ▟███████████████████▙           {19}{}\r
{20}        ▟█████████▛▀▀▜████████▙          {19}{}\r
{20}       ▟████████▛      ▜███████▙         {19}{}\r
{20}      ▟█████████        ████████▙        {19}{}\r
{20}     ▟██████████        █████▆▅▄▃▂       {19}{}\r
{20}    ▟██████████▛        ▜█████████▙      {19}{}\r
{20}   ▟██████▀▀▀              ▀▀██████▙     {19}{}\r
{20}  ▟███▀▘                       ▝▀███▙    {19}{}\r
{20} ▟▛▀                               ▀▜▙   {19}{}",
    // Move / Uncomment the "" and the get functions around to change the order
    "", //get_product(),
    get_distro(),
    get_kernel(),
    get_wmde(), 
    get_theme()[0],
    get_theme()[1],
    get_theme()[3],
    "",
    get_cpu(), 
    get_gpu(), 
    get_disk(), 
    get_memory(),
    get_uptime(),
    get_resolution(),
    get_packages(),
    get_music(), 
    "", //get_theme()[2],
    dark_scheme, // For showing off your darker colorscheme
    light_scheme, // For showing off your lighter colorscheme
    color::Fg(color::LightWhite), // Color of the text
    color::Fg(color::Blue),  // Color of the logo
);

    print!("{}", color::Fg(color::Reset));
    println!();
}
