section .data
    formato db "MDC calculado: %ld", 10, 0  ; String para o printf

section .text
    global main
    extern printf

main:
    ; Configuração inicial dos valores (Exemplo: A = 15, B = 10)
    mov rax, 15             ; RAX representa a variável "A"
    mov rbx, 10             ; RBX representa a variável "B"

rotulo_teste_igualdade:     ; [ESTADO DE TESTE 1] - Condição de Parada
    cmp rax, rbx            ; Compara A e B
    je rotulo_fim           ; Se A == B (Jump if Equal), achamos o MDC. Vai para o fim.

rotulo_teste_maior:         ; [ESTADO DE TESTE 2] - Bifurcação
    cmp rax, rbx            ; Compara A e B novamente
    jg rotulo_subtrai_a     ; Se A > B (Jump if Greater), desvia para subtrair em A

rotulo_subtrai_b:           ; [ESTADO DE OPERAÇÃO 1] (B = B - A)
    sub rbx, rax            ; Subtrai RAX de RBX
    jmp rotulo_teste_igualdade ; Retorna ao início do laço

rotulo_subtrai_a:           ; [ESTADO DE OPERAÇÃO 2] (A = A - B)
    sub rax, rbx            ; Subtrai RBX de RAX
    jmp rotulo_teste_igualdade ; Retorna ao início do laço

rotulo_fim:                 ; [ESTADO FINAL]
    ; Prepara a chamada para o printf
    mov rdi, formato        ; 1º argumento: a string de formatação
    mov rsi, rax            ; 2º argumento: o resultado (como A == B aqui, tanto faz RAX ou RBX)
    xor rax, rax            ; Limpa RAX para a convenção de chamada C (ABI)
    call printf             ; Chama a função do sistema

    ; Encerra o programa limpo
    mov rax, 60             ; Syscall sys_exit
    xor rdi, rdi            ; Código de retorno 0
    syscall