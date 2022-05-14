pub mod cli;
use image::Rgb;
use std::{
    error::Error,
    io::{stdin, Read},
};

pub fn encode(flags: &cli::Flags) -> Result<(), Box<dyn Error>> {
    let mut img = image::open(&flags.src)?.to_rgb8();
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf)?;
    let msg = String::from_utf8(buf)?;

    let mut bit_string = String::new();
    for byte in msg.chars() {
        bit_string.push_str(&format!("{:0>8b}", byte as u8));
    }

    for bit in bit_string.chars().zip(img.enumerate_pixels_mut()) {
        let (target_bit, (_x, _y, Rgb([r, g, b]))) = bit;
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

pub fn decode(flags: &cli::Flags) -> Result<(), Box<dyn Error>> {
    let img = image::open(&flags.src)?.to_rgb8();
    let mut bit_string = String::new();
    
    for pxl in img.pixels() {
        let Rgb([r, g, b]) = pxl;
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
        let byte = &bit_string[i..i+8];
        if byte == "00000000" { break; }
        let byte = u8::from_str_radix(byte, 2).expect("Not a binary number!");
        print!("{}", byte as char);
    }
    Ok(())
}
