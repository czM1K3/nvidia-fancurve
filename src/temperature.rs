use std::process::Command;

pub fn get_temperature() -> Option<i32> {
    let out = Command::new("nvidia-smi")
        .args(["--query-gpu=temperature.gpu", "--format=csv,noheader"])
        .output()
        .expect("Error");
    if out.status.success() {
        let out_str = String::from_utf8(out.stdout).unwrap();
        Some(out_str.trim().parse::<i32>().unwrap())
    } else {
        None
    }
}
