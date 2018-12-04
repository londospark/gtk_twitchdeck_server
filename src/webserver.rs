extern crate actix_web;
extern crate futures;

use self::actix_web::{http, server, App, HttpRequest, HttpResponse, Json};
use ears::{AudioController, Sound};
use std::sync::Arc;
use std::thread;
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
    let config_state = Arc::new(config_state.clone());
    thread::spawn(move || {
        let config_state = config_state.clone();
        let sound_to_play = &config_state
            .sounds
            .iter()
            .filter(|item| item.label == body.sound)
            .next()
            .unwrap()
            .filename;
        let mut sound = Sound::new(&sound_to_play).unwrap();
        sound.play();
        while sound.is_playing() {}
    });
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
