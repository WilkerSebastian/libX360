use std::env;
use std::fs::File;
use std::io::{Read};
use std::path::Path;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Use: {} <filePath>", args[0]);
        return;
    }

    let file_path = match args[1].parse::<String>() {
        Ok(path) => path,
        Err(_) => {

            println!("[ERROR]: O argumento tem que ser um arquivo, animal!");
            
            return;
        },
    };

    let path = Path::new(&file_path);

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {

            println!("[ERROR]: Arquivo não econtrado!");
            
            return;
        }
    };


    let mut buffer = Vec::new();

    let _ = file.read_to_end(&mut buffer);

    println!("Tamanho lido: {} bytes", buffer.len());
    println!("Primeiros 5 bytes: {:?}", &buffer[0..5]);    

}