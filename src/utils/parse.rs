use std::env;
use getopts::Options;

pub fn parse_args() -> (String, String, usize) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("i", "input", "set input folder", "FOLDER");
    opts.optopt("o", "output", "set output folder", "FOLDER");
    opts.optopt("d", "dot-size", "set dot size", "SIZE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e);
            print_usage(&args[0]);
            std::process::exit(1);
        }
    };

    let input = matches.opt_str("i").unwrap_or_else(|| "input".to_string());
    let output = matches.opt_str("o").unwrap_or_else(|| "output".to_string());
    let dot_size: usize = matches.opt_str("d")
        .map_or(7, |size_str| size_str.parse().unwrap_or_else(|_| {
            eprintln!("Invalid dot size");
            print_usage(&args[0]);
            std::process::exit(1);
        }));

    (input, output, dot_size)
}

fn print_usage(program: &str) {
    println!("Usage: {} -i <input> -o <output> [-d <dot_size>]", program);
}
