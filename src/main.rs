use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::{stdout, Write},
};

use clap::Parser;

/**
 * file: The file to read
 * com: Single line comment form.
 * writer: The file to write to.
 */
fn run(file: String, com: String, writer: &mut Box<dyn Write>) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&file).unwrap();
    let mut code_mode = false;
    let mut ignore_mode = false;
    for line in content.lines() {
        if line.trim() == "%@lit-ignore on" {
            ignore_mode = true;
        }
        if !ignore_mode {
            if code_mode {
                if line.trim() == "\\end{code}" {
                    writeln!(writer, "{com}{line}")?;
                    code_mode = false;
                } else {
                    writeln!(writer, "{line}")?;
                }
            } else {
                writeln!(writer, "{com}{line}")?;
                if line.trim() == "\\begin{code}" {
                    code_mode = true;
                }
            }
        }
        if line.trim() == "%@lit-ignore off" {
            ignore_mode = false;
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[clap(
    name="unlit",
    author="Yaxley Peaks <epiclycoolgaemer@gmail.com>",
    version,
    about="Haskell style literate programming for all languages", long_about=None)]
struct Args {
    #[clap(
        short = 'c',
        help = "Single line comment character sequence. Default: '//'",
        value_parser
    )]
    com: Option<String>,
    #[clap(short = 'i', help = "Input file", value_parser)]
    in_file: String,
    #[clap(short = 'o', help = "Output file. Default: <stdout>", value_parser)]
    out_file: Option<String>,
}

fn main() {
    let _arg = Args::parse();
    let mut writer: Box<dyn Write> = match _arg.out_file {
        Some(filename) => match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
        {
            Ok(handle) => Box::new(handle),
            Err(err) => panic!("{err}"),
        },
        None => Box::new(stdout()),
    };
    run(
        _arg.in_file,
        _arg.com.unwrap_or("//".to_string()),
        &mut writer,
    )
    .expect("Error writing file");
}
