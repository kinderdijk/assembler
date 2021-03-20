use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result, BufRead};
use phf::phf_map;
use clap::{Arg, App, crate_version};

/////////////////////////////////////////////////////////
/// 
/// Things to do:
///   - Add labels and the function to work on that.
///   - Different addressing modes. 
/// 
//////////////////////////////////////////////////////////


static INSTRUCTIONS: phf::Map<&'static str, u8> = phf_map! {
    "LDA" => 0b00000001,
    "STA" => 0b00000010,
    "ADD" => 0b00000011,
    "LDI" => 0b00000100,
    "OUT" => 0b00000101,
    "HLT" => 0b00011111,
};

fn main() -> Result<()> {
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
                            .get_matches();


    let filename = arg_matches.value_of("ASM_FILE").unwrap();
    let verbosity = match arg_matches.occurrences_of("v") {
        0 => "info",
        1 => "debug",
        2 => "trace",
        3 | _ => "trace"
    };

    println!("{:?}", filename);

    if !valid_file(filename) {
        panic!("File is not valid. filename={}", filename);
    }

    let file_lines = read_lines_from_file(filename).unwrap();
    let mut binary_instructions = Vec::<u8>::new();
    for line in file_lines {
        if line.starts_with(".") {
            // Deal with the directive commands in here.
            println!("Line that starts with period. {:?}", line);
        } else {
            // Deal with the other instructions here.
            decode_instruction(&line, &mut binary_instructions);
        }
    }
    binary_instructions.push(0b11111111);
    if verbosity == "debug" {
        println!("binary: {:?}", binary_instructions);
        println!("Writing file....");
    }

    let mut out_file = File::create("out.myobj")?;
    out_file.write_all(&binary_instructions)?;

    Ok(())
}

fn valid_file(filename: &str) -> bool {
    let mut valid: bool = true;
    let extension = Path::new(filename).extension()
            .and_then(OsStr::to_str)
            .expect("No extension was found.");

    if extension != "myasm" {
        valid = false;
        return valid;
    }

    return valid;
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

fn decode_instruction(instruction_line: &str, binary_instructions: &mut Vec<u8>) {
    // Find the instructions and replace them with the opcode. 
    // Throw an error if there is an unrecognized instruction.

    if !instruction_line.is_empty() {
        let instruction = &instruction_line[..3];
        match INSTRUCTIONS.get(instruction) {
            Some(code) => binary_instructions.push(*code),
            None => panic!("Invalid instruction: {} is not defined.", instruction)
        }

        if instruction_line.contains("$") {
            let memory = &instruction_line[5..];
            let memory: u8 = memory.parse().unwrap();
            binary_instructions.push(memory);
            println!("Memory location: {:?}", memory);
        } else {
            let memory = 0b00000000;
            binary_instructions.push(memory);
        }
    }
}