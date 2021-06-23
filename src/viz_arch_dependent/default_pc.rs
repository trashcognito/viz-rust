use std::fs::File;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Write};
use rand;
pub fn load_program(program_space:&mut [u16; 0x10000]) {
    let mut line = String::new();
    println!("Enter file path to run:");
    std::io::stdout()
        .flush()
        .expect("Could not flush stdout!");
    std::io::stdin().read_line(&mut line)
        .expect("Could not get file path from user!");
    println!("Opening {}", line.trim());
    let mut f = File::open(line.trim())
        .expect("Could not open file for reading!");
    for i in 0..65536 {
        match f.read_u16::<BigEndian>() {
            Ok(val) => {
                program_space[i] = val;
            }
            Err(_) => {
                break;
            }
        }
    }
}
pub fn io_in(port:u16) -> u16 {
    return if port == 0 {
        std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u16)
            .expect("Could not get input from user!")
    } else {
        print!("Port {} requested data: ", port);
        let mut output = String::new();
        std::io::stdin().read_line(&mut output)
            .expect("Could not get input from user!");
        output.parse::<u16>()
            .expect("Could not get u16 from user!")
    }
}
pub fn io_out(data:u16, port:u16) {
    if port == 0 {
        std::io::stdout().write(&[data as u8])
            .expect("Could not write to stdout!");
    } else {
        println!("Port {} wrote data {}", port, data);
    }
}
pub fn entropy() -> u16 {
    rand::random()
}
pub fn pow16(base:u16, exp:u16) -> u16 {
    return base.pow(exp as u32)
}