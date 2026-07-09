pub struct Machine {
    pub registers: [u32; 26],
    pub ram: [u32; 1024],
    pub last_ram_address: u32, // Guarda o último endereço escrito
}

impl Machine {
    // Cria uma nova máquina com registradores e RAM zerados
    pub fn new() -> Self {
        Machine {
            registers: [0; 26],
            ram: [0; 1024],
            last_ram_address: u32::MAX, // Iniciamos com -1 (MAX) pra que o próximo endereço some +1 e caia no 0.
        }
    }

    // Função auxiliar privada para converter a letra (A-Z) para um índice (0-25)
    // Não precisamos de nenhuma lib externa pra isso! O Rust já converte ASCII facilmente.
    fn get_index(address: &str) -> usize {
        // Pega a primeira letra (ex: 'A'), e garante que seja maiúscula
        let letra = address.chars().next().expect("Endereço não pode ser vazio").to_ascii_uppercase();
        
        // Verifica se é uma letra de A até Z
        if letra >= 'A' && letra <= 'Z' {
            // Em ASCII, 'A' é 65. Se fizermos Letra - 'A', temos o índice!
            // Ex: 'A' (65) - 'A' (65) = 0
            // Ex: 'B' (66) - 'A' (65) = 1
            // Ex: 'Z' (90) - 'A' (65) = 25
            (letra as u8 - b'A') as usize
        } else {
            panic!("Endereço de registrador inválido: {}. Use letras de A a Z.", address);
        }
    }

    // Define (seta) o valor no registrador daquela letra
    pub fn set(&mut self, address: &str, value: u32) {
        let index = Self::get_index(address);
        self.registers[index] = value;
    }

    // Pega o valor que está no registrador daquela letra
    pub fn get(&self, address: &str) -> u32 {
        let index = Self::get_index(address);
        self.registers[index]
    }

    // Escreve um valor na RAM
    pub fn write_ram(&mut self, address: u32, value: u32) {
        if (address as usize) < self.ram.len() {
            self.ram[address as usize] = value;
            self.last_ram_address = address; // Atualizamos o rastreador
        } else {
            panic!("Segfault: Tentativa de escrita em memória fora dos limites da RAM (endereço {})", address);
        }
    }

    // Lê um valor da RAM
    pub fn read_ram(&self, address: u32) -> u32 {
        if (address as usize) < self.ram.len() {
            self.ram[address as usize]
        } else {
            panic!("Segfault: Tentativa de leitura em memória fora dos limites da RAM (endereço {})", address);
        }
    }
}
