use std::{array, io::Read, process::exit};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    #[arg(short, long, default_value_t = 80)]
    wrap: usize,
    #[arg(short, long, default_value_t = false)]
    decode: bool,
    file: String
}

fn u32_to_85(input: u32) -> [u8; 5] {
    let input = input.to_be();

    array::from_fn(|i| {
         (((input / 85u32.pow(4 - i as u32)) % 85) + 33) as u8
    })
}

fn main() {
    let config = Config::parse();

    let mut file = std::io::BufReader::new(
        std::fs::File::open(config.file)
        .unwrap_or_else(|_| {
            eprintln!("Invalid file name");
            exit(1);
        }),
    );

    let mut buf: [u8; 4] = [0; 4];

    let mut printed = 2;
    let mut size = file.read(&mut buf).unwrap_or_else(|_| {
        eprintln!("Invalid read");
        exit(1);
    });

    print!("<~");

    while size > 0 {
        if size != 4 {
            for item in buf.iter_mut().skip(4-size) {
                *item = 0;
            }
        }

        let buf_u32: u32 = unsafe { std::mem::transmute_copy(&buf) };
        let chars = u32_to_85(buf_u32);

        for i in chars.into_iter() {
            if printed == config.wrap {
                println!("");
                printed = 0;
            }
            print!("{}", i as char);
            printed += 1;
        }

        size = file.read(&mut buf).unwrap_or_else(|_| {
            eprintln!("Invalid read");
            exit(1);
        });
    }

    println!("~>");
}
