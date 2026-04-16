# Trabalho AV1 — Programas, Máquinas e Equivalência
**Disciplina:** Teoria da Computabilidade  
**Professor:** Daniel Leal Souza  
**Semestre:** 01/2026 — CESUPA  

---

## 👥 Equipe

| Nome |
|------|
| Benjamin Yuji Suzuki |
| Felipe de Freitas |
| Ryan Iketani |

---

## 📌 Função Computada: MDC (Máximo Divisor Comum)

A função escolhida é o **Máximo Divisor Comum (MDC)** de dois números naturais positivos, calculada pelo **Algoritmo de Euclides por Subtração**.

### Definição formal

```
mdc : ℕ⁺ × ℕ⁺ → ℕ⁺

mdc(a, b) = a,               se a = b
mdc(a, b) = mdc(a − b, b),  se a > b
mdc(a, b) = mdc(a, b − a),  se b > a
```

### Entradas e Saídas

| Parâmetro | Tipo | Restrição |
|-----------|------|-----------|
| `a` | Inteiro sem sinal de 64 bits | `a > 0` |
| `b` | Inteiro sem sinal de 64 bits | `b > 0` |
| **saída** | Inteiro sem sinal de 64 bits | MDC(a, b) |

### Exemplos

| a | b | MDC(a, b) |
|---|---|-----------|
| 15 | 10 | **5** |
| 12 | 8 | **4** |
| 7 | 13 | **1** |
| 6 | 6 | **6** |

---

## 🗂️ Organização dos Arquivos

```
.
├── README.md                  ← Este arquivo
├── mdc_monolitico.asm         ← Programa monolítico (Assembly x86-64, Linux)
└── main.rs                    ← Programas iterativo e recursivo (Rust)
```

---

## 💻 Programas Implementados

### 1. Programa Monolítico — Assembly x86-64

**Arquivo:** `mdc_monolitico.asm`

O programa monolítico é caracterizado por **desvios explícitos** (jumps/gotos) entre rótulos, sem abstrações de estruturas de controle. O fluxo é controlado exclusivamente por saltos condicionais e incondicionais — equivalente direto à lógica de um programa com `goto`.

```asm
section .data
    formato db "MDC calculado: %ld", 10, 0

section .text
    global main
    extern printf

main:
    mov rax, 15                      ; A = 15
    mov rbx, 10                      ; B = 10

rotulo_teste_igualdade:              ; [TESTE 1] Condição de parada
    cmp rax, rbx
    je rotulo_fim                    ; Se A == B → fim

rotulo_teste_maior:                  ; [TESTE 2] Bifurcação
    cmp rax, rbx
    jg rotulo_subtrai_a              ; Se A > B → subtrai em A

rotulo_subtrai_b:                    ; [OPERAÇÃO 1] B = B - A
    sub rbx, rax
    jmp rotulo_teste_igualdade

rotulo_subtrai_a:                    ; [OPERAÇÃO 2] A = A - B
    sub rax, rbx
    jmp rotulo_teste_igualdade

rotulo_fim:                          ; [ESTADO FINAL]
    mov rdi, formato
    mov rsi, rax
    xor rax, rax
    call printf
    mov rax, 60
    xor rdi, rdi
    syscall
```

**Rótulos e sua função:**

| Rótulo | Tipo | Descrição |
|--------|------|-----------|
| `rotulo_teste_igualdade` | Teste | Verifica se A = B (condição de parada) |
| `rotulo_teste_maior` | Teste | Verifica se A > B (bifurcação) |
| `rotulo_subtrai_b` | Operação | Executa B ← B − A |
| `rotulo_subtrai_a` | Operação | Executa A ← A − B |
| `rotulo_fim` | Final | Exibe o resultado e encerra |

---

### 2. Programa Iterativo — Rust

**Arquivo:** `main.rs` — função `mdc_iterativo`

O programa iterativo usa um **laço `while`** explícito que repete enquanto a condição de parada não é satisfeita.

```rust
fn mdc_iterativo(mut a: u64, mut b: u64) -> u64 {
    while a != b {          // Equivalente a rotulo_teste_igualdade
        if a > b {          // Equivalente a rotulo_teste_maior
            a = a - b;      // Equivalente a rotulo_subtrai_a
        } else {
            b = b - a;      // Equivalente a rotulo_subtrai_b
        }
    }
    a                       // Equivalente a rotulo_fim
}
```

---

### 3. Programa Recursivo — Rust

**Arquivo:** `main.rs` — função `mdc_recursivo`

O programa recursivo usa **chamadas recursivas** para cada caso e possui **condição-base** de parada explícita.

```rust
fn mdc_recursivo(a: u64, b: u64) -> u64 {
    if a == b {                              // Condição-base (parada)
        return a;
    }
    if a > b {
        return mdc_recursivo(a - b, b);     // Chamada recursiva
    } else {
        return mdc_recursivo(a, b - a);     // Chamada recursiva
    }
}
```

---

## ⚙️ Instruções de Compilação e Execução

### Programa Monolítico (Assembly)

> **Requisito:** Linux x86-64, NASM, GCC

```bash
# Montar o objeto
nasm -f elf64 mdc_monolitico.asm -o mdc_monolitico.o

# Linkar com libc (para usar printf)
gcc mdc_monolitico.o -o mdc_monolitico -no-pie

# Executar
./mdc_monolitico
```

**Saída esperada:**
```
MDC calculado: 5
```

---

### Programas Iterativo e Recursivo (Rust)

> **Requisito:** [Rust](https://rustup.rs/) instalado (`rustc` ou `cargo`)

```bash
# Compilar diretamente com rustc
rustc main.rs -o mdc_rust

# Executar
./mdc_rust
```

**Ou com Cargo** (se estiver em um projeto):

```bash
cargo run
```

**Saída esperada:**
```
--- Teste de Equivalência MDC ---
Valores de entrada: A = 15, B = 10
Resultado Iterativo: 5
Resultado Recursivo: 5
```

---

## 🔬 Máquina de Traços (MT)

A Máquina de Traços é o formalismo utilizado para analisar a **equivalência forte** entre programas. Ela registra, passo a passo, cada estado do programa (testes realizados, operações executadas, valores das variáveis) formando uma **cadeia de execução (traço)**.

### Notação utilizada

Um estado é representado como:

```
⟨rótulo, (valor_A, valor_B)⟩
```

### Traço do Programa Monolítico — Entrada (15, 10)

```
⟨rotulo_teste_igualdade, (15, 10)⟩
→ 15 ≠ 10 → segue

⟨rotulo_teste_maior, (15, 10)⟩
→ 15 > 10 → desvia para rotulo_subtrai_a

⟨rotulo_subtrai_a, (15, 10)⟩
→ A ← 15 − 10 = 5

⟨rotulo_teste_igualdade, (5, 10)⟩
→ 5 ≠ 10 → segue

⟨rotulo_teste_maior, (5, 10)⟩
→ 5 < 10 → cai para rotulo_subtrai_b

⟨rotulo_subtrai_b, (5, 10)⟩
→ B ← 10 − 5 = 5

⟨rotulo_teste_igualdade, (5, 5)⟩
→ 5 = 5 → desvia para rotulo_fim

⟨rotulo_fim, (5, 5)⟩
→ resultado = 5 ✓
```

### Traço do Programa Iterativo — Entrada (15, 10)

```
⟨while (a≠b), (15, 10)⟩  → verdadeiro
⟨if (a>b), (15, 10)⟩      → verdadeiro → a ← 5

⟨while (a≠b), (5, 10)⟩   → verdadeiro
⟨if (a>b), (5, 10)⟩       → falso → b ← 5

⟨while (a≠b), (5, 5)⟩    → falso → encerra

resultado = 5 ✓
```

### Traço do Programa Recursivo — Entrada (15, 10)

```
mdc(15, 10)
→ 15 ≠ 10, 15 > 10 → mdc(5, 10)
  → 5 ≠ 10, 5 < 10 → mdc(5, 5)
    → 5 = 5 → retorna 5
  ← 5
← 5

resultado = 5 ✓
```

---

## ✅ Exemplo de Equivalência Forte

**Entrada:** (15, 10)

Os três programas produzem a **mesma sequência de decisões e resultados intermediários**:

| Passo | Monolítico | Iterativo | Recursivo |
|-------|-----------|-----------|-----------|
| 1 | `(15,10)` → A≠B | `while` A≠B | `mdc(15,10)` → A≠B |
| 2 | A>B → A←5 | A>B → a←5 | A>B → `mdc(5,10)` |
| 3 | `(5,10)` → A≠B | `while` A≠B | A<B → `mdc(5,5)` |
| 4 | A<B → B←5 | A<B → b←5 | A=B → **retorna 5** |
| 5 | `(5,5)` → A=B | `while` falso | — |
| 6 | **resultado: 5** | **resultado: 5** | **resultado: 5** |

Os traços coincidem em **cada teste e operação**, na mesma ordem. Os três programas são **fortemente equivalentes** para esta entrada.

---

## ❌ Exemplo de Não-Equivalência

Para demonstrar não-equivalência, comparamos o **programa monolítico original** com uma **versão modificada** onde a ordem dos testes é alterada: o teste `rotulo_teste_maior` é substituído por um teste **invertido** (`rotulo_teste_menor`), que verifica `A < B` primeiro.

**Versão modificada (hipotética):**

```
rotulo_teste_igualdade → rotulo_teste_MENOR (verifica A < B primeiro)
  se A < B → rotulo_subtrai_b
  senão    → rotulo_subtrai_a  (trata A > B e A = B da mesma forma)
```

**Traço da versão modificada — Entrada (15, 10):**

```
⟨rotulo_teste_igualdade, (15, 10)⟩ → 15 ≠ 10

⟨rotulo_teste_MENOR, (15, 10)⟩
→ 15 < 10? NÃO → cai para rotulo_subtrai_a → A ← 5

⟨rotulo_teste_igualdade, (5, 10)⟩ → 5 ≠ 10

⟨rotulo_teste_MENOR, (5, 10)⟩
→ 5 < 10? SIM → rotulo_subtrai_b → B ← 5

⟨rotulo_teste_igualdade, (5, 5)⟩ → resultado = 5
```

Neste caso específico o resultado final coincide, mas o **teste realizado no passo 2 é diferente**: o programa original avalia `A > B`, enquanto a versão modificada avalia `A < B`. Os traços **divergem na natureza do teste executado**, caracterizando **não-equivalência forte** — mesmo que a saída final seja igual para esta entrada, a cadeia de estados não é idêntica.

> **Ponto de divergência:** `rotulo_teste_maior` (testa `A > B`) ≠ `rotulo_teste_MENOR` (testa `A < B`). Programas que divergem nos testes intermediários **não são fortemente equivalentes**, mesmo com saída idêntica.

---

## 🤖 Uso de Inteligência Artificial

| Ferramenta | Finalidade | O que foi aproveitado | Revisão da equipe |
|------------|------------|----------------------|-------------------|
| Google Gemini | Auxílio no desenvolvimento dos códigos-fonte | Geração inicial dos programas monolítico (Assembly x86-64), iterativo e recursivo (Rust) | Os códigos foram revisados, testados e validados pela equipe, que verificou a correção da lógica e a aderência aos conceitos da disciplina. |
| Claude (Anthropic) | Auxílio na estruturação do README, formatação da Máquina de Traços e revisão da notação formal | Estrutura do documento, tabelas de traços, notação dos estados | Toda a análise formal foi verificada e validada pela equipe com base no conteúdo da disciplina. |

---

## 📚 Referências

- DIVERIO, Tiaraju Asmuz; MENEZES, Paulo Fernando Blauth. **Teoria da Computação**. Sagra Luzzatto.
- Material de aula — Prof. Daniel Leal Souza (slides sobre programas monolíticos, iterativos, recursivos e Máquina de Traços).
- Documentação oficial Rust: https://www.rust-lang.org
- Documentação NASM: https://www.nasm.us
