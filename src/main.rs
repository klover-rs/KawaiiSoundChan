use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use tokio::time::{Duration, sleep};
use std::io::stdin;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
struct Options {
    playback_speed: f32,
    playback_volume: f32,
    time_interval_secs: u64,
}

fn read_json_file(file_path: &str) -> Result<Options, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file_path)?;

    // Read the file content into a string
    let mut content = String::new();
    file.take(u64::MAX.into()).read_to_string(&mut content)?;

    // Deserialize the JSON content
    let data: Options = serde_json::from_str(&content)?;

    Ok(data)
}

#[tokio::main]
async fn main() {

    let config_file = "config.json";

    let options = match read_json_file(config_file) {
        Ok(options) => options,
        Err(e) => {
            println!("Failed to read config file: {}", e);
            return;
        }
    };

    println!("provide the full path to your sound");

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let res = match buffer.trim_end() {
        "" => "Ah, well I can respect your wish to remain anonymous.".into(),
        filepath => filepath,
    };

    loop {
        play_sound(&res, options.playback_speed, options.playback_volume).await;

        sleep(Duration::from_secs(options.time_interval_secs)).await;
    }

    
}

async fn play_sound(file_path: &str, speed: f32, volume: f32) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(file_path).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.set_volume(volume);
    sink.set_speed(speed);
    sink.sleep_until_end();
}