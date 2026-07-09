# Arquitetura e Débitos Técnicos (v2.0)

A versão 1.0 congelou a ISA e estabeleceu estabilidade. Abaixo estão listados os problemas de arquitetura identificados durante a auditoria que não foram corrigidos para manter a compatibilidade com a v1.0.

## 1. O Falso Garbage Collector (Memory Leak)
O `WRITESTR` escreve caracteres na RAM e incrementa a variável global `last_ram_address`. A RAM é estritamente limitada a 1024 palavras.
Entretanto, ao utilizar a syscall `ITOA`, a instrução recebe um registrador de endereço destino. No compilador atual, o registrador associado frequentemente inicializa em `0`.
Como a função Rust `write_ram` sobrescreve `last_ram_address` com o último endereço escrito de forma cega, `ITOA A M N` (onde `M = 0`) faz com que o `last_ram_address` volte para `0` ou `3`.
Isso criou um "GC Acidental", prevenindo que programas com longos loops (como o Gerador de Primos) estourassem a RAM de 1024 palavras. 
**Para a v2.0:** Separar o ponteiro global de alocação de memória das escritas arbitrárias feitas na RAM ou implementar um Heap Manager.

## 2. Linker Silencioso (Silent Failure)
O módulo `linker.py` construído na v1.0 perdoa falhas:
- Importar módulos de bibliotecas nativas do Python (`time`, `math`) não gera erros de compilação, o Linker simplesmente ignora caso não ache o arquivo no diretório atual.
- Expressões e chamadas à funções não registradas na tabela de símbolos (`self.ctx.function_registry`) são ignoradas no visitante AST `visit_Expr`, sem emitir um `NotImplementedError` ou `NameError`.
**Para a v2.0:** Adicionar um modo Strict no compilador que aborte a compilação ao encontrar módulos ou funções não resolvidos.
