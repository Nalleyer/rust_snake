use ron::de::from_str;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Context {
    pub scale: f32,
    pub tick_duration: f64,
}

impl Context {
    pub fn new() -> Context {
        let file_content =
            fs::read_to_string("resources/context.ron").expect("reading context setting");
        from_str(&file_content).expect("parsing context")
    }
}
