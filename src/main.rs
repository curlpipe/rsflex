mod specs;

use termion::color;
use specs::*;

fn main() {
    let colorscheme = format!("{}   {}   {}   {}   {}   {}   {}   {}   {}", 
                               color::Bg(color::Black),
                               color::Bg(color::Red),
                               color::Bg(color::Green),
                               color::Bg(color::Blue),
                               color::Bg(color::Yellow),
                               color::Bg(color::Magenta),
                               color::Bg(color::Cyan),
                               color::Bg(color::White),
                               color::Bg(color::Reset));

    print!("{}", color::Fg(color::Blue));

    println!();
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
    colorscheme, // For showing off your colorscheme
    "",
    color::Fg(color::LightWhite), // Color of the text
    color::Fg(color::Blue),  // Color of the logo
);

    print!("{}", color::Fg(color::Reset));
    println!();
}
