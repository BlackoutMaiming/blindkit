use colored::Colorize;
use figlet_rs::FIGfont;

use std::path::Path;
use image;
use image::GenericImageView;
use std::fs::File;
use std::io::Read;

fn read_in() -> String {
    let mut x = String::new();
    std::io::stdin().read_line(&mut x).expect("couldn't read in");
    x
}

fn halt() {
    println!("Press {} to close.", "ENTER".italic().bold().red());
    read_in();
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

fn main() {
    let sheen_effect = image::open("sheen.png"); //21

    if sheen_effect.is_err() {
        println!("{} {}", "Cannot open".red(), "sheen.png".bold());
        halt();
        return
    }

    if !Path::new("cfg.json").exists() {
        println!("{} {}", "cfg.json".italic().bold(), "does not exist.".red());
        halt();
        return
    }

    let cfg_raw = File::open("cfg.json");

    if cfg_raw.is_err() {
        println!("{} {}", "Cannot open".red(), "cfg.json".bold());
        halt();
        return
    }

    let mut cfg_txt = String::new();

    // Copy contents of file to a mutable string
    let results = cfg_raw.unwrap().read_to_string(&mut cfg_txt);

    if results.is_err() {
        println!("{}", "Could not read contents of cfg.json".red());
        halt();
        return;
    }

    let json_results = serde_json::from_str(cfg_txt.as_str());

    if json_results.is_err() {
        println!("{}", "cfg.json is not valid JSON.".red());
        halt();
        return;
    }

    let cfg: serde_json::Value = json_results.unwrap();

    let sheen_effect = sheen_effect.unwrap();

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("blindkit");
    println!("{}", figure.unwrap().to_string().purple());

    println!("{}", "🛣️ path to blind:".bold());
    let path = read_in().trim().to_string();

    if !Path::new(path.as_str()).exists() {
        println!("{} {}", path.bold(), "does not exist.".red());
        halt();
        return
    }

    let img = image::open(path.clone());

    if img.is_err() {
        println!("{} {}", "Cannot open".red(), path.bold());
        halt();
        return
    }

    let loaded = img.unwrap();

    if loaded.width() != 34 || loaded.height() != 34 {
        println!("{}", format!("Image is incorrect size. Expected {}x{}, got {}x{}.", 34, 34, loaded.width(), loaded.height()).red());
        halt();
        return
    }

    if sheen_effect.width() != 714 || sheen_effect.height() != 34 {
        println!("{}", format!("Sheen is incorrect size. Expected {}x{}, got {}x{}.", 714, 34, sheen_effect.width(), sheen_effect.height()).red());
        halt();
        return
    }

    println!("🖼️ Creating sprite...");

    let mut sprite = image::RgbaImage::new(sheen_effect.width(), sheen_effect.height());

    for i in 0..714/34 {
        for x in 1..loaded.width() {
            for y in 1..loaded.height() {
                let mut px = loaded.get_pixel(x, y);
                let sheen = sheen_effect.get_pixel(i*34 + x, y);
                if sheen.0[3] > 0 {
                    px.0[0] = lerp(px.0[0] as f32, cfg["sheen_colour"]["r"].as_i64().expect("Cannot convert RGB value. (R)") as f32, cfg["sheen_amount"].as_f64().expect("Cannot get alpha interpolation value. (NAN)") as f32) as u8;
                    px.0[1] = lerp(px.0[1] as f32, cfg["sheen_colour"]["g"].as_i64().expect("Cannot convert RGB value. (G)") as f32, cfg["sheen_amount"].as_f64().expect("Cannot get alpha interpolation value. (NAN)") as f32) as u8;
                    px.0[2] = lerp(px.0[2] as f32, cfg["sheen_colour"]["b"].as_i64().expect("Cannot convert RGB value. (B)") as f32, cfg["sheen_amount"].as_f64().expect("Cannot get alpha interpolation value. (NAN)") as f32) as u8;
                }
                sprite.put_pixel(i*34 + x, y, px);
            }
        }
    }

    println!("💾 Writing to file...");

    let results = sprite.save("output.png");

    if results.is_err() {
        println!("{}", "Cannot write to file. Please check that you have enough space and your disk is not read-only.".red());
    }
    halt();
}