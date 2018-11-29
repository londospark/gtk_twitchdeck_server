#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub sounds: Vec<SoundEffect>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundEffect {
    pub filename: String,
    pub label: String,
}