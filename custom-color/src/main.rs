use regex::Regex;
use std::{
    fs,
    io::{stdin, stdout, Write},
    path::Path,
};

const PATCH: &str = include_str!("../safehouse.entity.patch.json");

fn main() -> std::io::Result<()> {
    let mut color = String::new();

    print!("Please enter your desired color>> ");
    stdout().flush()?;
    stdin().read_line(&mut color)?;

    color = color.trim().to_string();

    let has_leading_sharp = color.starts_with('#');

    if !has_leading_sharp {
        color = String::from('#') + color.as_str();
    }

    if !Regex::new(r"^#[A-F0-9a-f]{6}$")
        .expect("failed to check if color is valid")
        .is_match(&color)
    {
        panic!("invalid color! {} is not a valid hex color.", &color);
    }

    fs::write(
        Path::new(".")
            .join("custom")
            .join("chunk29")
            .join("custom-safehouse.entity.patch.json"),
        PATCH.replace("%color%", &color),
    )
}
