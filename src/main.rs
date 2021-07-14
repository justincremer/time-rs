use std::env;
use std::io::{stderr, stdout, Write};

use std::process::{exit, Command};
use std::time::Instant;

const MAN_PAGE: &'static str = /* @MANSTART{time} */
    r#"
NAME
    time - timer for commands

SYNOPSIS
    time [ -h | --help ][COMMAND] [ARGUEMENT]...

DESCRIPTION
    Runs the command taken as the first arguement and outputs the time the command took to execute.

OPTIONS
    -h
    --help
        display this help and exit
"#; /* @MANEND */

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut stderr = stderr();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => {
            let _ = stderr.write(b"Please provide an argument\n");
            exit(1);
        }
        length => match args[1].as_str() {
            "-h" | "--help" => {
                if let Err(e) = stdout.write(MAN_PAGE.as_bytes()) {
                    let _ = stderr.write(format!("{}\n", e).as_bytes());
                    exit(1);
                };
                exit(0);
            }
            _ => {
                let mut command = Command::new(&args[1]);
                if length > 2 {
                    for arg in &args[2..] {
                        command.arg(arg);
                    }
                }

                let time = Instant::now();
                match command.spawn() {
                    Ok(mut handle) => {
                        let _ = handle.wait();
                        let duration = time.elapsed();
                        if let Err(e) = stdout.write(
                            format!(
                                "\nTook {}m {:.3}s",
                                duration.as_secs() / 60,
                                (duration.as_secs() % 60) as f64
                                    + (duration.subsec_nanos() as f64) / 1000000000.0,
                            )
                            .as_bytes(),
                        ) {
                            let _ = stderr.write(format!("{}\n", e).as_bytes());
                            exit(1);
                        }
                        exit(0);
                    }
                    Err(e) => {
                        let _ = stderr.write(format!("{}\n", e).as_bytes());
                        exit(1);
                    }
                }
            }
        },
    };
}
