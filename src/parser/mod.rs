use crate::instruction::{Instruction, Value};
use crate::opcodes::OpCode;
use std::collections::HashMap;

pub fn parse(input: &str) -> (Vec<Instruction>, HashMap<String, usize>) {
    let mut instructions = Vec::new();
    let mut labels = HashMap::new();
    
    let mut clean_input = String::new();
    for line in input.lines() {
        if let Some(idx) = line.find("//") {
            clean_input.push_str(&line[..idx]);
        } else {
            clean_input.push_str(line);
        }
        clean_input.push('\n');
    }
    
    for statement in clean_input.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }

        let mut string_literal = None;
        let mut statement_string = statement.to_string();

        if let Some(start) = statement_string.find('"') {
            if let Some(end) = statement_string[start + 1..].find('"') {
                string_literal = Some(statement_string[start + 1..start + 1 + end].to_string());
                statement_string.replace_range(start..start + end + 2, "");
            }
        }

        let trimmed = statement_string.trim().to_string();

        if let Some(idx) = trimmed.find(':') {
            let label_name = trimmed[..idx].trim().to_string();
            labels.insert(label_name, instructions.len());
            statement_string = trimmed[idx + 1..].trim().to_string();
            if statement_string.is_empty() {
                continue;
            }
        } else {
            statement_string = trimmed;
        }

        let cleaned_statement = statement_string.replace(',', " ");
        let parts: Vec<&str> = cleaned_statement.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }

        let op_code = match parts[0].to_uppercase().as_str() {
            "SET" => OpCode::SET,
            "ADD" => OpCode::ADD,
            "SUB" => OpCode::SUB,
            "MUL" => OpCode::MUL,
            "DIV" => OpCode::DIV,
            "MOD" => OpCode::MOD,
            "POW" => OpCode::POW,
            "XOR" => OpCode::XOR,
            "LOG" => OpCode::LOG,
            "JMP" => OpCode::JMP,
            "JZ" => OpCode::JZ,
            "JGT" => OpCode::JGT,
            "JLT" => OpCode::JLT,
            "JEQ" => OpCode::JEQ,
            "JNE" => OpCode::JNE,
            "CALL" => OpCode::CALL,
            "LOAD" => OpCode::LOAD,
            "STORE" => OpCode::STORE,
            "PRINT" => OpCode::PRINT,
            "PRINTCHAR" => OpCode::PRINTCHAR,
            "WRITESTR" => OpCode::WRITESTR,
            "GETLASTADDR" => OpCode::GETLASTADDR,
            "WRITE" => OpCode::WRITE,
            "ITOA" => OpCode::ITOA,
            "DRAWPIXEL" => OpCode::DRAWPIXEL,
            "UPDATEGUI" => OpCode::UPDATEGUI,
            "HALT" => OpCode::HALT,
            _ => panic!("Unknown opcode: {}", parts[0]),
        };

        let mut values = Vec::new();
        
        if let Some(text) = string_literal {
            values.push(Value::String(text));
        }

        for &part in &parts[1..] {
            if part.starts_with("0b") {
                let num = u32::from_str_radix(&part[2..], 2).expect("Número binário inválido");
                values.push(Value::Value(num));
            } else if part.chars().all(|c| c.is_digit(10)) {
                let num = part.parse::<u32>().unwrap();
                values.push(Value::Value(num));
            } else {
                values.push(Value::Address(part.to_string()));
            }
        }

        instructions.push(Instruction {
            op_code,
            values,
        });
    }

    (instructions, labels)
}