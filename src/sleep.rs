use std::process::{Command, ExitStatus};

pub fn sleep(ms: u16) -> ExitStatus {

    let secs = (ms as f32) / 1000.0;

    let arg = std::format!("{:.3}", secs);

    let status = Command::new("sleep")
        .arg(arg)
        .status()
        .expect("[ERROR] Falha ao executar comando sleep");

    return status;

}