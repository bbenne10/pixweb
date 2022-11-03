use std::collections::HashSet;

use regex::{Captures, Regex};

#[derive(Clone, Copy)]
enum PrimaryColor {
    Red = 1,
    Green = 2,
    Blue = 3,
}

impl PrimaryColor {
    fn component_from_capture(&self, capture: &Captures) -> u8 {
        let long = capture.get(*self as usize);
        let short = capture.get(*self as usize + 3);

        match (long, short) {
            (Some(long_color), None) => {
                match hex::decode(long_color.as_str().to_owned().as_bytes()) {
                    Ok(result) => result[0],
                    Err(_) => panic!("Can't unpack {} as hex int", long_color.as_str()),
                }
            }
            (None, Some(short_color)) => {
                match hex::decode(lengthen(short_color.as_str()).as_bytes()) {
                    Ok(result) => result[0],
                    Err(_) => panic!("Can't unpack {} as hex int", short_color.as_str()),
                }
            }
            (Some(_), Some(_)) => unreachable!(),
            (None, None) => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl std::fmt::Display for &Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{}",
            hex::encode_upper(vec![self.red, self.green, self.blue])
        )
    }
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub fn from_capture(capture: Captures) -> Option<Color> {
        let red = PrimaryColor::Red.component_from_capture(&capture);
        let green = PrimaryColor::Green.component_from_capture(&capture);
        let blue = PrimaryColor::Blue.component_from_capture(&capture);
        Some(Color::new(red, green, blue))
    }
}

fn lengthen(s: &str) -> String {
    // TODO: don't panic here. Option instead?
    match s.len() {
        1 => format!("{}{}", s, s),
        2 => s.to_string(),
        _ => panic!("Only know how to handle 1-2 char strings; got {}", s),
    }
}

pub fn find_matches(content: &str) -> Vec<Color> {
    lazy_static! {
        static ref COLOR_RE: Regex = Regex::new(
            r"\b#?([a-fA-F\d]{2})([a-fA-F\d]{2})([a-fA-F\d]{2})\b|\b#?([a-fA-F\d])([a-fA-F\d])([a-fA-F\d])\b"
        ).unwrap();
    }

    let mut colors: HashSet<Color> = HashSet::new();
    for cap in COLOR_RE.captures_iter(content) {
        log::info!("cap: {:?}", cap);
        if let Some(color) = Color::from_capture(cap) {
            colors.insert(color);
        }
    }
    colors.drain().collect()
}
