use clap::{arg, Command, ArgMatches};
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let matches: ArgMatches = Command::new("MyApp")
        .version("0.1")
        .author("Joshua Soon")
        .about("Does awesome things")
        .arg(arg!(--bedfile <VALUE>).required(true))
        .arg(arg!(--mapfile <VALUE>).required(true))
        .get_matches();

    let bed_file_name: &String = matches.get_one::<String>("bedfile").expect("required");
    let map_file_name: &String = matches.get_one::<String>("mapfile").expect("required");  

    // Print out file names to screen
    println!("BED file loaded: {:?}", bed_file_name);
    println!("Map file loaded: {:?}", map_file_name);

    let bed_file_path: &Path = Path::new(bed_file_name);
    if !bed_file_path.exists() {
        println!("{} not found!", bed_file_name);
        std::process::exit(1)
    }
    let map_file_path: &Path = Path::new(map_file_name);
    if !map_file_path.exists() {
        println!("{} not found!", map_file_name);
        std::process::exit(1)
    }

    let bed_file = File::open(bed_file_path).unwrap();
    let bed_reader = BufReader::new(bed_file);
    for bed_lines in bed_reader.lines() {
        println!("{}", bed_lines.unwrap());
    }
}
