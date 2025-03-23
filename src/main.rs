use std::fs::{self, OpenOptions};
use std::io::{self, ErrorKind, IsTerminal};
use std::process;
use clap::{Parser, value_parser};

#[derive(Parser)]
#[command(name = "myzstd")]
struct Cli {
    #[arg(short, long, value_parser = value_parser!(u8).range(1..=22),
        help = "The compression level with range of [1..22] and default value of 3")]
    level: Option<u8>,

    #[arg(short, long, help = "The decompression mode to decompress the input file")]
    decompress: bool,

    #[arg(short, long, help = "The force mode to overwrite the existing output file")]
    force: bool,

    #[arg(short, long, help = "The output file")]
    output_file: Option<String>,

    #[arg(help = "The input file")]
    input_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let level = cli.level.unwrap_or(3) as i32;
    let mut input_use_file = false;
    let mut output_use_file = false;
    let mut in_path = String::from("");
    let mut out_path = String::from("");
    let reader : Box<dyn io::Read>;
    let writer : Box<dyn io::Write>;

    if cli.input_file.is_some() {
        in_path = cli.input_file.unwrap();
        if in_path != "-" {
            input_use_file = true;
        }
    }

    if cli.output_file.is_some() {
        out_path = cli.output_file.unwrap();
        output_use_file = true
    }

    if input_use_file {
        // check the input file
        let metadata = fs::metadata(&in_path).unwrap_or_else(|error| {
            eprintln!("Error: Failed in checking the input file '{in_path}': {error:?}");
            process::exit(1);
        });

        // report error if it's a directory
        if metadata.is_dir() {
            eprintln!("Error: The input file '{in_path}' is a directory, not a file.");
            process::exit(1);
        }
        drop(metadata);

        // open the input file
        let input_file = OpenOptions::new()
            .read(true)
            .open(&in_path).unwrap_or_else(|error| {
                eprintln!("Error: Failed in opening the input file '{in_path}': {error:?}");
                process::exit(1);
            });
        reader = Box::new(input_file);

        // get the output file path
        if !output_use_file {
            output_use_file = true;
            out_path = in_path.clone();
            if !cli.decompress {
                out_path.push_str(".zst");
            } else if out_path.len() > 4 && out_path.ends_with(".zst") {
                out_path.truncate(out_path.len() - 4);
            } else {
                out_path.push_str(".unzst");
            }
        }
    }
    else if io::stdin().is_terminal() {
        eprintln!("Error: No input file is specified, and stdin is terminal, existing");
        process::exit(1);
    } else {
        reader = Box::new(io::stdin());
    }

    if output_use_file {
        // check the output file
        let result = fs::metadata(&out_path);
        match result {
            Ok(metadata) => {
                if metadata.is_dir() {
                    eprintln!("Error: '{out_path}' is a directory, not a file.");
                    process::exit(1);
                }
                if !cli.force {
                    eprintln!("Error: output file '{out_path}' exist, but no --force specified, existing");
                    process::exit(1);
                }
            },
            Err(error) => {
                if error.kind() != ErrorKind::NotFound {
                    eprintln!("Error: Failed in checking the output file '{out_path}': {error:?}");
                    process::exit(1);
                }
            }
        }

        // open the output file
        let output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&out_path).unwrap_or_else(|error| {
                eprintln!("Error: Failed in opening the output file '{out_path}': {error:?}");
                process::exit(1);
        });
        writer = Box::new(output_file);
    } else {
        writer = Box::new(io::stdout());
    }

    // do compression or decompression
    if !cli.decompress {
        zstd::stream::copy_encode(reader, writer, level).unwrap();
    } else {
        zstd::stream::copy_decode(reader, writer).unwrap();
    }
}
