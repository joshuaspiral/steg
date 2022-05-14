use color_eyre::Result;
use colored::Colorize;
use image::Rgb;
use std::{
    io::{stdin, Read},
    process::exit,
};

pub mod cli;

pub fn encode(src: &str, target: &str, band: &str) -> Result<()> {
    let mut img = image::open(src)?.to_rgb8();
    let msg = read_msg()?;

    let (w, h) = img.dimensions();
    let max_size = w * h;
    let len = (msg.len() * 8) as u32;

    if len >= max_size {
        eprintln!(
            "{}\n{}{3}\n{}{3}",
            "Message is too long!".red().bold(),
            format!("- message length: {len}").yellow(),
            format!("- max size: {max_size}").yellow(),
            "B".yellow()
        );
        exit(1);
    }

    let bit_string: String = msg
        .chars()
        .map(|byte| format!("{:0>8b}", byte as u8))
        .collect();

    wipe(target, band)?;
    for (target_bit, (_, _, Rgb([r, g, b]))) in bit_string.chars().zip(img.enumerate_pixels_mut()) {
        let band = match band {
            "r" => r,
            "g" => g,
            "b" => b,
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

pub fn wipe(target: &str, band: &str) -> Result<()> {
    let mut img = image::open(target)?.to_rgb8();
    for (_, _, Rgb([r, g, b])) in img.enumerate_pixels_mut() {
        let band = match band {
            "r" => r,
            "g" => g,
            "b" => b,
            _ => unreachable!(),
        };

        if *band & 1 == 1 {
            *band ^= 1;
        }
    }
    img.save(target)?;
    Ok(())
}

pub fn decode(src: &str, band: &str) -> Result<()> {
    let img = image::open(src)?.to_rgb8();
    let mut bit_string = String::new();

    for px in img.pixels() {
        let Rgb([r, g, b]) = px;

        let band = match band {
            "r" => r,
            "g" => g,
            "b" => b,
            _ => unreachable!(),
        };

        let lsb = (band & 1).to_string();
        bit_string.push_str(&lsb)
    }

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
