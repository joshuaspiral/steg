use color_eyre::Result;
use colored::Colorize;
use image::Rgb;
use std::{
    io::{stdin, Read},
    process::exit,
};

pub mod cli;

pub fn encode(src: &str, target: &str) -> Result<()> {
    let mut img = image::open(src)?.to_rgb8();
    let msg = read_msg()?;

    let (w, h) = img.dimensions();
    let max_size = w * h * 3;
    let len = (msg.len() * 8) as u32;

    if len >= max_size {
        eprintln!(
            "{}\n{}{3}\n{}{3}",
            "Message is too long!".red().bold(),
            format!("- message length: {}", len / 8).yellow(),
            format!("- max size: {}", max_size / 8).yellow(),
            "B".yellow()
        );
        exit(1);
    }

    let bit_string: String = msg
        .chars()
        .map(|byte| format!("{:0>8b}", byte as u8))
        .collect();

    wipe(target)?;
    for (target_bit, (x, y, Rgb([r, g, b]))) in bit_string.chars().zip(img.enumerate_pixels_mut()) {
        let band = match (y * w + x) / len {
            0 => r,
            1 => g,
            2 => b,
            _ => unreachable!(),
        };

        let lsb = *band & 1;
        if lsb != (target_bit as u8 - 48) {
            *band ^= 1;
        }
    }

    img.save(target)?;
    Ok(())
}

pub fn wipe(target: &str) -> Result<()> {
    let mut img = image::open(target)?.to_rgb8();
    for Rgb(bands) in img.pixels_mut() {
        for band in bands {
            if *band & 1 == 1 {
                *band ^= 1;
            }
        }
    }
    img.save(target)?;
    Ok(())
}

pub fn decode(src: &str) -> Result<()> {
    let img = image::open(src)?.to_rgb8();

    let mut red_bs = String::new();
    let mut green_bs = String::new();
    let mut blue_bs = String::new();

    for px in img.pixels() {
        let Rgb([r, g, b]) = px;

        let r_lsb = (r & 1).to_string();
        let g_lsb = (g & 1).to_string();
        let b_lsb = (b & 1).to_string();
        red_bs.push_str(&r_lsb);
        green_bs.push_str(&g_lsb);
        blue_bs.push_str(&b_lsb);
    }

    let bit_string = format!("{red_bs}{green_bs}{blue_bs}");

    println!(
        "{} {} {}",
        "<<".green(),
        "begin decoded message".green().bold(),
        ">>".green()
    );

    (0..bit_string.len())
        .step_by(8)
        .map(|i| &bit_string[i..i + 8])
        .take_while(|byte| *byte != "0".repeat(8))
        .for_each(|byte| {
            let byte = u8::from_str_radix(byte, 2).expect("Not a binary number!");
            print!("{}", byte as char);
        });

    println!("{} {} {}", "<<".red(), "EOF".red().bold(), ">>".red());

    Ok(())
}

fn read_msg() -> Result<String> {
    println!(
        "{} {} {}",
        "<<".blue(),
        "enter message".blue().bold(),
        ">>".blue()
    );

    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf)?;
    let s = String::from_utf8(buf)?;

    println!(
        "{} {} {}{} {}",
        "<<".green(),
        "read message of size".green().bold(),
        s.len().to_string().yellow().bold(),
        "B".yellow().bold(),
        ">>".green()
    );

    Ok(s)
}
