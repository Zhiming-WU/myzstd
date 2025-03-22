use std::fs::{self, OpenOptions};
use std::io::{self, IsTerminal};
use std::process;
use clap::{Parser, value_parser};

#[derive(Parser)]
#[command(name = "myzstd")]
struct Cli {
    #[arg(short, long, value_parser = value_parser!(u8).range(1..=22))]
    level: Option<u8>,

    #[arg(short, long)]
    decompress: bool,

    input_file_path: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let level = cli.level.unwrap_or(3) as i32;

    if let Some(path) = cli.input_file_path {
        // check the input file
        let metadata = fs::metadata(&path).unwrap_or_else(|error| {
            eprintln!("Error: Failed in checking the input file({path}): {error:?}");
            process::exit(1);
        });

        // report error if it's a directory
        if metadata.is_dir() {
            eprintln!("Error: '{path}' is a directory, not a file.");
            process::exit(1);
        }
        drop(metadata);

        // open the input file
        let input = OpenOptions::new()
            .read(true)
            .open(&path).unwrap_or_else(|error| {
            eprintln!("Error: Failed in opening the input file({path}): {error:?}");
            process::exit(1);
        });

        // open the input file
        let mut out_path: String = path.clone();
        out_path.push_str(".zst");
        let output = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&out_path).unwrap_or_else(|error| {
                eprintln!("Error: Failed in opening the output file({out_path}): {error:?}");
                process::exit(1);
        });
        drop(out_path);

        // do compression or decompression
        if !cli.decompress {
            zstd::stream::copy_encode(input, output, level).unwrap();
        } else {
            zstd::stream::copy_decode(input, output).unwrap();
        }
    } else if io::stdin().is_terminal() {
        eprintln!("Error: no input file is specified, and stdin is terminal, existing");
        process::exit(1);
    } else {
        if !cli.decompress {
            zstd::stream::copy_encode(io::stdin(), io::stdout(), level).unwrap();
        } else {
            zstd::stream::copy_decode(io::stdin(), io::stdout()).unwrap();
        }
    }
}
