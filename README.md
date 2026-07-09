# My VM & Compiler - v1.0

Este projeto implementa uma **Máquina Virtual** de arquitetura de registradores e um **Compilador Python** (`vm-compiler`) construídos do zero para traduzir um subconjunto estrito de código Python em um bytecode/assembly próprio.

## Estrutura do Projeto

- `/src`: O motor da Virtual Machine escrito em Rust.
- `/vm-compiler`: O compilador (transpilador) escrito em Python, usando `ast`.
- `/tests`: Exemplos de código testáveis e validáveis.
- `ISA.md`: Documentação Oficial do Conjunto de Instruções.
- `TODO.md`: Débitos Técnicos para a versão 2.

## Como Compilar e Rodar

O fluxo de trabalho envolve dois passos: compilar o código fonte `.py` para um `.asm`, e instruir a VM em Rust a executar o `.asm`.

```bash
# Passo 1: Compilar de Python para Assembly
python3 vm-compiler/main.py tests/test_loops.py target.asm

# Passo 2: Executar na VM
cargo run -- target.asm
```

## Escrevendo Programas

Sua sintaxe deve seguir um subconjunto limpo de Python, suportando `if`, `else`, `while`, `for i in range()` e operações aritméticas puras. 
Você não tem acesso à Biblioteca Padrão do Python (`import math`, etc). Tudo deve ser escrito manualmente ou utilizando dependências resolvidas localmente pelo Linker integrado.

### Exemplo de Programa (Múltiplos Módulos)
**`matematica.py`**
```python
def ao_quadrado(x):
    return x * x
```
**`main.py`**
```python
from myvm_lib import entry_point, print_str
import matematica

@entry_point
def main():
    resultado = matematica.ao_quadrado(5)
    print_str("O quadrado e:")
    print_str(str(resultado))
```

## Limitações Atuais (v1.0)
- Registradores limitados de `A` até `Z`.
- Sem suporte nativo do compilador para Tipos Complexos (Listas, Classes, Dicionários).
- Tratamento silencioso de módulos ausentes pelo Linker.
- Tamanho total da RAM restrito a 1024 words.

Para mais detalhes da arquitetura, consulte o documento `ISA.md`.
