use std::{array, io::{stdout, Read, Write}, process::exit};

fn u32_to_85(input: u32) -> [u8; 5] {
    let input = input.to_be();

    array::from_fn(|i| {
         (((input / 85u32.pow(4 - i as u32)) % 85) + 33) as u8
    })
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut file = std::io::BufReader::new(
        std::fs::File::open(args.get(1).unwrap_or_else(|| {
            eprintln!("No file passed");
            exit(1);
        }))
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
            for i in (4 - size)..4 {
                buf[i] = 0;
            }
        }

        let buf_u32: u32 = unsafe { std::mem::transmute_copy(&buf) };
        let chars = u32_to_85(buf_u32);

        for i in chars.into_iter() {
            if printed == 80 {
                print!("\n");
                printed = 0;
            }
            print!("{}", i as char);
            printed += 1;
        }

        //println!(
        //    "{:?}",
        //    chars
        //        .into_iter()
        //        .map(|x| unsafe { char::from_u32_unchecked(x as u32) })
        //        .collect::<String>()
        //);

        size = file.read(&mut buf).unwrap_or_else(|_| {
            eprintln!("Invalid read");
            exit(1);
        });
    }

    print!("~>");
    stdout().flush().unwrap();
}
