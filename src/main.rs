mod file_format;
mod table_byte_to_note;
mod audio;
mod freq_table;
mod read_csv;
mod view;

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
use crate::view::error;
use crate::view::help;
use crate::view::info;
use crate::view::melody;
use crate::view::song;
use crate::view::success;
use crate::view::version;

fn main() {

    let player = AudioPlayer::new();

    let mut verbose = false;

    let mut vol: f32 = 0.3;
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        help();
        return;
    }

    if args[1] == "--version" || args[1] == "-v" {
        version();
        return;
    }

    let path = Path::new(&args[1]);

    if args.contains(&String::from("--verbose")) {
        verbose = true;
        info(&format!("Path read: {}", args[1]));
    }

    if args[2] == "-o" {

        let data_csv = match read_csv(path) {
            Ok(data) => data,
            Err(e) => {

                error(&e.to_string());
                
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
                    success(&format!("Writing the {} successfully completed!", args[3]));
                }

                std::process::exit(0);
            }
            Err(e) => { 
                error(&e.to_string());

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
                            success(&format!("volume set to {}", (vol * 100.0) as u8));
                        }

                    }
                    Err(e) => error(&format!("Conversion error: {}", e))
                }

            },
            _ => {}
        };

    }

    let x360_file = match read(path)  {
        Ok(x) => x,
        Err(e) => {

            error(&format!("Error processing file: {}", e));
            
            std::process::exit(1);

        },
    };

    if verbose {

        info(&format!("File version: {}", x360_file.version));

        info(&format!("File name: {}", std::str::from_utf8(&x360_file.name).unwrap_or("?")));

        info(&format!("Volume: {}", vol));

        info(&format!("Notes read:"));

    }

    for pair in &x360_file.pairs  {

        let string_note: &str;
        
        match get_note_by_byte(pair.note) {
            Some(s) => string_note = s,
            None => string_note = "none",
        };
        
        if verbose {
            melody(&format!("{}: {}ms", string_note, pair.ms));
        }

    }

    for pair in &x360_file.pairs  {

        if pair.note == 255 {
        
            if verbose {
                song(&"0Hz".to_string());
            }

            sleep(Duration::from_millis(pair.ms as u64));

            continue;

        }

        let freq: f32 = FREQ_TABLE[pair.note as usize];

        if verbose {
            song(&format!("{}Hz", freq));
        }

        player.beep(freq, pair.ms, vol);

    }

}