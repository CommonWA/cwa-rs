#![no_main]

#[macro_use]
extern crate cwa;

use cwa::resource::Resource;

static HELP_TEXT: &'static str = r#"Usage: cvrun cat.wasm [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

Examples:
  cat f - g  Output f's contents, then standard input, then g's contents.
  cat        Copy standard input to standard output.
"#;

#[derive(Default)]
struct CmdOpts {
}

fn dump_stdin() {
    cwa::io::with_stdin(|input| {
        cwa::io::with_stdout(|out| {
            std::io::copy(input, out).unwrap();
        })
    });
}

fn parse_opt(_opts: &mut CmdOpts, key: &str) {
    match key {
        "help" => {
            eprintln!("{}", HELP_TEXT);
            ::std::process::exit(0);
        },
        _ => {
            eprintln!("Unknown option: {}", key);
            eprintln!("{}", HELP_TEXT);
            panic!();
        }
    }
}

main!({
    let mut opts: CmdOpts = CmdOpts::default();
    let mut opts_ended: bool = false;
    let mut done: usize = 0;
    let mut args = cwa::startup::args().into_iter();
    args.next().unwrap(); // skip self

    for _ in args.map(|filename| {
        if !opts_ended && filename.starts_with("--") {
            if filename.len() == 2 {
                opts_ended = true;
                return;
            }

            parse_opt(&mut opts, filename.split_at(2).1);
            return;
        }

        opts_ended = true;

        done += 1;

        if filename == "-" {
            dump_stdin();
            return;
        }

        let _: Option<()> = Resource::open(&filename)
            .or_else(|| {
                eprintln!("Unable to open resource `{}`", filename);
                None
            })
            .and_then(|mut file| {
                cwa::io::with_stdout(|out| {
                    std::io::copy(&mut file, out).unwrap();
                });
                None
            });
    }) {}

    // No path arguments
    if done == 0 {
        dump_stdin();
    }
});
