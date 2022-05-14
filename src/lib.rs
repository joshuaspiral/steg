use color_eyre::Result;
use image::Rgb;
use std::{
    io::{stdin, Read},
    process::exit,
};

pub mod cli;

pub fn encode(flags: &cli::Flags) -> Result<()> {
    let mut img = image::open(&flags.src)?.to_rgb8();
    let msg = read_msg()?;

    let (w, h) = img.dimensions();
    let max_size = w * h;

    if msg.len() * 8 >= max_size as usize {
        eprintln!("Message size is too large! You inputted {} characters when the image could only fit {}.", msg.len() * 8, max_size);
        exit(1);
    }

    let bit_string: String = msg
        .chars()
        .map(|byte| format!("{:0>8b}", byte as u8))
        .collect();

    for (target_bit, (_, _, Rgb([r, g, b]))) in bit_string.chars().zip(img.enumerate_pixels_mut()) {
        let band = match flags.band.as_ref().unwrap().to_ascii_lowercase().as_str() {
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

    img.save(&flags.target)?;
    Ok(())
}

pub fn decode(flags: &cli::Flags) -> Result<()> {
    let img = image::open(&flags.src)?.to_rgb8();
    let mut bit_string = String::new();

    for px in img.pixels() {
        let Rgb([r, g, b]) = px;

        let band = match flags.band.as_ref().unwrap().to_ascii_lowercase().as_str() {
            "r" => r,
            "g" => g,
            "b" => b,
            _ => unreachable!(),
        };

        let lsb = (band & 1).to_string();
        bit_string.push_str(&lsb)
    }

    for i in (0..bit_string.len()).step_by(8) {
        let byte = &bit_string[i..i + 8];

        if byte == "0".repeat(8) {
            break;
        }

        let byte = u8::from_str_radix(byte, 2).expect("Not a binary number!");
        print!("{}", byte as char);
    }

    Ok(())
}

fn read_msg() -> Result<String> {
    println!("<< enter message >>");

    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}
