use std::env::var;

macro_rules! unwrap {
    [ $val:expr ] => {
        $val.unwrap_or_else(|_| "".to_string())
    };
}

fn main() {
    println!("   \x1b[32;1muser\x1b[0m {user}\n     \x1b[32;1msh\x1b[0m {shell}\n     \x1b[32;1mwm\x1b[0m {desktopenv} ({backend})\n   \x1b[32;1mterm\x1b[0m {term}\n \x1b[32;1mlocale\x1b[0m {lang}\n   \x1b[32;1mcols\x1b[0m \x1b[41m  \x1b[42m  \x1b[43m  \x1b[44m  \x1b[45m  \x1b[0m",
        user = unwrap!(var("USER")),
        shell = unwrap!(var("SHELL")),
        desktopenv = unwrap!(var("XDG_CURRENT_DESKTOP")),
        backend = unwrap!(var("XDG_SESSION_TYPE")),
        term = unwrap!(var("TERM")),
        lang = unwrap!(var("LANG")),
    );
}
