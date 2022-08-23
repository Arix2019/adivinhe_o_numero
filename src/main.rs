// código que obtem um palpite do usuario e o imprime
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Por favor, insira seu palpite.");
        
        let mut adivinhe = String::new();

        // lê um valor informado pelo teclado
        io::stdin()
            .read_line(&mut adivinhe)
            .expect("Falha ao ler a linha");

        // convertendo string para numero
        // .trim() eliminara espaço ante e depois do valor informado
        // 'u32' informa o tipo de conversão (numero inteiro)
        let adivinhe: u32 = match adivinhe.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // compara dois numeros
        match adivinhe.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Acertou!!");
                break;
            }
        }
    }

}
