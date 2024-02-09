use std::process::Command;

pub fn set_fan(speed: i32) -> bool {
    let formated = format!("xhost si:localuser:root && sudo -S /usr/bin/nvidia-settings -a \"*:1[fan-0]/GPUTargetFanSpeed={}\" -a \"*:1[fan-1]/GPUTargetFanSpeed={}\" && xhost -si:localuser:root", speed, speed);
    let res = Command::new("bash")
        .args(["-c", &formated])
        .output()
        .expect("Error");
    res.status.success()
}

pub fn init_fan() -> bool {
    let res = Command::new("bash").args(["-c", "xhost si:localuser:root && sudo -S /usr/bin/nvidia-settings -a \"*:1[gpu:0]/GPUFanControlState
    =1\" && xhost -si:localuser:root"]).output().expect("Error");
    res.status.success()
}

pub fn reset_fan() -> bool {
    let res = Command::new("bash").args(["-c", "xhost si:localuser:root && sudo -S /usr/bin/nvidia-settings -a \"*:1[gpu:0]/GPUFanControlState
    =0\" && xhost -si:localuser:root"]).output().expect("Error");
    res.status.success()
}
