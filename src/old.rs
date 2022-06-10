const PI: f32 = 3.14;
static VARIAVEL_GLOBAL: u8 = 1;

fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra() {
    let a = 123;
    {
        let b = 456;
        println!("dentro, b = {}", b);
        let a = 777;
        println!("dentro, a = {}", a);
    }
    //println!("fora, b = {}", b);
    println!("a = {}", a);
}

fn escopo() {
    println!("PI = {}", PI);
    unsafe {
        println!("variavel_global = {}", VARIAVEL_GLOBAL);
    }

    let variavel: i32 = 300;
    println!(
        "variavel = {} tamanho = {} bytes",
        variavel,
        std::mem::size_of_val(&variavel)
    );

    let decimal: f32 = 2.5;
    println!(
        "decimal = {} tamanho = {} bytes",
        decimal,
        std::mem::size_of_val(&decimal)
    );

    let mut booleana: bool = false;
    booleana = true;
    println!(
        "booleana = {} tamanho = {} bytes",
        booleana,
        std::mem::size_of_val(&booleana)
    );

    let letra: char = 'C';
    println!(
        "letra = {} tamanho = {} bytes",
        letra,
        std::mem::size_of_val(&letra)
    );
}

fn condicionais() {
    let idade: u8 = 33;

    let condicao = if idade > 17 {
        "Maior de idade"
    } else {
        "Menor de idade"
    };
}

fn repeticao() {
    let mut i = 0;
    while i < 10 {
        println!("i = {}", i);
        i += 1;
    }
    i = 0;
    //do while
    loop {
        println!("i = {}", i);
        i += 1;
        if i == 10 {
            break;
        }
    }

    for i in 0..=10 {
        println!("i = {}", i);
    }
}

fn matchStatement() {
    let linguagem = "PHP";
    let proposito = match linguagem {
        "Rust" => "Programação",
        "Python" => "Programação",
        "Java" => "Programação",
        "C" => "Programação",
        "C++" => "Programação",
        "C#" => "Programação",
        "JavaScript" => "Programação",
        "PHP" => "Programação PHP",
        "Swift" => "Programação",
        "Kotlin" => "Programação",
        "Go" => "Programação",
        _ => "Desconhecido",
    };

    println!("O proposito {} é {} ", linguagem, proposito);
}

fn ownership() {
    let str = String::from("Claudio");
    rouba(&str);
    println!("{}", str);
}
fn rouba(string: &String) {
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!(
            "{}: {}",
            x,
            match x {
                1 => "Pouco",
                2 | 3 => "Um pouquinho",
                4..=10 => "Um bocado",
                _ if x % 2 == 0 => "Uma boa quantidade",
                _ => "Muito",
            }
        )
    }
}

fn erro() {
//  panic!("Erro proposital");
//  let v = vec![1,2,3];
  //v[4];
  
}

fn resultado() -> Result<String, u8>
{
  //Err(String::from("Tudo deu certo"))
  Err(42)
}
fn main() {
  match resultado() {
    Ok(s) => println!("String de sucesso = {}", s),
    Err(numero) => println!("Erro = {}", numero),
  }
}
