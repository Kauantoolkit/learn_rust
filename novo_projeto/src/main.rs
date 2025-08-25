// #############################################################################
// Guia Inicial para Linguagem de Programação RUST
// Autor: Fabiano Dicheti
// Objetivo: reunir em um arquivo só os exemplos do tutorial com comentários
// linha a linha e anotações “antes/depois” como comentários para orientar
// estudantes no passo-a-passo.
//
// COMO USAR ESTE ARQUIVO
// ----------------------
// 1) Este arquivo está em: src/main.rs
// 2) Por padrão, `main()` chama `demo_hello()`. Para ver outros exemplos,
//    troque a chamada em `main()` conforme indicado (basta comentar/alternar).
// 3) Cada bloco está HYPER-COMENTADO para servir de guia de estudo.
// 4) Alguns trechos (como a demo de `rand`) estão comentados propositalmente.
//    Siga as instruções para ativar.
//
// SUMÁRIO (neste arquivo)
// -----------------------
// [Pré-requisitos e Instalação]         => comentários abaixo
// [Verificação]                         => comentários abaixo
// [Comandos Cargo e Estrutura]          => comentários abaixo
// [DEMO 1: Hello World]                 => fn demo_hello()
// [DEMO 2: Fundamentos + testes]        => fn demo_basics(), módulo #[cfg(test)]
// [DEMO 3: Ownership & Borrowing]       => fn demo_ownership()
// [DEMO 4: Usando uma dependência]      => BLOCO COMENTADO: demo_rand
// [VS Code + Produtividade]             => comentários
// [Comandos úteis / Erros comuns]       => comentários
// [Próximos passos]                     => comentários
//
// #############################################################################
// PRÉ-REQUISITOS (LEITURA)
// ------------------------
// - macOS ou Linux (ou WSL no Windows), não recomendo o uso no Windows.
// - (Opcional) VS Code com extensões: rust-analyzer, CodeLLDB, Even Better TOML.
//
// INSTALAÇÃO (LEITURA)
// --------------------
// macOS/Linux/WSL:
//   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
//   # Feche e reabra o terminal (ou `source $HOME/.cargo/env`).
//
// VERIFICAÇÃO (LEITURA)
// ---------------------
//   $ rustc --version
//   $ cargo --version
//   $ rustup update
//
// CRIAR PROJETO (LEITURA)
// -----------------------
//   $ cargo new ola_rust
//   $ cd ola_rust
//   $ cargo run
//
// ESTRUTURA (LEITURA)
// -------------------
//   ola_rust/
//   ├─ Cargo.toml
//   └─ src/
//      └─ main.rs   <- ESTE ARQUIVO
//
// #############################################################################
// DICAS GERAIS (LEITURA)
// ----------------------
// - `cargo check` confere erros rapidamente sem gerar binário.
// - `cargo fmt` formata o código (precisa do componente rustfmt).
// - `cargo clippy` sugere melhorias (precisa do componente clippy).
// - `rustup doc --book` abre o The Rust Book offline.
// #############################################################################

// #############################################################################
// PONTO DE ENTRADA
// #############################################################################
fn main() {
    // >>> Para alternar a demo ativa, troque a chamada aqui:
    demo_hello();        // 1) Hello World (padrão: simples e imediato)

    // demo_basics();    // 2) Fundamentos (variáveis, funções, if/for/match) + testes
    // demo_ownership(); // 3) Ownership & Borrowing (conceito central)

    // DICA: A "DEMO 4 (rand)" está mais abaixo como BLOCO COMENTADO para evitar
    // erro de compilação caso a dependência não esteja adicionada. Veja instruções
    // no próprio bloco e ative manualmente quando quiser.
}

// #############################################################################
// DEMO 1 — HELLO WORLD
// Objetivo: executar algo de ponta a ponta com o menor atrito possível.
// #############################################################################
fn demo_hello() {
    // `println!` é uma macro (note o "!") que imprime texto com quebra de linha.
    // Aqui usamos Unicode (emoji) sem problemas; Rust lida bem com UTF-8 por padrão.
    println!("Olá, Rust!");

    // Interpolação de variáveis por nome (Rust >= 1.58):
    // Basta ter variáveis no escopo com os mesmos nomes dos placeholders.
    let nome = "Estudante";
    let versao_exemplo = 1;
    println!("Bem-vindo(a), {nome}! Versão do exemplo = {versao_exemplo}");

    // DICA (depois): volte a `cargo run` sempre que alterar o código.
    // DICA (VS Code): instale "rust-analyzer" para autocompletar e tooltips.
}

// #############################################################################
// DEMO 2 — FUNDAMENTOS
// - Imutabilidade por padrão (`let`), mutabilidade com `mut`.
// - Tipagem estática e inferência de tipos.
// - Funções, `if` expressivo, laço `for` com ranges e `match` para pattern matching.
// - Inclui um teste unitário simples em #[cfg(test)] no final do arquivo.
// #############################################################################

// Função simples fora do `demo_basics` para ser testável em #[cfg(test)].
// Linha a linha:
// - `fn` define uma função.
// - `soma` é o nome.
// - `(a: i32, b: i32)` são os parâmetros do tipo `i32` (inteiros 32 bits).
// - `-> i32` indica o tipo de retorno.
// - Corpo retorna `a + b`. Sem ponto e vírgula => retorna o valor da expressão.
fn soma(a: i32, b: i32) -> i32 { a + b }

fn demo_basics() {
    // Declaração imutável (padrão): o compilador infere `i32` para `x`.
    let x = 10;
    // Declaração mutável: `mut` permite reatribuir/alterar o valor.
    let mut y = 10;
    y += 1; // Agora `y` é 11.

    // Anotações explícitas de tipo (opcional quando o compilador pode inferir).
    let pi: f64 = 3.1415; // ponto flutuante 64 bits.
    let nome: &str = "aqui coloca uma string, no tutorial estava meu nome"; // &str: fatia de string estática, embutida no binário.

    // Interpolação por nome (Rust >= 1.58): usa variáveis do escopo diretamente.
    println!("x={x}, y={y}, pi={pi}, nome={nome}");

    // Chamando a função `soma` definida acima; imprime com formatação posicional `{}`.
    println!("2 + 3 = {}", soma(2, 3));

    // `if` como expressão: retorna um valor, que guardamos em `msg`.
    let n = 7;
    let msg = if n % 2 == 0 { "par" } else { "ímpar" };
    println!("{n} é {msg}");

    // Laço `for` com intervalo INCLUSIVO: `1..=3` gera 1, 2, 3.
    for i in 1..=3 {
        println!("i = {i}");
    }

    // `match` faz pattern matching poderoso; casos cobrem padrões e intervalos.
    match n {
        0 => println!("zero"),
        1..=5 => println!("entre 1 e 5"),
        _ => println!("maior que 5"),
    }

    // DICA (antes): observe warnings do compilador; eles ajudam no estilo idiomático.
    // DICA (depois): rode `cargo fmt` (formatar) e `cargo clippy` (sugestões).
}

// #############################################################################
// DEMO 3 — OWNERSHIP & BORROWING
// - Cada valor possui um dono (owner).
// - "Mover" transfere a posse (o antigo dono não pode usar depois do move).
// - "Emprestar" (&T ou &mut T) permite acesso sem mover a posse, respeitando regras.
// #############################################################################

// Função que recebe um EMPRÉSTIMO imutável de String (&String) e retorna seu tamanho.
// Com &String, não movemos a posse; quem chamou continua podendo usar o valor original.
fn tamanho(s: &String) -> usize { s.len() }

fn demo_ownership() {
    // Cria uma String alocada no heap (diferente de &str). Dono: `s1`.
    let s1 = String::from("Hello");

    // MOVE: transfere a posse de `s1` para `s2`. Após esta linha, `s1` fica inválida.
    let s2 = s1;

    // Esta linha causaria erro de compilação porque `s1` foi movida:
    // println!("{}", s1);

    // Empréstimo (borrowing) imutável: passamos &s3 para uma função que não toma posse.
    let s3 = String::from("Olá");
    println!("tamanho: {}", tamanho(&s3)); // &s3 => `s3` continua válido aqui.

    // DICA (depois): experimente criar um empréstimo mutável (&mut String) e editar o texto.
    // Regra importante: no mesmo escopo, só pode haver UM empréstimo mutável ativo
    // ou vários empréstimos imutáveis (não ambos ao mesmo tempo).
}

// #############################################################################
// DEMO 4 — USANDO UMA DEPENDÊNCIA (rand)
// -----------------------------------------------------------------------------
// STATUS: BLOCO COMENTADO para evitar erro de compilação caso você ainda não
//         tenha adicionado a dependência.
// COMO ATIVAR:
//   1) Rode: cargo add rand
//   2) Descomente o bloco abaixo (remova os `//`)
//   3) Chame `demo_rand()` no `main()`
// -----------------------------------------------------------------------------

// use rand::Rng; // <- DESCOMENTE após `cargo add rand`
//
// fn demo_rand() {
//     // `thread_rng()` cria um gerador de números aleatórios por thread.
//     // `gen_range(1..=10)` gera um `u8` no intervalo inclusivo 1..=10.
//     let n: u8 = rand::thread_rng().gen_range(1..=10);
//     println!("Seu número aleatório: {n}");
// }
//
// DICA (depois): explore a documentação da crate `rand` para ver distribuição,
//               semente (seed), etc.

// #############################################################################
// VS CODE + PRODUTIVIDADE (LEITURA)
// ---------------------------------
// - Extensões recomendadas: rust-analyzer, CodeLLDB, Even Better TOML.
// - Ative "Format on Save" para usar `rustfmt` automaticamente.
// - Comandos úteis:
//     rustup component add rustfmt clippy
//     cargo fmt
//     cargo clippy
// - Dica: `cargo check` é mais rápido que `cargo build` para detectar erros cedo.
// #############################################################################

// #############################################################################
// COMANDOS DO DIA A DIA (LEITURA)
// -------------------------------
// cargo new meu_app        # cria novo projeto binário
// cargo run                # compila e executa
// cargo check              # checa erros rapidamente (sem gerar binário)
// cargo test               # roda testes
// cargo fmt                # formata o código (rustfmt)
// cargo clippy             # lints (sugestões/avisos)
// rustup update            # atualiza toolchain
// rustup doc --book        # abre "The Rust Book" offline
// #############################################################################

// #############################################################################
// ERROS COMUNS (LEITURA)
// ----------------------
// - `cargo`/`rustc` não encontrados:
//     * Reabra o terminal; confira PATH: Windows (%USERPROFILE%\.cargo\bin)
//       e Linux/macOS ($HOME/.cargo/bin).
// - Windows pedindo MSVC/Build Tools:
//     * Instale Microsoft C++ Build Tools (Desktop development with C++).
// - Ao usar `rand`, erros de dependência:
//     * Rode `cargo add rand` antes de descomentar a demo.
// #############################################################################

// #############################################################################
// PRÓXIMOS PASSOS (LEITURA)
// ------------------------
// - The Rust Book (capítulos 3–6): tipos, ownership, referências, struct/enum.
// - Rustlings: exercícios práticos no terminal.
// - Aprofunde em: Result/Option, erros, pattern matching avançado, módulos,
//   traits, lifetimes, coleções (Vec, HashMap) e criação de CLIs.
// #############################################################################

// #############################################################################
// TESTES — ficam sempre no final por convenção.
// - `#[cfg(test)]` garante que este módulo só é compilado quando rodamos `cargo test`.
// - `use super::*;` importa itens do módulo pai para usar aqui (`soma`).
// #############################################################################
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soma_deve_funcionar() {
        // `assert_eq!` verifica se os valores são iguais.
        // Se falhar, o teste falha e o runner de testes informa o motivo.
        assert_eq!(soma(2, 3), 5);
    }
}
