use crate::instruction::Instruction;
use crate::instruction::Value;
use crate::machine::machine::Machine;
use crate::opcodes::OpCode;
use std::collections::HashMap;

pub fn execute(
    instructions: Vec<Instruction>,
    labels: HashMap<String, usize>,
    machine: &mut Machine
) {
    // Começa a execução pela label "main"
    let mut pc = *labels.get("main").expect("Erro: A label 'main:' é obrigatória e não foi encontrada!");

    while pc < instructions.len() {
        let instruction = &instructions[pc];
        pc += 1; // Avança o PC para a próxima instrução
        
        match instruction.op_code {
            OpCode::SET => {
                if let [Value::Address(addr), Value::Value(val)] = &instruction.values[..] {
                    machine.set(addr, *val);
                }
            }
            OpCode::ADD => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = machine.get(dest);
                    let val_src = machine.get(src);
                    machine.set(dest, val_dest.wrapping_add(val_src));
                }
            }
            OpCode::SUB => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = machine.get(dest);
                    let val_src = machine.get(src);
                    machine.set(dest, val_dest.wrapping_sub(val_src));
                }
            }
            OpCode::MUL => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = machine.get(dest);
                    let val_src = machine.get(src);
                    machine.set(dest, val_dest.wrapping_mul(val_src));
                }
            }
            OpCode::DIV => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = machine.get(dest);
                    let val_src = machine.get(src);
                    machine.set(dest, val_dest / val_src);
                }
            }
            OpCode::MOD => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = machine.get(dest);
                    let val_src = machine.get(src);
                    machine.set(dest, val_dest % val_src);
                }
            }
            OpCode::POW => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = machine.get(dest);
                    let val_src = machine.get(src);
                    machine.set(dest, val_dest.wrapping_pow(val_src));
                }
            }
            OpCode::XOR => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = machine.get(dest);
                    let val_src = machine.get(src);
                    machine.set(dest, val_dest ^ val_src);
                }
            }
            OpCode::LOG => {
                if let [Value::Address(addr)] = &instruction.values[..] {
                    println!("{}: {}", addr, machine.get(addr));
                }
            }
            OpCode::JMP => {
                if let [Value::Address(target)] = &instruction.values[..] {
                    // Tenta ler como label, se não achar, tenta ler como registrador
                    if let Some(&index) = labels.get(target) {
                        pc = index;
                    } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                        pc = machine.get(target) as usize;
                    } else {
                        panic!("Alvo de JMP '{}' não é nem uma Label nem um Registrador válido.", target);
                    }
                }
            }
            OpCode::JZ => {
                // Sintaxe esperada: JZ <RegistradorTeste>, <Label ou RegistradorAlvo>
                if let [Value::Address(reg), Value::Address(target)] = &instruction.values[..] {
                    if machine.get(reg) == 0 {
                        if let Some(&index) = labels.get(target) {
                            pc = index;
                        } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                            pc = machine.get(target) as usize;
                        } else {
                            panic!("Alvo de JZ '{}' não é nem uma Label nem um Registrador válido.", target);
                        }
                    }
                }
            }
            OpCode::JGT => {
                if let [Value::Address(reg1), val2, Value::Address(target)] = &instruction.values[..] {
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if machine.get(reg1) > v2 {
                        if let Some(&index) = labels.get(target) {
                            pc = index;
                        } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                            pc = machine.get(target) as usize;
                        } else {
                            panic!("Alvo de JGT '{}' inválido.", target);
                        }
                    }
                }
            }
            OpCode::JLT => {
                if let [Value::Address(reg1), val2, Value::Address(target)] = &instruction.values[..] {
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if machine.get(reg1) < v2 {
                        if let Some(&index) = labels.get(target) {
                            pc = index;
                        } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                            pc = machine.get(target) as usize;
                        } else {
                            panic!("Alvo de JLT '{}' inválido.", target);
                        }
                    }
                }
            }
            OpCode::JEQ => {
                if let [Value::Address(reg1), val2, Value::Address(target)] = &instruction.values[..] {
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if machine.get(reg1) == v2 {
                        if let Some(&index) = labels.get(target) {
                            pc = index;
                        } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                            pc = machine.get(target) as usize;
                        } else {
                            panic!("Alvo de JEQ '{}' inválido.", target);
                        }
                    }
                }
            }
            OpCode::JNE => {
                if let [Value::Address(reg1), val2, Value::Address(target)] = &instruction.values[..] {
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if machine.get(reg1) != v2 {
                        if let Some(&index) = labels.get(target) {
                            pc = index;
                        } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                            pc = machine.get(target) as usize;
                        } else {
                            panic!("Alvo de JNE '{}' inválido.", target);
                        }
                    }
                }
            }
            OpCode::CALL => {
                if let [Value::Address(target)] = &instruction.values[..] {
                    machine.set("Z", pc as u32);
                    if let Some(&index) = labels.get(target) {
                        pc = index;
                    } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                        pc = machine.get(target) as usize;
                    } else {
                        panic!("Alvo de CALL '{}' inválido.", target);
                    }
                }
            }
            OpCode::WRITE => {
                if let [fd_val, Value::Address(addr_reg), len_val] = &instruction.values[..] {
                    let fd = match fd_val {
                        Value::Value(v) => *v,
                        Value::Address(reg) => machine.get(reg),
                        Value::String(_) => panic!("FD inválido"),
                    };
                    let start_addr = machine.get(addr_reg);
                    let length = match len_val {
                        Value::Value(v) => *v,
                        Value::Address(reg) => machine.get(reg),
                        Value::String(_) => panic!("Length inválido"),
                    };
                    
                    if fd == 1 {
                        for i in 0..length {
                            let ch_val = machine.read_ram(start_addr + i);
                            if let Some(c) = std::char::from_u32(ch_val) {
                                print!("{}", c);
                            } else {
                                print!("?");
                            }
                        }
                    }
                }
            }
            OpCode::ITOA => {
                // Sintaxe: ITOA <RegNumero>, <RegEnderecoDestino>, <RegTamanhoDestino>
                if let [Value::Address(num_reg), Value::Address(addr_reg), Value::Address(len_reg)] = &instruction.values[..] {
                    let num = machine.get(num_reg);
                    let start_addr = machine.get(addr_reg);
                    
                    let s = num.to_string();
                    let len = s.len() as u32;
                    
                    for (i, ch) in s.chars().enumerate() {
                        machine.write_ram(start_addr + (i as u32), ch as u32);
                    }
                    
                    machine.set(len_reg, len);
                } else {
                    panic!("ITOA requer 3 argumentos (registradores)");
                }
            }
            OpCode::LOAD => {
                // Sintaxe: LOAD <RegistradorDestino>, <Registrador/Endereço>
                if let [Value::Address(dest_reg), ptr_val] = &instruction.values[..] {
                    let addr = match ptr_val {
                        Value::Address(reg) => machine.get(reg), // Lê o endereço que está dentro do registrador
                        Value::Value(val) => *val,               // Lê o endereço direto
                        Value::String(_) => panic!("String não pode ser usada como endereço!"),
                    };
                    let val = machine.read_ram(addr);
                    machine.set(dest_reg, val);
                }
            }
            OpCode::STORE => {
                // Sintaxe: STORE <Registrador/Endereço>, <RegistradorOrigem>
                if let [ptr_val, Value::Address(src_reg)] = &instruction.values[..] {
                    let addr = match ptr_val {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(val) => *val,
                        Value::String(_) => panic!("String não pode ser usada como endereço!"),
                    };
                    let val = machine.get(src_reg);
                    machine.write_ram(addr, val);
                }
            }
            OpCode::PRINT => {
                // Sintaxe: PRINT <Registrador ou ValorDireto>
                if let [val] = &instruction.values[..] {
                    match val {
                        Value::Address(addr) => println!("{}", machine.get(addr)), // Printa o valor no registrador
                        Value::Value(v) => println!("{}", v),                      // Printa o número fixo
                        Value::String(s) => println!("{}", s),                     // Suporte bônus: print de string pura
                    }
                }
            }
            OpCode::PRINTCHAR => {
                // Sintaxe: PRINTCHAR <Registrador ou ValorDireto>
                if let [val] = &instruction.values[..] {
                    let number = match val {
                        Value::Address(addr) => machine.get(addr),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não pode ser usada no PRINTCHAR!"),
                    };
                    
                    // Converte de u32 para char (ASCII/Unicode) e dá print sem pular linha
                    if let Some(c) = std::char::from_u32(number) {
                        print!("{}", c);
                    } else {
                        print!("?"); // Caractere inválido
                    }
                }
            }
            OpCode::WRITESTR => {
                // Sintaxe: WRITESTR "Texto"
                if let [Value::String(text)] = &instruction.values[..] {
                    for ch in text.chars() {
                        // Calcula qual o próximo endereço disponível
                        let next_addr = machine.last_ram_address.wrapping_add(1);
                        // Ao escrever, a função write_ram já atualiza o last_ram_address!
                        machine.write_ram(next_addr, ch as u32);
                    }
                }
            }
            OpCode::GETLASTADDR => {
                // Sintaxe: GETLASTADDR <RegistradorDestino>
                if let [Value::Address(dest_reg)] = &instruction.values[..] {
                    machine.set(dest_reg, machine.last_ram_address);
                }
            }
            OpCode::HALT => {
                break; // Para a execução da máquina
            }
        }
    }
}