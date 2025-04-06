use colored::Colorize;
use figlet_rs::FIGfont;

use std::path::Path;
use image;
use image::{GenericImageView, Rgba};
use std::fs::File;
use std::io::Read;
use serde_json::Value;

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

fn add_sheen(sheen:Rgba<u8>, mut px:Rgba<u8>, cfg:&Value) -> Rgba<u8> {
    if sheen.0[3] > 0 {
        px.0[0] = lerp(px.0[0] as f32, cfg["sheen_colour"]["r"].as_i64().expect("Cannot convert RGB value. (R)") as f32, cfg["sheen_amount"].as_f64().expect("Cannot get alpha interpolation value. (NAN)") as f32) as u8;
        px.0[1] = lerp(px.0[1] as f32, cfg["sheen_colour"]["g"].as_i64().expect("Cannot convert RGB value. (G)") as f32, cfg["sheen_amount"].as_f64().expect("Cannot get alpha interpolation value. (NAN)") as f32) as u8;
        px.0[2] = lerp(px.0[2] as f32, cfg["sheen_colour"]["b"].as_i64().expect("Cannot convert RGB value. (B)") as f32, cfg["sheen_amount"].as_f64().expect("Cannot get alpha interpolation value. (NAN)") as f32) as u8;
        return px;
    }
    px
}

fn main() {
    //Config
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

    let cfg: Value = json_results.unwrap();

    //Sheen
    let sheen34x = image::open("sheen34x.png"); //21
    let sheen68x = image::open("sheen68x.png"); //21

    if sheen34x.is_err() {
        println!("{} {}", "Cannot open".red(), "sheen34x.png".bold());
        halt();
        return
    }
    else if sheen68x.is_err() {
        println!("{} {}", "Cannot open".red(), "sheen68x.png".bold());
        halt();
        return
    }

    let sheen34x = sheen34x.unwrap();
    let sheen68x = sheen68x.unwrap();

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

    if (loaded.width() != 34 || loaded.height() != 34) && (loaded.width() != 68 || loaded.height() !=68) {
        println!("{}", format!("Image is incorrect size. Expected 34x34 or 68x68, got {}x{}.", loaded.width(), loaded.height()).red());
        halt();
        return
    }

    if sheen34x.width() != 714 || sheen34x.height() != 34 {
        println!("{}", format!("Sheen34x is incorrect size. Expected {}x{}, got {}x{}.", 714, 34, sheen34x.width(), sheen34x.height()).red());
        halt();
        return
    }

    if sheen68x.width() != 1428 || sheen68x.height() != 68 {
        println!("{}", format!("Sheen68x is incorrect size. Expected {}x{}, got {}x{}.", 1428, 68, sheen68x.width(), sheen68x.height()).red());
        halt();
        return
    }

    println!("🖼️ Creating sprite...");
    let mut sprite = image::RgbaImage::new(1,1);
    if loaded.width() == 34 && loaded.height() == 34 {
        sprite = image::RgbaImage::new(sheen34x.width(), sheen34x.height());
        for i in 0..714/34 {
            for x in 1..loaded.width() {
                for y in 1..loaded.height() {
                    let mut px = loaded.get_pixel(x, y);
                    let sheen = sheen34x.get_pixel(i*34 + x, y);
                    px = add_sheen(sheen, px, &cfg);
                    sprite.put_pixel(i*34 + x, y, px);
                }
            }
        }
    }
    else if loaded.width() == 68 && loaded.height() == 68 {
        sprite = image::RgbaImage::new(sheen68x.width(), sheen68x.height());
        for i in 0..1428/68 {
            for x in 1..loaded.width() {
                for y in 1..loaded.height() {
                    let mut px = loaded.get_pixel(x, y);
                    let sheen = sheen68x.get_pixel(i*68 + x, y);
                    px = add_sheen(sheen, px, &cfg);
                    sprite.put_pixel(i*68 + x, y, px);
                }
            }
        }
    }
    else {
        println!("{}", format!("Image is incorrect size. Expected 34x34 or 68x68, got {}x{}.", loaded.width(), loaded.height()).red());
        halt();
        return
    }

    println!("💾 Writing to file...");

    let results = sprite.save(format!("{}.png", cfg["output_file"].as_str().expect("Cannot get output name.")));

    if results.is_err() {
        println!("{}", "Cannot write to file. Please check that you have enough space and your disk is not read-only.".red());
    }
    halt();
}