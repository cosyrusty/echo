use std::env;
use std::error::Error;
use std::io::{self, Write};

const HELP: &str = "echo [OPTIONS...] [STRING...]";
const VERSION: &str = "v0.1";

fn main() -> Result<(), Box<dyn Error>> {
    let mut no_newline = false;
    let mut escaped = false;

    let mut values: Vec<String> = vec![];
    let mut parsed_option = false;

    for i in env::args().skip(1) {
        if !parsed_option && i.starts_with('-') {
            match &i[..] {
                "-n" => no_newline = true,
                "-e" => escaped = true,
                "-E" => escaped = false,
                "--help" => {
                    writeln!(io::stdout().lock(), "{}", HELP)?;
                    return Ok(());
                }
                "--version" => {
                    writeln!(io::stdout().lock(), "{}", VERSION)?;
                    return Ok(());
                }
                _ => {
                    values.push(i);
                    parsed_option = true;
                }
            }
        } else {
            parsed_option = true;
            values.push(i)
        }
    }

    let mut output = io::stdout().lock();

    for (i, input) in values.iter().enumerate() {
        if i > 0 {
            write!(&mut output, " ")?
        }

        if escaped {
            let should_stop = print_escaped(input, &mut output)?;
            if should_stop {
                break;
            }
        } else {
            write!(output, "{input}")?;
        }
    }

    if !no_newline {
        writeln!(output)?
    }

    Ok(())
}

fn print_escaped(v: &str, writer: impl Write) -> io::Result<bool> {
    Ok(true)
}
