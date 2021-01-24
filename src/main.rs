use clap::{App, Arg};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("yuucu")
        .about("")
        .arg(
            Arg::with_name("formula_file")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::with_name("verbos")
                .short("v")
                .long("verbose")
                .required(false),
        )
        .get_matches();

    let verbose = matches.is_present("verbos");
    if let Some(path) = matches.value_of("formula_file") {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
