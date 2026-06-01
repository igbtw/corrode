use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let flags = Flags::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Reading {}:", flags.filename);

    let contents = fs::read_to_string(flags.filename)
        .expect("Something went wrong while trying to read the file.");
    println!("The contents is:\n{}", contents);
}

struct Flags {
    filename: String,
}

impl Flags {
    fn new(args: &[String]) -> Result<Flags, &str> {
        // returns the filename if Ok and a str if error
        if args.len() < 2 {
            // run on cargo is tecnically a flag
            return Err("Not enough arguments/flags.");
        }

        let filename = args[1].clone();

        Ok(Flags { filename })
    }
}
