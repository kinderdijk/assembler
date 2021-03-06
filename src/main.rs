use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result, BufRead};
use clap::{Arg, App, crate_version};
use env_logger::Builder;
use log::{LevelFilter, trace, debug, info};

mod instruction;

/////////////////////////////////////////////////////////
/// 
/// Things to do:
///   - Add labels and the function to work on that.
///   - Add define directlive.
/// 
//////////////////////////////////////////////////////////


fn main() -> Result<()> {
    // Setting up the arguments for the program.
    let arg_matches = App::new("Custom 8-bit Computer Assembler")
                            .version(crate_version!())
                            .author("Jon Pendlebury")
                            .about("Assembles a custom script to be run on a custom 8-bit computer")
                            .arg(Arg::with_name("ASM_FILE")
                                    .help("The name of the assembly file. Must be 'myasm' extension")
                                    .required(true))
                            .arg(Arg::with_name("v")
                                    .short("v")
                                    .multiple(true)
                                    .help("Sets the verbosity of the output."))
                            .arg(Arg::with_name("o")
                                    .short("o")
                                    .help("Set the name of the output files. If nothing is specified the default filename will be 'out'")
                                    .takes_value(true))
                            .get_matches();


    // Get the arument values and set them accordingly.
    let prog_filename = arg_matches.value_of("ASM_FILE").unwrap();
    let out_filename = match arg_matches.value_of("o") {
        Some(value) => value,
        None => "out"
    };
    let verbosity = match arg_matches.occurrences_of("v") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 | _ => LevelFilter::Trace,
    };
    Builder::new().filter_level(verbosity).init();
    let output_filename = format!("{}{}", out_filename, ".myobj");

    debug!("Input filename: {:?}", prog_filename);
    debug!("Output filename: {:?}", output_filename);

    if !valid_file(prog_filename) {
        panic!("File is not valid. filename={}", prog_filename);
    }

    let instruction_set: instruction::InstructionSet = instruction::InstructionSet::new().unwrap();

    let file_lines = read_lines_from_file(prog_filename).unwrap();
    let mut binary_instructions = Vec::<u8>::new();
    for line in file_lines {
        if line.starts_with(".") {
            // Deal with the directive commands in here.
            info!("Line that starts with period. {:?}", line);
        } else {
            // Deal with the other instructions here.
            if !line.is_empty() {
                let parsed_value: instruction::OpCode = instruction_set.map_instruction(&line);
                binary_instructions.push(parsed_value.instruction);
                binary_instructions.push(parsed_value.argument);
            }
        }
    }
    binary_instructions.push(0b11111111);
    trace!("Binary values: {:?}", binary_instructions);
    debug!("Writing file.....");

    let mut out_file = File::create(output_filename)?;
    out_file.write_all(&binary_instructions)?;

    Ok(())
}

fn valid_file(filename: &str) -> bool {
    let extension = Path::new(filename).extension()
            .and_then(OsStr::to_str)
            .expect("No extension was found.");

    if extension != "myasm" {
        panic!("Invalid file extension. {:?}", extension)
    }

    return true;
}

fn read_lines_from_file(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename).expect("Unable to open ");
    let file = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in file.lines() {
        let line = line.expect("Unable to read file line.");
        lines.push(line);
    }
    Ok(lines)
}
