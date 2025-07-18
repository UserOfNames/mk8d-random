use std::io::{self, Write};

#[macro_export]
macro_rules! continue_on_err {
    ($result:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        }
    };

    ($result:expr, $emsg:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}: {}", $emsg, e);
                continue;
            }
        }
    }
}

pub fn get_input(mes: &str) -> Result<String, io::Error> {
    print!("{mes}");
    let mut buf = String::new();
    io::stdout().flush()?;

    io::stdin().read_line(&mut buf)?;

    let len = buf.trim_end_matches('\n').len();
    buf.truncate(len);
    Ok(buf)
}
