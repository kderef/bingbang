#[macro_export]
macro_rules! pflush {
    () => {
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    };
}

#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        {
            eprintln!("\x1b[91m[ERROR]\x1b[0m: {}", $msg);
        }
    };
    ($msg:expr, $($fmt_args:expr),*) => {
        {
            eprintln!("\x1b[91m[ERROR]\x1b[0m: {}", format!($msg, $($fmt_args),*));
        }
    };
}

#[macro_export]
macro_rules! assert_err {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            eprintln!("\x1b[91m[ERROR]\x1b[0m: {}", $msg);
            std::process::exit(1);
        }
    };
    ($cond:expr, $msg:expr, $($fmt_args:expr),*) => {
        if !($cond) {
            eprintln!("\x1b[91m[ERROR]\x1b[0m: {}", format!($msg, $($fmt_args),*));
            std::process::exit(1);
        }
    }
}