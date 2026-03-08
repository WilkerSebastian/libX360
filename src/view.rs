pub const RESET: &str = "\x1b[0m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";

pub fn help() {

    println!(r#"
        Uses: 
            x360 <file.x360> [options]                      Play file in x360 format

            x360 <file.csv> -o <file.x360> [options]        CSV encoding in x360 format

        Options:
            --volume <value>                                To change the playback volume from 0 to 100
            --verbose                                       Show all the logs of what is being done

        Commands:
            --version | -v                                  Show the installed version
            --help   | -h                                   Show this help
    "#)

}

pub fn version() {

    println!(r#"
x   x  xxx    xxxx  xxx 
 x x      x  x     x   x
  x    xxxxx xxxx  x   x
 x x      x  x   x x   x
x   x  xxx    xxxx  xxx 

        V1.0.0
    "#);

}

pub fn info(msg: &String) {

    println!("{}[INFO]{} {}", BLUE, RESET, msg);

}

pub fn error(msg: &String) {

    println!("{}[ERROR]{} {}", RED, RESET, msg);

}

pub fn melody(msg: &String) {

    println!("{}[MELODY]{} {}", YELLOW, RESET, msg);

}

pub fn success(msg: &String) {

    println!("{}[SUCCESS]{} {}", GREEN, RESET, msg);

}

pub fn song(msg: &String) {

    println!("{}[SONG]{} {}", GREEN, RESET, msg);

}