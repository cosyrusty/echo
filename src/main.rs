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
                return Ok(());
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

// TODO: impl \x and \0 seq
fn print_escaped(v: &str, mut writer: impl Write) -> io::Result<bool> {
    let mut should_stop = false;

    let mut iter = v.chars();
    let mut buffer = ['\\'; 2];
    // buffer = ['\', '\'] is important
    // if there is a seq than, parse seq replace buf[1] = parsed char
    // and print the 2nd element only or buf[1] only, hence start = 1.

    // else not a seq than, replace buf[1] the same as it was in original str
    // print the buffer from start, hence start = 0 and return 'next' itself when
    // nothing matched

    while let Some(mut c) = iter.next() {
        let mut start = 1; // index, from where to print the buffer
        if c == '\\' {
            if let Some(next) = iter.next() {
                c = match next {
                    '\\' => '\\',
                    'a' => '\x07',
                    'b' => '\x08',
                    'c' => {
                        should_stop = true;
                        break;
                    }
                    'e' => '\x1b',
                    'f' => '\x0c',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    'v' => '\x0b',
                    'x' => todo!(), // parse hexcode seq
                    '0' => todo!(), // parse octcal upto 2 digits
                    _ => {
                        start = 0;
                        c
                    }
                }
            }
        }

        buffer[1] = c;

        for c in &buffer[start..] {
            write!(writer, "{c}")?
        }
    }

    Ok(should_stop)
}
