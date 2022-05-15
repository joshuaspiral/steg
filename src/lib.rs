use color_eyre::Result;
use colored::Colorize;
use image::{Rgb, RgbImage};
use std::{
    io::{stdin, Read},
    path::Path,
    process::exit,
};

pub mod cli;

#[derive(Debug, Clone)]
pub struct Image {
    pub img: RgbImage,
    pub dimensions: (u32, u32),
    pub max_size: u32,
}

impl Image {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let img = image::open(path)?.to_rgb8();
        let (w, h) = img.dimensions();
        let max_size = w * h * 3;

        Ok(Image {
            img,
            dimensions: (w, h),
            max_size,
        })
    }

    #[inline]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        Ok(self.img.save(path)?)
    }

    #[inline]
    pub fn wipe(&mut self) {
        wipe(&mut self.img)
    }

    pub fn encode(&mut self, target: &str) -> Result<()> {
        self.wipe();

        let msg = read_msg()?;
        let len = (msg.len() * 8) as u32;
        let (w, _h) = self.dimensions;

        if len > self.max_size {
            eprintln!(
                "{}\n{}{3}\n{}{3}",
                "Message is too long!".red().bold(),
                format!("- message length: {}", len / 8).yellow(),
                format!("- max size: {}", self.max_size / 8).yellow(),
                "B".yellow()
            );
            exit(1);
        }

        let bit_string: String = msg
            .chars()
            .map(|byte| format!("{:0>8b}", byte as u8))
            .collect();

        if Path::new(target).exists() {
            let mut img = Image::new(target)?;
            img.wipe();
            img.save(target)?;
        }

        for (target_bit, (x, y, Rgb([r, g, b]))) in
            bit_string.chars().zip(self.img.enumerate_pixels_mut())
        {
            let band = match (y * w + x) / len {
                0 => r,
                1 => g,
                2 => b,
                _ => unreachable!(),
            };

            let lsb = *band & 1;
            if lsb != target_bit.to_digit(2).expect("Digit conversion failed") as u8 {
                *band ^= 1;
            }
        }

        self.save(target)
    }

    pub fn decode(&self) {
        let mut red_bs = String::new();
        let mut green_bs = String::new();
        let mut blue_bs = String::new();

        for px in self.img.pixels() {
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

        let mut len = 0;
        for byte in (0..bit_string.len())
            .step_by(8)
            .map(|i| &bit_string[i..i + 8])
            .take_while(|byte| *byte != "0".repeat(8))
        {
            let byte = u8::from_str_radix(byte, 2).expect("Not a binary number!");
            print!("{}", byte as char);
            len += 1;
        }

        println!(
            "{} {} {} {}",
            "<<".red(),
            "EOF - message length".red().bold(),
            format!("{len}B").yellow().bold(),
            ">>".red()
        );
    }
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

fn wipe(img: &mut RgbImage) {
    for Rgb(bands) in img.pixels_mut() {
        for band in bands {
            if *band & 1 == 1 {
                *band ^= 1;
            }
        }
    }
}
