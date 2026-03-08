mod file_format;
mod read_file;
mod table_byte_to_note;
mod audio;
mod sleep;
mod freq_table;

use std::env;
use std::path::Path;
use std::cmp::min;

use read_file::read;
use table_byte_to_note::get_note_by_byte;
use audio::AudioPlayer;
use sleep::sleep;
use freq_table::FREQ_TABLE;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

fn main() {

    let player = AudioPlayer::new();

    let mut verbose = false;

    let mut vol: f32 = 0.3;
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Use: {} <filePath>", args[0]);
        return;
    }

    let file_path = match args[1].parse::<String>() {
        Ok(path) => path,
        Err(_) => {

            println!("{}[ERROR]{} O argumento tem que ser um arquivo, animal!", RED, RESET);
            
            return;
        },
    };

    if args.contains(&String::from("--verbose")) {
        verbose = true;
        println!("{}[INFO]{} caminho lido: {}", BLUE, RESET, file_path);
    }

    for i in 2 .. args.len() - 1 {
        
        let flag = args[i].as_str();

        match flag {

            "--volume" => {

                let str_volume = &args[i + 1];

                match str_volume.trim().parse::<u8>() {
                    Ok(num) =>  { 
                        
                        vol = min(num, 100) as f32 / 100.0;

                        if verbose {
                            println!("{}[SUCCESS]{} volume setado para {}", GREEN, RESET, (vol * 100.0) as u8);
                        }

                    }
                    Err(e) => println!("Erro na conversão: {}", e),
                }

            },
            _ => {}
        };

    }

    let path = Path::new(&file_path);

    let x360_file = match read(path)  {
        Ok(x) => x,
        Err(e) => {

            eprintln!("{}[ERROR]{} Erro ao processar arquivo: {}", RED, RESET, e);
            
            std::process::exit(1);

        },
    };

    if verbose {

        println!("{}[INFO]{} Versão do arquivo: {}", BLUE, RESET, x360_file.version);

        println!("{}[INFO]{} Nome do arquivo: {}", BLUE, RESET, std::str::from_utf8(&x360_file.name).unwrap_or("?"));

        println!("{}[INFO]{} Volume: {}", BLUE, RESET, vol);

        println!("{}[INFO]{} Notas lidas:", BLUE, RESET);

    }

    for pair in &x360_file.pairs  {

        let string_note: &str;
        
        match get_note_by_byte(pair.note) {
            Some(s) => string_note = s,
            None => string_note = "none",
        };
        
        if verbose {
            println!("{}[MELODY]{} {}: {}ms", YELLOW, RESET, string_note, pair.ms);
        }

    }

    for pair in &x360_file.pairs  {

        if pair.note == 255 {
        
            if verbose {
                println!("{}[SONG]{} 0Hz", GREEN, RESET);
            }

            sleep(pair.ms);

            continue;

        }

        let freq: f32 = FREQ_TABLE[pair.note as usize];

        if verbose {
            println!("{}[SONG]{} {}Hz", GREEN, RESET, freq);
        }

        player.beep(freq, pair.ms, vol);

    }

}