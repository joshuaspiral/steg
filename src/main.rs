use std::{env, error::Error, io::{stdin, Read}};
use image::Rgb;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf)?;
    let msg = String::from_utf8(buf)?;
    let mut img = image::open(env::args().nth(1).unwrap())?.to_rgb8();

    let mut bit_string = String::new();
    for byte in msg.chars() {
        bit_string.push_str(&format!("{:0>8b}", byte as u8));
    }

    for bit in bit_string.chars().zip(img.enumerate_pixels_mut()) {
        let (target_bit, (x, y, Rgb([r, g, b]))) = bit;
        let r_lsb = *r & 1;
        if r_lsb != (target_bit as u8 - 48) {
            *r ^= 1;
        }
    }

    img.save("out.png")?;
    Ok(())
}
