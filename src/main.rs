use clap::{Command, Arg};

fn main() {
    let matches = Command::new("My RPN program")
        .version("1.0.0")
        .author("yss")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::new("formula_file")
                .help("Fomulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .help("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.get_one::<String>("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    match matches.value_source("verbose") {
        Some(_verbose) => println!("Is verbosity specified?: {}", matches.get_one::<String>("verbose").unwrap()),
        None => {},
    }
}
