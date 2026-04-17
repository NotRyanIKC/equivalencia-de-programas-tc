fn mdc_iterativo(mut a: u64, mut b: u64) -> u64 {
    while a != b { // Equivalente ao rotulo_teste_igualdade
        if a > b { // Equivalente ao rotulo_teste_maior
            a = a - b; // Equivalente ao rotulo_subtrai_a
        } else {
            b = b - a; // Equivalente ao rotulo_subtrai_b
        }
    }
    a // Equivalente ao rotulo_fim
}

fn mdc_recursivo(a: u64, b: u64) -> u64 {
    if a == b {
        return a; // Condição-base (rotulo_teste_igualdade)
    }
    if a > b {
        return mdc_recursivo(a - b, b); // Chamada recursiva (rotulo_subtrai_a)
    } else {
        return mdc_recursivo(a, b - a); // Chamada recursiva (rotulo_subtrai_b)
    }
}

// NOVA FUNÇÃO: A Máquina de Traços (que imprime a fita pulando linhas)
fn maquina_tracos_monolitico(mut a: u64, mut b: u64) {
    println!("--- Fita da Máquina de Traços (Entrada: {}, {}) ---", a, b);
    
    // Estado inicial
    println!("| ⟨rotulo_teste_igualdade, ({}, {})⟩ |", a, b);
    
    loop {
        if a == b {
            // Se A == B, vai para o fim e encerra
            println!(" ⊢ | ⟨rotulo_fim, ({}, {})⟩ |", a, b);
            break;
        }
        
        // Se não for igual, ele testa quem é maior
        println!(" ⊢ | ⟨rotulo_teste_maior, ({}, {})⟩ |", a, b);
        
        if a > b {
            a = a - b; // Subtrai A
            println!(" ⊢ | ⟨rotulo_subtrai_a, ({}, {})⟩ |", a, b);
        } else {
            b = b - a; // Subtrai B
            println!(" ⊢ | ⟨rotulo_subtrai_b, ({}, {})⟩ |", a, b);
        }
        
        // Retorna ao teste de igualdade
        println!(" ⊢ | ⟨rotulo_teste_igualdade, ({}, {})⟩ |", a, b);
    }
    println!();
}

fn main() {
    let a = 15;
    let b = 10;
    
    println!("--- Teste de Equivalência MDC ---");
    println!("Valores de entrada: A = {}, B = {}", a, b);
    println!("Resultado Iterativo: {}", mdc_iterativo(a, b));
    println!("Resultado Recursivo: {}", mdc_recursivo(a, b));
    
    // Chama a simulação para exibir a fita no terminal
    println!("\n--- Gerando Fita da Máquina de Traços ---");
    maquina_tracos_monolitico(a, b); 
}