use std::env;
use std::io;
use std::process;

fn print_usage(prog: &str) {
    println!("Usage: {} {{-c | -d}} [level]", prog);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut level = 22;

    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let do_comp: bool = match args[1].as_str() {
        "-c" => true,
        "-d" => false,
        _ => {
            print_usage(&args[0]);
            process::exit(1);
        }
    };

    if args.len() > 2 {
        if let Ok(num) = args[2].parse::<i32>() {
            if num > 0 && num <= 22 {
                level = num
            }
        }
    }

    if do_comp {
        zstd::stream::copy_encode(io::stdin(), io::stdout(), level).unwrap();
    } else {
        zstd::stream::copy_decode(io::stdin(), io::stdout()).unwrap();
    }
}
