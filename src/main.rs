#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate rodio;

extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate imgui_glutin_support;

use rodio::Source;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::Arc;

mod webserver;
mod config;

use imgui::*;

mod support_gfx;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let mut file = File::open("twitchdeck.toml").expect("File not found.");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read file.");
    let config: config::Config = toml::from_str(&contents).expect("Cannot parse file");
    let thread_safe_config : Arc<config::Config> = Arc::new(config.clone());

    thread::spawn(move || {
        let web_server_config = thread_safe_config.clone();
        webserver::start_server(web_server_config);
    });
    
    support_gfx::run("Twitchdeck Server".to_owned(), &config, CLEAR_COLOR, main_loop);
}

fn main_loop<'a>(config: &config::Config, ui: &Ui<'a>) -> bool {
    ui.window(im_str!("Play Sounds"))
        .size((300.0, 100.0), ImGuiCond::FirstUseEver)
        .build(|| {
            for effect in &config.sounds {
                if ui.button(im_str!("{}", effect.label), (250.0, 25.0)) {
                    let device = rodio::default_output_device().unwrap();
                    let file = File::open(&effect.filename).unwrap();
                    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                    rodio::play_raw(&device, source.amplify(32.0).convert_samples());
                }
            }
        });

    true
}
