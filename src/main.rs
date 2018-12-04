#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate ears;

use ears::{AudioController, Sound};
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::Arc;

mod webserver;
mod config;

fn main() {

    let mut file = File::open("twitchdeck.toml").expect("File not found.");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read file.");
    let config: config::Config = toml::from_str(&contents).expect("Cannot parse file");
    let thread_safe_config : Arc<config::Config> = Arc::new(config.clone());

    for effect in &config.sounds {
        let ref_file = Arc::new(effect.filename.to_owned());

        move || {
            let outer_clone = ref_file.clone();
            thread::spawn(move || {
                let clone_file = outer_clone.clone();
                let mut sound = Sound::new(&clone_file).unwrap();
                sound.play();
                while sound.is_playing() {}
            });
        };
    }

    
    thread::spawn(move || {
        let web_server_config = thread_safe_config.clone();
        webserver::start_server(web_server_config);
    });

}
