use log::{trace, debug, info};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub struct OpCode {
    pub instruction: u8, 
    pub argument: u8
}

#[derive(Debug, Deserialize)]
pub struct InstructionSet {
    instructions: HashMap<String, InstructionAddress>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstructionAddress {
    op_code: u8,
    #[serde(default)]
    addressable_op_code:u8
}

impl InstructionSet {
    pub fn new() -> Result<InstructionSet, ()> {
        let file = File::open("./instructions.json").unwrap();
        let reader = BufReader::new(file);

        let set = serde_json::from_reader(reader).unwrap();

        trace!("Read the file to json. {:?}", set);
        Ok(set)
    }

    pub fn map_instruction(&self, instruction_line: &str) -> OpCode {
        trace!("Mapping instruction line {}", instruction_line);

        let instruction = get_instruction_part(instruction_line);
        debug!("Got the instruction part of the line. {:?}", instruction);
    
        let operand = get_operand_part(instruction_line);
        debug!("Got the operand part of the line. {:?}", operand);
    
        let final_operand: u8;
        let final_instruction: u8;
        match operand {
            None => {
                final_operand = 0;
                final_instruction = Self::get_address_operation(&self, instruction, false);
            }
            Some(value) => {
                final_operand = value[1..].parse().unwrap(); 
                if value.contains("#") {
                    final_instruction = Self::get_address_operation(&self, instruction, false);
                } else if value.contains("$") {
                    final_instruction = Self::get_address_operation(&self, instruction, true);
                } else {
                    final_instruction = Self::get_address_operation(&self, instruction, false);
                }
            }
        };

        OpCode {
            instruction: final_instruction, 
            argument: final_operand
        }
    }

    fn get_address_operation(&self, instruction: String, is_address: bool) -> u8 {
        match self.instructions.get(&instruction) {
            None => {
                info!("The instruction '{}' is not found.", instruction);
                0
            },
            Some(value) => {
                if is_address {
                    debug!("Getting addressable op-code: {}", value.addressable_op_code);
                    value.addressable_op_code
                } else {
                    debug!("Getting op-code: {}", value.op_code);
                    value.op_code
                }
            }
        }
    }
}

fn get_instruction_part(instruction_line: &str) -> String {
    let trimmed_instruction_string = instruction_line.trim_start().to_uppercase();

    match trimmed_instruction_string.find(" ") {
        Some(value) => trimmed_instruction_string[..value].to_string(),
        None => trimmed_instruction_string.to_string()
    }
}

fn get_operand_part(instruction_line: &str) -> Option<String> {
    let trimmed_instruction_string = instruction_line.trim_start().to_uppercase();

    // TODO: If there are multiple spaces between the instuctions and the operand this will fail.
    match trimmed_instruction_string.find(" ") {
        Some(value) => Some(trimmed_instruction_string[value+1..].to_string()),
        None => None
    }
}