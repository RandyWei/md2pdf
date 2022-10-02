use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::path::PathBuf;
use std::ffi::OsStr;

use clap::{crate_authors, crate_description, crate_name, crate_version, Command, Arg};

use md2pdf::{markdown_to_latex, markdown_to_pdf};

macro_rules! unwrap {
    ($e: expr, $m: expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {}", $m, e);
                exit(1);
            }
        }
    }
}

fn main() {

    let matches = Command::new(crate_name!())
        .bin_name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(Arg::new("INPUT")
             .long("input")
             .short('i')
             .help("Input markdown files")
             .required(true)
             .value_parser(clap::value_parser!(PathBuf))
            )
        .arg(Arg::new("OUTPUT")
             .long("output")
             .short('o')
             .help("Output tex or pdf file")
             .required(true)
             .value_parser(clap::value_parser!(PathBuf))
            )
        .get_matches();

    let input_path = matches.get_one::<PathBuf>("INPUT").unwrap();
    let mut content = String::new();
    let mut input = unwrap!(File::open(input_path), "couldn't open input file");
    unwrap!(input.read_to_string(&mut content), "couldn't read file content");

    let output_path = matches.get_one::<PathBuf>("OUTPUT").unwrap();
    let output_path_ext = output_path.extension().and_then(OsStr::to_str);
    let mut output = unwrap!(File::create(output_path), "couldn't open output file");

    match output_path_ext {
        Some("tex") => {
            let tex = markdown_to_latex(content);
            unwrap!(output.write(tex.as_bytes()), "couldn't write output file");
        },
        Some("pdf") => {
            let data = unwrap!(markdown_to_pdf(content), "error while compiling latex, this is most likely a bug");
            unwrap!(output.write(&data), "couldn't write output file");
        },
        Some(ext) => {
            eprintln!("unknown file format ({}) for output: {}", ext, output_path.display());
            exit(1);
        },
        None => {
            eprintln!("unknown file format for output: {}", output_path.display());
            exit(1);
        },
    }
}
