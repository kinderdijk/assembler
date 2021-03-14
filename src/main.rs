use std::env;
use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result, BufRead};
use std::collections::HashMap;

const LDA: u8 = 0b00000001;
const STA: u8 = 0b00000010;
const ADD: u8 = 0b00000011;
const LDI: u8 = 0b00000100;
const OUT: u8 = 0b00000101;
const HLT: u8 = 0b00011111;

fn main() -> Result<()> {
    let instructions: HashMap<&str, u8> = [
        ("LDA", LDA),
        ("STA", STA),
        ("ADD", ADD),
        ("LDI", LDI),
        ("OUT", OUT),
        ("HLT", HLT)
    ].iter().cloned().collect();
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
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
            decode_instruction(&line, &instructions, &mut binary_instructions);
            println!("binary: {:?}", binary_instructions);

            let mut out_file = File::create("out.myobj")?;
            out_file.write_all(&binary_instructions)?;
        }
    }

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

fn decode_instruction(instruction_line: &str, instructions: &HashMap<&str, u8>, binary_instructions: &mut Vec<u8>) {
    // Find the instructions and replace them with the opcode. 
    // Throw an error if there is an unrecognized instruction.

    if !instruction_line.is_empty() {
        let instruction = &instruction_line[..3];
        match instructions.get(instruction) {
            Some(code) => binary_instructions.push(*code),
            None => println!("Instruction not valid. {:?}", instruction)
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