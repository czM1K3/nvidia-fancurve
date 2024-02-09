use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use signal_hook::low_level::exit;

pub fn parse_json() -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("nvidia-fancurve").unwrap();
    let path_raw = xdg_dirs.find_config_file("config.json");
    let path = match path_raw {
        None => {
            eprintln!("Error reading config file");
            exit(1);
        }
        Some(v) => v,
    };
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let data: HashMap<i32, i32> = serde_json::from_reader(reader)?;

    Ok(data)
}
