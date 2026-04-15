# Equivalência de Programas - Teoria da Computação (TC)

Este repositório contém um projeto desenvolvido para a disciplina de **Teoria da Computação**. O objetivo principal é demonstrar de forma prática o conceito de **Equivalência de Programas**, comparando duas implementações de um mesmo algoritmo (o cálculo do Máximo Divisor Comum - MDC) em diferentes níveis de abstração e paradigmas.

## 🧠 Sobre o Projeto

Na teoria da computação, dois programas são considerados equivalentes se, para a mesma entrada, eles produzem a mesma saída (equivalência comportamental/funcional). 

Para ilustrar isso, este repositório traz:
1. Uma implementação de **alto nível** e estruturada utilizando **Rust**.
2. Uma implementação de **baixo nível**, construída de forma **monolítica** (sem uso de sub-rotinas/funções estruturadas, dependendo apenas de saltos/jumps), utilizando **Assembly**.

Ambos os programas recebem dois números inteiros e calculam o MDC entre eles, provando que é possível atingir o mesmo resultado computacional independentemente da linguagem ou da estrutura de controle de fluxo utilizada.

## 📂 Estrutura do Repositório

* `main.rs`: Contém o código-fonte em Rust. Representa a solução em linguagem de alto nível.
* `mdc_monolitico.asm`: Contém o código-fonte em linguagem Assembly. Representa a solução monolítica e de baixo nível computacional.

## 🛠️ Tecnologias Utilizadas

* **[Rust](https://www.rust-lang.org/)**: Linguagem de programação focada em segurança e performance.
* **Assembly**: Linguagem de montagem (baixo nível).

