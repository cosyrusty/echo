use std::env;

const HELP: &str = "echo [OPTIONS...] [STRING...]";
const VERSION: &str = "v0.1";

fn main() {
    let mut no_newline = false;
    let mut escaped = false;
    let mut help = false;
    let mut version = false;

    let mut values: Vec<String> = vec![];
    let mut parsed_option = false;

    'a: for i in env::args().skip(1) {
        if !parsed_option && i.starts_with('-') {
            match &i[..] {
                "-u" => no_newline = true,
                "-e" => escaped = true,
                "-E" => escaped = false,
                "--help" => {
                    help = true;
                    break 'a;
                }
                "--version" => {
                    version = true;
                    break 'a;
                }
                _ => {}
            }
        } else {
            parsed_option = true;
            values.push(i.clone())
        }
    }

    if help {
        print!("{}", HELP)
    } else if version {
        print!("{}", VERSION)
    } else if escaped {
        print_escaped(values)
    } else {
        for (i, v) in values.iter().enumerate() {
            if i > 0 {
                print!(" {}", v)
            } else {
                print!("{}", v)
            }
        }
    }

    if !no_newline {
        println!()
    }
}

fn print_escaped(values: Vec<String>) {
    print!("Hello Ecs")
}
