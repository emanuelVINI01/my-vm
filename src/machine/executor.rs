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
    let mut pc = 0;
    println!("INICIO: SP={} Y={}", machine.sp, machine.get("Y"));
    let mut cycle_count = 0;

    while pc < instructions.len() {
        cycle_count += 1;
        if cycle_count % 10000 == 0 {
            machine.poll_events();
        }
        
        // Dispatcher de Interrupções
        if machine.interrupts_enabled && !machine.pending_interrupts.is_empty() {
            let int_num = machine.pending_interrupts.remove(0);
            let handler_addr = machine.read_ram(int_num as u32); // Lê o vetor de interrupção (0..255)
            
            if handler_addr != 0 {
                // Empurra o PC atual na pilha
                machine.sp = machine.sp.wrapping_sub(1);
                machine.write_ram(machine.sp as u32, pc as u32);
                
                // Pula para o handler
                pc = handler_addr as usize;
                
                // Desativa interrupções até o IRET
                machine.interrupts_enabled = false;
                continue; // Vai pro próximo ciclo no novo PC
            }
        }
        let instruction = &instructions[pc];
        pc += 1; // Avança o PC para a próxima instrução
        
        match instruction.op_code {
            OpCode::SET => {
                if let [Value::Address(dest), val] = &instruction.values[..] {
                    let v = match val {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(n) => *n,
                        Value::String(_) => panic!("Strings não podem ser definidas com SET"),
                    };
                    machine.set(dest, v);
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
                    let res = val_dest.wrapping_sub(val_src);
                    machine.set(dest, res);
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
            OpCode::FADD => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = f32::from_bits(machine.get(dest));
                    let val_src = f32::from_bits(machine.get(src));
                    machine.set(dest, (val_dest + val_src).to_bits());
                }
            }
            OpCode::FSUB => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = f32::from_bits(machine.get(dest));
                    let val_src = f32::from_bits(machine.get(src));
                    machine.set(dest, (val_dest - val_src).to_bits());
                }
            }
            OpCode::FMUL => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = f32::from_bits(machine.get(dest));
                    let val_src = f32::from_bits(machine.get(src));
                    machine.set(dest, (val_dest * val_src).to_bits());
                }
            }
            OpCode::FDIV => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let val_dest = f32::from_bits(machine.get(dest));
                    let val_src = f32::from_bits(machine.get(src));
                    machine.set(dest, (val_dest / val_src).to_bits());
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
                if let [val1, val2, Value::Address(target)] = &instruction.values[..] {
                    let v1 = match val1 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if v1 > v2 {
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
                if let [val1, val2, Value::Address(target)] = &instruction.values[..] {
                    let v1 = match val1 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if v1 < v2 {
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
                if let [val1, val2, Value::Address(target)] = &instruction.values[..] {
                    let v1 = match val1 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if v1 == v2 {
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
                if let [val1, val2, Value::Address(target)] = &instruction.values[..] {
                    let v1 = match val1 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if v1 != v2 {
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
            OpCode::JLE => {
                if let [val1, val2, Value::Address(target)] = &instruction.values[..] {
                    let v1 = match val1 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if v1 <= v2 {
                        if let Some(&index) = labels.get(target) {
                            pc = index;
                        } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                            pc = machine.get(target) as usize;
                        } else {
                            panic!("Alvo de JLE '{}' inválido.", target);
                        }
                    }
                }
            }
            OpCode::JGE => {
                if let [val1, val2, Value::Address(target)] = &instruction.values[..] {
                    let v1 = match val1 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    let v2 = match val2 {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        Value::String(_) => panic!("String não permitida"),
                    };
                    if v1 >= v2 {
                        if let Some(&index) = labels.get(target) {
                            pc = index;
                        } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                            pc = machine.get(target) as usize;
                        } else {
                            panic!("Alvo de JGE '{}' inválido.", target);
                        }
                    }
                }
            }
            OpCode::CALL => {
                if let [Value::Address(target)] = &instruction.values[..] {
                    // PUSH pc na pilha
                    machine.sp = machine.sp.wrapping_sub(1);
                    machine.write_ram(machine.sp as u32, pc as u32);
                    
                    if let Some(&index) = labels.get(target) {
                        pc = index;
                    } else if target.len() == 1 && target.chars().next().unwrap().is_ascii_alphabetic() {
                        pc = machine.get(target) as usize;
                    } else {
                        panic!("Alvo de CALL '{}' inválido.", target);
                    }
                }
            }
            OpCode::RET => {
                // POP pc da pilha
                let ret_pc = machine.read_ram(machine.sp as u32);
                machine.sp = machine.sp.wrapping_add(1);
                pc = ret_pc as usize;
            }
            OpCode::PUSH => {
                if let [val] = &instruction.values[..] {
                    let v = match val {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(n) => *n,
                        Value::String(_) => panic!("Não pode dar push em string"),
                    };
                    machine.sp = machine.sp.wrapping_sub(1);
                    machine.write_ram(machine.sp as u32, v);
                }
            }
            OpCode::POP => {
                if let [Value::Address(reg)] = &instruction.values[..] {
                    let v = machine.read_ram(machine.sp as u32);
                    machine.sp = machine.sp.wrapping_add(1);
                    machine.set(reg, v);
                } else {
                    panic!("POP requer um registrador de destino");
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
            OpCode::MOV => {
                if let [Value::Address(dest), Value::Address(src)] = &instruction.values[..] {
                    let v = machine.get(src);
                    machine.set(dest, v);
                } else {
                    panic!("MOV requer dois registradores: MOV dest, src");
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
                        Value::Address(addr) => print!("{}", machine.get(addr)), // Printa o valor no registrador
                        Value::Value(v) => print!("{}", v),                      // Printa o número fixo
                        Value::String(s) => print!("{}", s),                     // Suporte bônus: print de string pura
                    }
                    use std::io::Write;
                    let _ = std::io::stdout().flush();
                }
            }
            OpCode::PRINTLN => {
                if let [val] = &instruction.values[..] {
                    match val {
                        Value::Address(addr) => println!("{}", machine.get(addr)),
                        Value::Value(v) => println!("{}", v),
                        Value::String(s) => {
                            if s == "\"\"" {
                                println!();
                            } else {
                                println!("{}", s);
                            }
                        }
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
                    
                    // Converte de u32 para char (ASCII/Unicode) e exibe no terminal do host
                    if let Some(c) = std::char::from_u32(number) {
                        print!("{}", c);
                    } else {
                        print!("?");
                    }
                }
            }

            OpCode::GETLASTADDR => {
                // Sintaxe: GETLASTADDR <RegistradorDestino>
                if let [Value::Address(dest_reg)] = &instruction.values[..] {
                    machine.set(dest_reg, machine.last_ram_address);
                }
            }
            OpCode::GETSP => {
                if let [Value::Address(dest_reg)] = &instruction.values[..] {
                    machine.set(dest_reg, machine.sp as u32);
                }
            }
            OpCode::SETSP => {
                if let [Value::Address(src_reg)] = &instruction.values[..] {
                    machine.sp = machine.get(src_reg) as usize;
                } else if let [Value::Value(val)] = &instruction.values[..] {
                    machine.sp = *val as usize;
                }
            }
            OpCode::IN => {
                if let [Value::Address(dest_reg), port_val] = &instruction.values[..] {
                    let port = match port_val {
                        Value::Value(v) => *v as usize,
                        Value::Address(reg) => machine.get(reg) as usize,
                        _ => panic!("Porta inválida"),
                    };
                    let value = if port < machine.io_ports.len() {
                        machine.io_ports[port]
                    } else {
                        0xFFFF
                    };
                    machine.set(dest_reg, value);
                }
            }
            OpCode::OUT => {
                if let [port_val, val] = &instruction.values[..] {
                    let port = match port_val {
                        Value::Value(v) => *v as usize,
                        Value::Address(reg) => machine.get(reg) as usize,
                        _ => panic!("Porta inválida"),
                    };
                    let value = match val {
                        Value::Value(v) => *v,
                        Value::Address(reg) => machine.get(reg),
                        _ => panic!("Valor inválido"),
                    };
                    if port < machine.io_ports.len() {
                        machine.io_ports[port] = value;
                    }
                }
            }
            OpCode::CLI => {
                machine.interrupts_enabled = false;
            }
            OpCode::STI => {
                machine.interrupts_enabled = true;
            }
            OpCode::HLT => {
                if machine.interrupts_enabled {
                    pc -= 1;
                    machine.poll_events();
                } else {
                    break;
                }
            }
            OpCode::IRET => {
                let ret_pc = machine.read_ram(machine.sp as u32);
                machine.sp = machine.sp.wrapping_add(1);
                pc = ret_pc as usize;
                machine.interrupts_enabled = true;
            }
            OpCode::YIELD => {
                pc -= 1;
                machine.poll_events();
            }

            OpCode::HALT => {
                break;
            }
            OpCode::DRAWPIXEL => {
                if let [x_val, y_val, c_val] = &instruction.values[..] {
                    let x = match x_val {
                        Value::Address(reg) => machine.get(reg) as usize,
                        Value::Value(v) => *v as usize,
                        _ => panic!("X invalido"),
                    };
                    let y = match y_val {
                        Value::Address(reg) => machine.get(reg) as usize,
                        Value::Value(v) => *v as usize,
                        _ => panic!("Y invalido"),
                    };
                    let color = match c_val {
                        Value::Address(reg) => machine.get(reg),
                        Value::Value(v) => *v,
                        _ => panic!("Cor invalida"),
                    };
                    machine.draw_pixel(x, y, color);
                }
            }
            OpCode::UPDATEGUI => {
                machine.update_gui();
            }
        }
    }
}