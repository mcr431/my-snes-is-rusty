mod cpu;

use std::fs;
use std::io::{Read, Error};

use cpu::cpu::CPU;

fn main() -> Result<(), Error> {
    // todo -> read LinkToThePast
    // todo -> chunk into groups of 2 chars
    // todo -> parse the two chars as hex byte
    // at this point we have Vec<u8> of size ~256k
    // todo -> chop first 512 elements (header)
    // construct CPU
    // todo -> pass the chopped Vec to CPU

    let rom = load_rom("LinkToThePast")?;
    let mut cpu = CPU::new();
    cpu.run(rom.into_iter().skip(512).collect());

    Ok(())
}

type Rom = Vec<u8>;

fn load_rom(rom_name: &str) -> Result<Rom, Error> {
    let file_path = format!("/Users/matthewrusso/rust/my_snes_is_rusty/data/{}.smc", rom_name);

    let f = fs::File::open(file_path)?;
    let bytes: Vec<u8> = f.bytes().map(|c| c.unwrap()).collect();

    Ok(bytes)
}

// old -> reads chars of hexdump

//fn read_file() -> Result<Vec<char>, Error> {
//    let rom_bytes = fs::read("/Users/matthewrusso/rust/my_snes_is_rusty/data/LinkToThePast.dat")?;
//    let rom = String::from_utf8_lossy(&rom_bytes);
//    Ok(rom.chars().filter(|c| c != &'\n').collect())
//}
//
//fn skip_header_and_print() {
//    match read_file() {
//        Ok(bytes) => {
//            let skipped: Vec<char> = bytes.into_iter().skip(1024).collect();
//            for n in 0..100 {
//                println!("byte {} is {}", n, skipped[n]);
//            }
//        },
//        Err(e) => println!("Error reading file: {:?}", e),
//    }
//}


