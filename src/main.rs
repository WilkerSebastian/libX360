mod file_format;
mod read_file;
mod table_byte_to_note;
mod beep;
mod sleep;
mod freq_table;

use std::env;
use std::path::Path;

use read_file::read;
use table_byte_to_note::get_note_by_byte;
use beep::beep;
use sleep::sleep;
use freq_table::FREQ_TABLE;

fn main() {

    let vol: f32 = 0.3;
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Use: {} <filePath>", args[0]);
        return;
    }

    let file_path = match args[1].parse::<String>() {
        Ok(path) => path,
        Err(_) => {

            println!("[ERROR] O argumento tem que ser um arquivo, animal!");
            
            return;
        },
    };

    println!("[INFO] caminho lido: {}", file_path);

    let path = Path::new(&file_path);

    let x360_file = match read(path)  {
        Ok(x) => x,
        Err(e) => {

            eprintln!("[ERROR] Erro ao processar arquivo: {}", e);
            
            std::process::exit(1);

        },
    };

    println!("[INFO] Versão do arquivo: {}", x360_file.version);

    println!("[INFO] Nome do arquivo: {}", std::str::from_utf8(&x360_file.name).unwrap_or("?"));

    println!("[INFO] Notas lidas:");

    for pair in &x360_file.pairs  {

        let string_note: &str;
        
        match get_note_by_byte(pair.note) {
            Some(s) => string_note = s,
            None => string_note = "none",
        };
        
        println!("[MELODY] {}: {}ms", string_note, pair.ms);

    }

    for pair in &x360_file.pairs  {

        if pair.note == 255 {
        
            println!("[SONG] 0Hz");

            sleep(pair.ms);

            continue;

        }

        let freq: f32 = FREQ_TABLE[pair.note as usize];

        println!("[SONG] {}Hz", freq);
        
        beep(freq, pair.ms, vol);

    }

}