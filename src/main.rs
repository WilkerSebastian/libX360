mod file_format;
mod table_byte_to_note;
mod audio;
mod freq_table;
mod read_csv;

use std::env;
use std::path::Path;
use std::cmp::min;
use std::thread::sleep;
use std::time::Duration;

use file_format::read;
use file_format::write;
use table_byte_to_note::get_note_by_byte;
use audio::AudioPlayer;
use freq_table::FREQ_TABLE;
use read_csv::read_csv;

use crate::file_format::{AudioBlock, X360File};


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

    let path = Path::new(&args[1]);

    if args.contains(&String::from("--verbose")) {
        verbose = true;
        println!("{}[INFO]{} caminho lido: {}", BLUE, RESET, args[1]);
    }

    if args[2] == "-o" {

        let data_csv = match read_csv(path) {
            Ok(data) => data,
            Err(e) => {

                println!("{}[ERROR]{} {}", RED, RESET, e);
                
                std::process::exit(1);

            },
        };

        let mut pairs: Vec<AudioBlock> = [].to_vec();

        for c in data_csv  {
            pairs.push(AudioBlock {
                note: c.note,
                ms: c.time
            });
        }

        let data = X360File {
            version: 1,
            name: [ 0b01011000, 0b00110011, 0b00110110, 0b00110000 ],
            pairs: pairs
        };

        match write(Path::new(&args[3]), data) {
            Ok(_) => {
                if verbose {
                    println!("{}[SUCCESS]{} escrita do {} feito com sucesso!", GREEN, RESET, args[3]);
                }

                std::process::exit(0);
            }
            Err(e) => { 
                println!("{}[ERROR]{} {}", RED, RESET, e);

                std::process::exit(1);
            }
        };

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

            sleep(Duration::from_millis(pair.ms as u64));

            continue;

        }

        let freq: f32 = FREQ_TABLE[pair.note as usize];

        if verbose {
            println!("{}[SONG]{} {}Hz", GREEN, RESET, freq);
        }

        player.beep(freq, pair.ms, vol);

    }

}