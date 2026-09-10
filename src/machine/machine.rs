use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions, Scale};

pub const SCREEN_WIDTH: usize = 1280;
pub const SCREEN_HEIGHT: usize = 800;

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
    
    // Mouse state
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_buttons: u32,  // bit0=left, bit1=right, bit2=middle
    pub mouse_click: u32,    // 1 se clicou neste frame
}

impl Machine {
    pub fn new() -> Self {
        let width = SCREEN_WIDTH;
        let height = SCREEN_HEIGHT;
        
        let mut opts = WindowOptions::default();
        opts.scale = Scale::X1;
        opts.resize = false;
        
        let mut window = Window::new("ZorinVM OS", width, height, opts)
            .expect("Falha ao criar janela GUI");
            
        window.set_target_fps(60);
        
        let mem_size = 256 * 1024 * 1024;
        let mut regs = [0; 26];
        regs[24] = (mem_size - 1) as u32; // Y = Base Pointer
        
        Machine {
            registers: regs,
            ram: vec![0; mem_size].into_boxed_slice(),
            last_ram_address: u32::MAX,
            sp: mem_size - 1,
            vram: vec![0xFF1A1A2E; width * height],
            window: Some(window),
            cursor_x: 10,
            cursor_y: 10,
            interrupts_enabled: false,
            pending_interrupts: Vec::new(),
            io_ports: [0; 1024],
            frame_count: 0,
            mouse_x: 0,
            mouse_y: 0,
            mouse_buttons: 0,
            mouse_click: 0,
        }
    }

    fn get_index(address: &str) -> usize {
        let letra = address.chars().next().expect("Endereço não pode ser vazio").to_ascii_uppercase();
        if letra >= 'A' && letra <= 'Z' {
            (letra as u8 - b'A') as usize
        } else {
            panic!("Endereço de registrador inválido: {}. Use letras de A a Z.", address);
        }
    }

    pub fn set(&mut self, address: &str, value: u32) {
        let index = Self::get_index(address);
        self.registers[index] = value;
    }

    pub fn get(&self, address: &str) -> u32 {
        let index = Self::get_index(address);
        self.registers[index]
    }

    pub fn write_ram(&mut self, address: u32, value: u32) {
        if (address as usize) < self.ram.len() {
            self.ram[address as usize] = value;
            self.last_ram_address = address;
        } else {
            panic!("Segfault: Tentativa de escrita em memória fora dos limites da RAM (endereço {})", address);
        }
    }

    pub fn read_ram(&self, address: u32) -> u32 {
        if (address as usize) < self.ram.len() {
            self.ram[address as usize]
        } else {
            panic!("Segfault: Tentativa de leitura em memória fora dos limites da RAM (endereço {})", address);
        }
    }
    
    // === MÉTODOS GRÁFICOS BÁSICOS ===
    
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            // Converte ARGB -> BGRA que minifb espera
            self.vram[y * SCREEN_WIDTH + x] = color;
        }
    }
    
    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            self.vram[y * SCREEN_WIDTH + x]
        } else {
            0
        }
    }
    
    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let x0 = x.max(0) as usize;
        let y0 = y.max(0) as usize;
        let x1 = (x + w).min(SCREEN_WIDTH as i32) as usize;
        let y1 = (y + h).min(SCREEN_HEIGHT as i32) as usize;
        
        for py in y0..y1 {
            for px in x0..x1 {
                self.vram[py * SCREEN_WIDTH + px] = color;
            }
        }
    }
    
    pub fn draw_rect_border(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        // Top
        self.fill_rect(x, y, w, 1, color);
        // Bottom
        self.fill_rect(x, y + h - 1, w, 1, color);
        // Left
        self.fill_rect(x, y, 1, h, color);
        // Right
        self.fill_rect(x + w - 1, y, 1, h, color);
    }
    
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1i32 } else { -1i32 };
        let sy = if y0 < y1 { 1i32 } else { -1i32 };
        let mut err = dx - dy;
        let mut cx = x0;
        let mut cy = y0;
        
        loop {
            if cx >= 0 && cy >= 0 && (cx as usize) < SCREEN_WIDTH && (cy as usize) < SCREEN_HEIGHT {
                self.vram[cy as usize * SCREEN_WIDTH + cx as usize] = color;
            }
            if cx == x1 && cy == y1 { break; }
            let e2 = 2 * err;
            if e2 > -dy { err -= dy; cx += sx; }
            if e2 < dx { err += dx; cy += sy; }
        }
    }
    
    /// Desenha um caractere 8x16 na posição pixel (px, py) com cores fg e bg
    pub fn draw_char_pixel(&mut self, px: i32, py: i32, ch: char, fg: u32, bg: u32) {
        use font8x8::UnicodeFonts;
        let bitmap_ch = if ch.is_ascii() { ch } else { '?' };
        if let Some(bitmap) = font8x8::BASIC_FONTS.get(bitmap_ch) {
            for (row, &byte) in bitmap.iter().enumerate() {
                // Scale 8x8 font to 8x16 by doubling rows
                for scale in 0..2usize {
                    let screen_y = py + (row * 2 + scale) as i32;
                    for col in 0..8usize {
                        let screen_x = px + col as i32;
                        if screen_x >= 0 && screen_y >= 0 
                           && (screen_x as usize) < SCREEN_WIDTH 
                           && (screen_y as usize) < SCREEN_HEIGHT {
                            let pixel_on = (byte >> col) & 1 != 0;
                            let color = if pixel_on { fg } else { bg };
                            // Se bg for 0 (transparente), só desenha o fg
                            if color != 0 || pixel_on {
                                self.vram[screen_y as usize * SCREEN_WIDTH + screen_x as usize] = color;
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Desenha uma string na posição pixel
    pub fn draw_text_pixel(&mut self, px: i32, py: i32, text: &str, fg: u32, bg: u32) {
        let mut cx = px;
        for ch in text.chars() {
            if ch == '\n' {
                cx = px;
                // Avanço vertical tratado externamente
                continue;
            }
            self.draw_char_pixel(cx, py, ch, fg, bg);
            cx += 8;
        }
    }
    
    /// Desenha retângulo com cantos arredondados
    pub fn fill_round_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, color: u32) {
        // Preenche parte central
        self.fill_rect(x + r, y, w - 2 * r, h, color);
        self.fill_rect(x, y + r, r, h - 2 * r, color);
        self.fill_rect(x + w - r, y + r, r, h - 2 * r, color);
        
        // Quatro cantos usando círculo
        let corners = [(x + r, y + r), (x + w - r - 1, y + r), 
                       (x + r, y + h - r - 1), (x + w - r - 1, y + h - r - 1)];
        for (cx, cy) in corners {
            for dy in 0..=r {
                for dx in 0..=r {
                    if dx * dx + dy * dy <= r * r {
                        let px = cx + dx - r;
                        let py_val = cy + dy - r;
                        if px >= 0 && py_val >= 0 && (px as usize) < SCREEN_WIDTH && (py_val as usize) < SCREEN_HEIGHT {
                            self.vram[py_val as usize * SCREEN_WIDTH + px as usize] = color;
                        }
                    }
                }
            }
        }
    }
    
    pub fn update_gui(&mut self) {
        self.frame_count += 1;
        if let Some(window) = &mut self.window {
            window.update_with_buffer(&self.vram, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
        }
    }
    
    pub fn poll_events(&mut self) {
        self.update_gui();
        self.mouse_click = 0;
        
        if let Some(window) = &mut self.window {
            // Mouse position
            if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
                self.mouse_x = mx as i32;
                self.mouse_y = my as i32;
            }
            
            // Mouse buttons
            let prev_buttons = self.mouse_buttons;
            let mut new_buttons = 0u32;
            if window.get_mouse_down(MouseButton::Left)  { new_buttons |= 1; }
            if window.get_mouse_down(MouseButton::Right) { new_buttons |= 2; }
            if window.get_mouse_down(MouseButton::Middle) { new_buttons |= 4; }
            self.mouse_buttons = new_buttons;
            
            // Detecta click (transição 0->1 do botão esquerdo)
            if (new_buttons & 1) != 0 && (prev_buttons & 1) == 0 {
                self.mouse_click = 1;
            }
            
            // Timer Tick (32)
            if !self.pending_interrupts.contains(&32) {
                self.pending_interrupts.push(32);
            }
            
            // Teclado
            let keys = window.get_keys_pressed(minifb::KeyRepeat::Yes);
            for key in keys {
                self.io_ports[0x60] = key as u32;
                // Alt+Tab = key Tab (23) com Alt pressionado
                if key == Key::Tab {
                    let alt_held = window.is_key_down(Key::LeftAlt) || window.is_key_down(Key::RightAlt);
                    if alt_held {
                        self.io_ports[0x61] = 1; // sinaliza Alt+Tab
                        if !self.pending_interrupts.contains(&34) {
                            self.pending_interrupts.push(34);
                        }
                    }
                }
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
