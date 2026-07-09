use my_vm::parser;

use std::env;
use std::fs;

fn main() {
    // Pega os argumentos passados no CLI
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} <caminho_do_arquivo>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    
    // Lê o arquivo
    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Erro ao ler o arquivo '{}': {}", file_path, err);
        std::process::exit(1);
    });

    // Roda o parser e pega a lista de instruções e o mapa de labels
    let (instructions, labels) = parser::parse(&content);

    // Cria a máquina virtual zerada
    let mut machine = my_vm::machine::machine::Machine::new();

    // Executa as instruções na máquina
    my_vm::machine::executor::execute(instructions, labels, &mut machine);
}
