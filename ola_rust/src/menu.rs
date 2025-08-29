use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== Menu Ultra Completo Rust ===");
        println!("1 - Funções");
        println!("2 - If como expressão / If tradicional");
        println!("3 - Loops (for, while, loop, array)");
        println!("4 - Match");
        println!("5 - Variáveis, mutabilidade e shadowing");
        println!("6 - Ownership");
        println!("7 - Borrowing");
        println!("8 - Lifetimes com strings");
        println!("9 - Struct Usuario e métodos");
        println!("0 - Sair");

        print!("Escolha uma opção: ");
       // io::stdout().flush().unwrap();//tratamento para aparecer em alguns terminais gptzada, n me foi necessário ent removi




        //isso é interessante, esse string tem: um ponteiro pro buffer em heap, o espaço alocado em buffer e quantos bytes são usados,
        //portanto, opcao é dono dessa string, eu poderia passar essa posse para outra variavel, mas então opcao perderia
        //o valor pois no caso de string só pode existir um dono por vez, seria o "move", mover a posse
        //assim que a variavel sair do escopo a memória dela é liberada, 
        let mut opcao = String::new();  

        io::stdin().read_line(&mut opcao).unwrap();
        let opcao = opcao.trim();

        match opcao {
            "1" => ownership_exemplo(),
            "2" => borrowing_exemplo(), //valores da stack são copiados, da heap são movidos
            "3" => loops_exemplo(), 
            "4" => match_exemplo(),
            "5" => variaveis_exemplo(),   //mutabilidade e shadowing
            "6" => funcoes_exemplo(),
            "7" => if_exemplo(),
            "8" => lifetimes_exemplo(),
            "9" => struct_usuario_exemplo(),
            "0" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }
}

// === 1 Funções ===
fn funcoes_exemplo() {
    println!("\n--- Funções ---");

    fn saudar() { println!("Olá, mundo!"); }
    fn saudar_pessoa(nome: &str) { println!("Olá, {}!", nome); }
    fn soma(a: i32, b: i32) -> i32 { a + b }
    fn multiplicacao(a: i32, b: i32) -> i32 { return a * b; }

    saudar();
    saudar_pessoa("Ferris");

    let resultado = soma(10, 5);
    println!("10 + 5 = {}", resultado);

    let produto = multiplicacao(4, 7);
    println!("4 × 7 = {}", produto);
}

// === 2 If ===
fn if_exemplo() {
    println!("\n--- If como expressão / If tradicional ---");
    let numero = 7;

    let tipo = if numero % 2 == 0 { "par" } else { "ímpar" };
    println!("{} é {}", numero, tipo);

    if numero > 10 {
        println!("Número grande!");
    } else if numero > 5 {
        println!("Número médio!");
    } else {
        println!("Número pequeno!");
    }
}

// === 3 Loops ===
fn loops_exemplo() {
    println!("\n--- Loops ---");

    println!("--- For de 1 a 5 (inclusivo) ---");
    for i in 1..=5 { println!("i = {}", i); }

    println!("--- For de 0 a 4 (exclusivo) ---");
    for i in 0..5 { println!("i = {}", i); }

    println!("--- Iterando sobre array ---");
    let numeros = [1, 2, 3, 4, 5];
    for n in numeros { println!("número: {}", n); }

    println!("--- While ---");
    let mut contador = 0;
    while contador < 3 {
        println!("contador = {}", contador);
        contador += 1;
    }

    println!("--- Loop infinito com break e continue ---");
    let mut x = 0;
    loop {
        if x == 5 { break; }
        if x % 2 == 0 {
            x += 1;
            println!("x = {} (par, continue)", x);
            continue;
        }
        x += 1;
        println!("x = {}", x);
    }
}

// === 4 Match ===
fn match_exemplo() {
    println!("\n--- Match ---");

    let numero = 3;

    match numero {
        0 => println!("Zero!"),
        1 => println!("Um!"),
        2 | 3 => println!("Dois ou três!"),
        4..=9 => println!("Entre 4 e 9!"),
        _ => println!("Outro número!"),
    }

    let descricao = match numero {
        0 => "zero",
        1..=5 => "pequeno",
        6..=10 => "médio",
        _ => "grande",
    };
    println!("O número {} é {}", numero, descricao);
}

// === 5 Variáveis, mutabilidade e shadowing ===
fn variaveis_exemplo() {
    println!("\n--- Variáveis, Mutabilidade e Shadowing ---");

    let x = 10;
    println!("x = {}", x);
    let x = 20; // shadowing
    println!("x = {}", x);

    let mut y = 10;
    println!("y = {}", y);
    y = 20; // mutável
    println!("y = {}", y);
}

// === 6 Ownership ===
fn ownership_exemplo() {
    println!("\n--- Ownership ---");

    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}", s1); // ERRO
    println!("s2 = {}", s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

// === 7 Borrowing ===
fn borrowing_exemplo() {
    println!("\n--- Borrowing ---");

    fn tamanho(s: &String) -> usize { s.len() }

    let texto = String::from("Rust é incrível!");
    let tam = tamanho(&texto);
    println!("'{}' tem {} caracteres", texto, tam);

    let mut s = String::from("hello");
    let r = &mut s;
    r.push_str(", world!");
    println!("s depois do borrow mutável: {}", r);
}

// === 8 Lifetimes ===
fn lifetimes_exemplo() {
    println!("\n--- Lifetimes ---");

    fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let string1 = String::from("longa string");
    let string2 = "curta";
    let res = maior(&string1, string2);
    println!("A maior string é: {}", res);
}

// === 9 Struct Usuario e métodos ===
fn struct_usuario_exemplo() {
    println!("\n--- Struct Usuario ---");

    struct Usuario { nome: String, email: String, ativo: bool }

    impl Usuario {
        fn exibir_info(&self) {
            println!("Usuário: {} ({})", self.nome, self.email);
            println!("Status: {}", if self.ativo { "Ativo" } else { "Inativo" });
        }
        fn ativar(&mut self) { self.ativo = true; println!("Usuário {} foi ativado!", self.nome); }
        fn deletar(self) -> String { println!("Deletando usuário {}...", self.nome); self.nome }
    }

    fn processar_usuario(usuario: &Usuario) {
        println!("Processando usuário: {}", usuario.nome);
    }

    let mut usuario = Usuario {
        nome: String::from("Ana Silva"),
        email: String::from("ana@email.com"),
        ativo: false,
    };

    usuario.exibir_info();
    processar_usuario(&usuario);
    usuario.ativar();
    let nome_deletado = usuario.deletar();
    println!("Nome do usuário deletado: {}", nome_deletado);
}
