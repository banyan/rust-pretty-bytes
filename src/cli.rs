use ::converter;

use std::env;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} <from> <to> [target_files]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

pub fn run(args: env::Args) -> () {
    let args: Vec<String> = args.collect();
    let ref program = args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print version");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("v") {
        return print_version();
    } else if matches.opt_present("h") || args.len() != 2 {
        return print_usage(&program, opts);
    }

    let num = match args[1].parse::<f64>() {
        Ok(n) => n,
        Err(f) => { panic!(f.to_string()) }
    };

    // TODO change name
    converter::convert(num);
}
