use rand::prelude::*;
use std::io;

// Gera a array da roleta aleatoriamente

fn generate_roulete() -> [u32; 9] {
    let mut rng = rand::thread_rng();
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for index in 0..9 {
        let numero_aleatorio = rng.gen_range(0..3);
        arr[index] = numero_aleatorio;
    }

    arr
}

// Retorna uma string lida pela array dada como argumento

fn print_roulete(roulete: [u32; 9]) -> String {
    let mut counter = 0;
    let mut printed: String = "".to_string();

    for numero in roulete.iter() {
        if counter == 3 {
            counter = 0;
            printed += "|\n";
        }

        printed += "|";
        printed += &numero.to_string();

        counter += 1;
    }

    printed += "|";

    return printed;
}

// Le possiveis resultados

fn read_roulete(roulete: [u32; 9], aposta: u32) -> u32 {
    let mut results: u32 = 0;

    if roulete[0] == roulete[1] && roulete[0] == roulete[2] {
        results += aposta * roulete[0];
    }

    if roulete[3] == roulete[4] && roulete[3] == roulete[5] {
        results += aposta * roulete[3];
    }

    if roulete[6] == roulete[7] && roulete[6] == roulete[8] {
        results += aposta * roulete[6];
    }

    if roulete[0] == roulete[4] && roulete[0] == roulete[8] {
        results += aposta * roulete[0];
    }

    if roulete[2] == roulete[4] && roulete[2] == roulete[6] {
        results += aposta * roulete[2];
    }

    return results;
}

fn main() {
    println!("Bem vindo ao jogo SLOT!");
    println!("Coloque sua aposta: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha!");

    let aposta: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Insira um valor válido!");
            return;
        }
    };

    let roulete = generate_roulete();
    let printed_roulete = print_roulete(roulete);

    println!("\n{}", printed_roulete);

    let results = read_roulete(roulete, aposta);
    println!("\nSeus resultados: {}", results);
}
