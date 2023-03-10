use std::{env, fs, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 4 || (args[2].clone() != *"+" && args[2].clone() != *"-") {
        println!("Usage: {} <file> <+ | -> <shift_by>", args[0].clone());
        println!("file: some file\nadd or rest: do you want to add or rest\nshift_by: shift bytes by some number");
        println!("\nExample: {} myfile.txt + 8", args[0].clone());
        return;
    }

    let decrypt = args[2] == *"-";

    match fs::read(args[1].clone()) {
        Ok(contents) => {
            let new_content = shift_bytes(contents, args[3].clone().parse::<u8>().unwrap(), decrypt);
            let mut file = fs::OpenOptions::new()
                .write(true)
                .open(args[1].clone())
                .unwrap();
            if let Err(e) = file.write_all(&new_content) {
                println!("Error: {:?}", e);
            } else {
                println!("Success! :)")
            }
        }

        Err(e) => {
            println!("Could not open file `{}`: {}", args[1], e);
        }
    }
}

fn shift_bytes(text: Vec<u8>, shift_by: u8, backwards: bool) -> Vec<u8> {
    text.iter()
        .map(|byte| {
            if backwards {
                byte.wrapping_sub(shift_by)
            } else {
                byte.wrapping_add(shift_by)
            }
        })
        .collect()
}
