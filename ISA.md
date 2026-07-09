# Instruction Set Architecture (ISA) - v1.0

Este documento descreve a ISA da Virtual Machine customizada.

## Especificações de Hardware
- **Registradores:** 26 registradores de Propósito Geral, endereçados pelas letras `A` a `Z`.
- **Palavra da Máquina:** Todos os valores e registradores são baseados em Inteiros sem sinal de 32-bits (`u32`). Overflow matemático aciona wrap-around nativo.
- **Memória RAM:** 1024 palavras de 32-bits (`[u32; 1024]`).

## Regras de Sintaxe
- O compilador encerra cada instrução com um ponto-e-vírgula (`;`).
- Labels são definidos com a sintaxe `NOME: ;`.
- Espaços separam a instrução dos argumentos. Não há suporte oficial para vírgulas na separação de argumentos pelo parser em Rust (embora elas sejam sanitizadas).

## Lista de Opcodes

### Aritméticos
As operações aritméticas armazenam o resultado de volta no Registrador de Destino.
- `SET <RegDestino> <Valor/Reg>`: Define o valor de um registrador.
- `ADD <RegDestino> <Valor/Reg>`: Adiciona ao registrador.
- `SUB <RegDestino> <Valor/Reg>`: Subtrai do registrador.
- `MUL <RegDestino> <Valor/Reg>`: Multiplica o registrador.
- `DIV <RegDestino> <Valor/Reg>`: Divide o registrador.
- `MOD <RegDestino> <Valor/Reg>`: Aplica operação de Módulo.
- `POW <RegDestino> <Valor/Reg>`: Eleva à potência.
- `XOR <RegDestino> <Valor/Reg>`: Ou Exclusivo (Bitwise XOR).
- `LOG <RegDestino> <Valor/Reg>`: Logaritmo na base especificada.

### Desvios e Controle de Fluxo
- `JMP <Label>`: Pulo incondicional.
- `JZ <Reg> <Label>`: Pula se o registrador for zero.
- `JEQ <Reg1> <Reg2> <Label>`: Pula se Reg1 for igual a Reg2.
- `JNE <Reg1> <Reg2> <Label>`: Pula se Reg1 for diferente de Reg2.
- `JLT <Reg1> <Reg2> <Label>`: Pula se Reg1 for menor que Reg2.
- `JGT <Reg1> <Reg2> <Label>`: Pula se Reg1 for maior que Reg2.
- `CALL <Label>`: Chamada de função, empilha o endereço de retorno.
- `HALT`: Encerra a execução da VM.

### Memória e E/S
- `LOAD <RegDestino> <Reg/Endereco>`: Lê uma palavra da RAM para o registrador.
- `STORE <Reg/Endereco> <RegOrigem>`: Escreve o conteúdo do registrador na RAM.
- `PRINT <Reg/Valor>`: Imprime o número no STDOUT (com quebra de linha).
- `PRINTCHAR <Reg/Valor>`: Imprime como caractere ASCII (sem quebra de linha).
- `WRITESTR "String"`: Escreve a string literal na memória RAM.
- `GETLASTADDR <RegDestino>`: Obtém o último endereço de RAM escrito e armazena no registrador.
- `WRITE <FD> <RegInicio> <RegTamanho>`: Syscall de I/O. Escreve `Tamanho` bytes a partir da RAM no File Descriptor (geralmente `1` para STDOUT).
- `ITOA <RegNumero> <RegDestino> <RegTamanho>`: Syscall para conversão Integer-to-ASCII em memória.

## Interface Gráfica (GUI) - v1.1
A VM possui uma VRAM interna de tamanho `1000x1000` (1 milhão de pixels). A janela gráfica é inicializada automaticamente em modo de Hardware, e a VM desenha nessa tela de forma síncrona.

| OpCode       | Argumentos                     | Descrição                                                                                             |
|--------------|--------------------------------|-------------------------------------------------------------------------------------------------------|
| `DRAWPIXEL`  | `<X>, <Y>, <CorARGB>`         | Pinta o pixel nas coordenadas X e Y da VRAM com a Cor especificada.                                   |
| `UPDATEGUI`  | Nenhum                         | Faz o *Swap Buffers* transferindo tudo o que foi pintado na VRAM diretamente para a Janela do OS.      |

**Aviso Textual:** Os Opcodes `WRITE` e `PRINTCHAR` foram sobrescritos no nível da VM para converter os Caracteres ASCII em Bitmaps (usando `font8x8`) e renderizar as letras diretamente na interface gráfica, como um console de BIOS antigo.
