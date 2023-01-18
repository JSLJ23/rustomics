use clap::{arg, Command, ArgMatches};
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::fs;


fn main() {
    let matches: ArgMatches = Command::new("MyApp")
        .version("0.1")
        .author("Joshua Soon")
        .about("Tools for bioinformatics built in awesome Rust!")
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

    let mut chromosome_map : HashMap<String, String>= HashMap::new();

    let map_file: File = File::open(map_file_path).unwrap();
    let map_reader: BufReader<File> = BufReader::new(map_file);
    for map_line in map_reader.lines(){
        let map_lines_split:Vec<String> = Vec::from_iter(map_line.unwrap().split("\t").map(String::from));
        chromosome_map.insert(map_lines_split[0].to_string(), map_lines_split[1].to_string());
        println!("{},{}", map_lines_split[0].to_string(), map_lines_split[1].to_string());
    }

    let mut new_bed_file_vec: Vec<String> = Vec::new();

    let bed_file: File = File::open(bed_file_path).unwrap();
    let bed_reader: BufReader<File> = BufReader::new(bed_file);
    for bed_lines in bed_reader.lines() {
        let bed_line: String = bed_lines.unwrap();
        let chromosome_identifer: &str = bed_line.split_whitespace().next().unwrap();
        let new_chromosome_identifier: &str = chromosome_map.get(chromosome_identifer).unwrap().as_str();
        let new_bed_line: String = bed_line.replace(chromosome_identifer, new_chromosome_identifier);
        new_bed_file_vec.push(new_bed_line);
    }

    let output_bed_file = format!("{}{}", "renamed_", bed_file_name);
    fs::write(output_bed_file, new_bed_file_vec.join("\n")).expect("");

}
