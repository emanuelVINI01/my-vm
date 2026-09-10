#[derive(Debug)]
pub enum OpCode {
    SET,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    POW,
    XOR,
    LOG,
    FADD,
    FSUB,
    FMUL,
    FDIV,
    MOV,
    JMP,
    JZ,
    JGT,
    JLT,
    JEQ,
    JNE,
    JLE,
    JGE,
    CALL,
    RET,
    PUSH,
    POP,
    LOAD,
    STORE,
    PRINT,
    PRINTLN,
    PRINTCHAR,
    GETLASTADDR,
    WRITE,
    ITOA,
    DRAWPIXEL,
    UPDATEGUI,
    IN,
    OUT,
    CLI,
    STI,
    HLT,
    HALT,
    IRET,
    YIELD,
    GETSP,
    SETSP,
    // Novos opcodes para GUI avançada (OS desktop)
    FILLRECT,      // FILLRECT x y w h color  - Preenche retângulo com cor sólida
    DRAWRECT,      // DRAWRECT x y w h color  - Desenha borda do retângulo
    DRAWCHAR,      // DRAWCHAR x y char color bg_color  - Desenha caractere na posição pixel
    DRAWTEXT,      // DRAWTEXT x y addr len color bg_color  - Desenha string na posição pixel
    GETMOUSEX,     // GETMOUSEX reg  - Obtém coordenada X do mouse
    GETMOUSEY,     // GETMOUSEY reg  - Obtém coordenada Y do mouse
    GETMOUSEBTN,   // GETMOUSEBTN reg  - Obtém estado dos botões do mouse (bit 0=left, bit1=right)
    CLEARSCREEN,   // CLEARSCREEN color  - Limpa tela com cor
    COPYREGION,    // COPYREGION src_x src_y dst_x dst_y w h - Copia região da VRAM
    DRAWLINE,      // DRAWLINE x1 y1 x2 y2 color  - Desenha linha
    FILLROUNDRECT, // FILLROUNDRECT x y w h radius color  - Retângulo com cantos arredondados
    BLITCHAR,      // BLITCHAR x y char_code fg bg  - Desenha char 8x16 em pixel coords
    GETSCREENW,    // GETSCREENW reg  - Obtém largura da tela
    GETSCREENH,    // GETSCREENH reg  - Obtém altura da tela
    SETWINDOWTITLE, // SETWINDOWTITLE addr len  - Muda título da janela
    LABELADDR,     // LABELADDR reg, label  - Carrega o endereço (índice de instrução) de um label/função em reg
}