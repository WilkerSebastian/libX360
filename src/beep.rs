use std::process::{Command, ExitStatus};

pub fn beep(freq: f32, ms: u16, vol: f32) -> ExitStatus {

    let secs = (ms as f32) / 1000.0;

    let args_str = format!("-n -q synth {:.3} sine {:.6} vol {:.2}", secs, freq, vol);
    let args: Vec<&str> = args_str.split_whitespace().collect();

    let status = Command::new("play")
        .args(args)
        .status()
        .expect("[ERROR] Falha ao executar comando play do sox");

    return status;

}