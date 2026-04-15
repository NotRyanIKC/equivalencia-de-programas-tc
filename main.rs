

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

fn main() {

}

