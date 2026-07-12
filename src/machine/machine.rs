use minifb::{Window, WindowOptions};

pub struct Machine {
    pub registers: [u32; 26],
    pub ram: Box<[u32]>,
    pub last_ram_address: u32,
    pub sp: usize,
    
    pub vram: Vec<u32>,
    pub window: Option<Window>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    
    pub interrupts_enabled: bool,
    pub pending_interrupts: Vec<usize>,
    pub io_ports: [u32; 1024],
    pub frame_count: usize,
}

impl Machine {
    // Cria uma nova máquina com registradores, RAM zerados e Janela Gráfica
    pub fn new() -> Self {
        let width = 1000;
        let height = 1000;
        let mut window = Window::new("My VM", width, height, WindowOptions::default())
            .expect("Falha ao criar janela GUI");
            
        // Limita a ~60 FPS
        window.set_target_fps(60);
        
        let mem_size = 256 * 1024 * 1024;
        let mut regs = [0; 26];
        regs[24] = (mem_size - 1) as u32; // Y = Base Pointer
        
        Machine {
            registers: regs,
            ram: vec![0; mem_size].into_boxed_slice(),
            last_ram_address: u32::MAX,
            sp: mem_size - 1,
            vram: vec![0; width * height],
            window: Some(window),
            cursor_x: 10,
            cursor_y: 10,
            interrupts_enabled: false,
            pending_interrupts: Vec::new(),
            io_ports: [0; 1024],
            frame_count: 0,
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
    
    // Métodos Gráficos
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < 1000 && y < 1000 {
            self.vram[y * 1000 + x] = color;
        }
    }
    
    pub fn update_gui(&mut self) {
        self.frame_count += 1;
        if let Some(window) = &mut self.window {
            // Draw hardware cursor
            let mut vram_copy = vec![0; 1000 * 1000];
            
            use font8x8::UnicodeFonts;
            
            // Renderiza 80x25 caracteres da RAM (VGA Buffer = 0xB8000 = 753664)
            for i in 0..(80 * 25) {
                let addr = 0xB8000 + i;
                let ch_val = self.ram[addr];
                if ch_val != 0 && ch_val != 0x20 {
                    if let Some(ch) = std::char::from_u32(ch_val & 0xFF) {
                        if let Some(bitmap) = font8x8::BASIC_FONTS.get(ch) {
                            let cx = (i % 80) * 8;
                            let cy = (i / 80) * 16;
                            for (r, row) in bitmap.iter().enumerate() {
                                for c in 0..8 {
                                    if (*row & 1 << c) != 0 {
                                        let px = cx + c;
                                        let py = cy + r;
                                        if px < 1000 && py < 1000 {
                                            vram_copy[py * 1000 + px] = 0xFFFFFFFF;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Lê posição do cursor (0..2000 para 80x25)
            let cursor_pos = self.io_ports[0x3D5] as usize;
            if cursor_pos < 80 * 25 {
                let cursor_x = (cursor_pos % 80) * 8;
                let cursor_y = (cursor_pos / 80) * 16;
                
                // Pisca a cada 30 frames (0.5s a 60FPS)
                if (self.frame_count / 30) % 2 == 0 {
                    for r in 14..16 { // desenha uma linha embaixo do caractere
                        for c in 0..8 {
                            let px = cursor_x + c;
                            let py = cursor_y + r;
                            if px < 1000 && py < 1000 {
                                vram_copy[py * 1000 + px] = 0xFFFFFFFF;
                            }
                        }
                    }
                }
            }
            
            window.update_with_buffer(&vram_copy, 1000, 1000).unwrap();
        }
    }
    
    // Impede o SO de travar a janela
    pub fn poll_events(&mut self) {
        self.update_gui();
        
        if let Some(window) = &mut self.window {
            // Timer Tick (32)
            if !self.pending_interrupts.contains(&32) {
                self.pending_interrupts.push(32);
            }
            
            let keys = window.get_keys_pressed(minifb::KeyRepeat::Yes);
            for key in keys {
                self.io_ports[0x60] = key as u32;
                if !self.pending_interrupts.contains(&33) {
                    self.pending_interrupts.push(33);
                }
            }
            
            let keys_released = window.get_keys_released();
            for key in keys_released {
                self.io_ports[0x60] = (key as u32) | 0x80;
                if !self.pending_interrupts.contains(&33) {
                    self.pending_interrupts.push(33);
                }
            }
        }
    }
    

}
