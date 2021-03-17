#[macro_export]
macro_rules! show {
    ($pre:literal, $data:expr, $space:expr) => {
        if let Some(os) = $data {
            $space.push(format!("{}{}", $pre, os));
        }
    };
    ($space:expr) => {
        $space.push("".to_string());
    };
    ($data:expr, $space:expr) => {
        $space.extend_from_slice(&$data)
    };
}

#[macro_export]
macro_rules! render {
    ($system:expr, $space:expr) => {
        let rows = $system.logo.split('\n');
        let max = rows.clone().map(|x| uws::width(x)).max().unwrap() + 3;
        let rows: Vec<&str> = rows.collect();
        println!("");
        for i in 0..cmp::max(rows.len(), $space.len()) {
            if let Some(k) = rows.get(i) {
                use lliw::Fg;
                let pad = " ".repeat(max - uws::width(&k[..]));
                print!("{}{}{}{}", $system.shade, k, pad, Fg::Reset);
            } else {
                print!("{}", " ".repeat(max));
            }
            if let Some(v) = $space.get(i) {
                print!("{}", v);
            }
            println!("");
        }
    };
}
