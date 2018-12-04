extern crate actix_web;
extern crate futures;

use self::actix_web::{http, server, App, HttpRequest, HttpResponse, Json};
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use webserver::actix_web::Responder;
use webserver::actix_web::State;

use config::Config;

#[derive(Deserialize, Debug)]
struct SoundRequest {
    sound: String,
}

#[derive(Serialize, Debug)]
struct Sounds {
    names: Vec<String>,
}

fn get_sounds(req: &HttpRequest<Arc<Config>>) -> self::actix_web::Result<Json<Sounds>> {
    let mut names = Vec::<String>::new();
    for sound in &req.state().sounds {
        names.push((&sound.label).to_string());
    }
    println!("{:?}", req.state());
    Ok(Json(Sounds { names: names }))
}

fn index(_req: &HttpRequest<Arc<Config>>) -> &'static str {
    "Hello world!"
}

fn play_sound(body: Json<SoundRequest>, config_state: State<Arc<Config>>) -> impl Responder {
    println!("{:?}", body);
    let config_state = config_state.clone();
    let device = rodio::default_output_device().unwrap();
    let sound_to_play = &config_state
        .sounds
        .iter()
        .filter(|item| item.label == body.sound)
        .next()
        .unwrap()
        .filename;
    let file = File::open(sound_to_play).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    rodio::play_raw(&device, source.amplify(32.0).convert_samples());
    HttpResponse::Ok()
}

pub fn start_server(config: Arc<Config>) {
    let config = config.clone();
    server::new(move || {
        let config = config.clone();
        App::with_state(config)
            .resource("/", |r| r.f(index))
            .resource("/get_sounds", |r| r.f(get_sounds))
            .resource("/play_sound", |r| {
                r.method(http::Method::POST).with(play_sound)
            })
    }).bind("0.0.0.0:8087")
    .unwrap()
    .run();
}
