use std::path::PathBuf;
use std::{env, fs::File};
use std::io::prelude::*;

enum Argument {
    Register(u8),
    Literal(u8),
    Address(u16),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filename_out = &args[2];
    let mut buf = String::new();
    let mut file = File::open(filename).expect("Failed to open file");
    file.read_to_string(&mut buf).expect("Failed to read file");
    let lines: Vec<_> = buf.lines().collect();
    let mut binary: Vec<u16> = Vec::new();
    for l in lines {
        let line = l.split(";").collect::<Vec<_>>()[0];
        // Blank line
        let line_whitespace: Vec<_> = line.split_whitespace().collect();
        if line_whitespace.join("") == "" {
            continue;
        }
        let line_components = line.split(" ").collect::<Vec<_>>();
        let mnemonic = line_components[0];
        let opcode = match mnemonic {
            "NOP" => 0x0,
            "ADD" => 0x1,
            "SUB" => 0x2,
            "DIV" => 0x3,
            "AND" => 0x4,
            "ORR" => 0x5,
            "XOR" => 0x6,
            "NOT" => 0x7,
            "LSH" => 0x8,
            "RSH" => 0x9,
            "RET" => 0xA,
            "BIR" => 0xB,
            "LDM" => 0xC,
            "STR" => 0xD,
            "LDI" => 0xE,
            "CMP" => 0xF,
            ""    => continue,
            // Psuedo Instructions
            "INC" => 0x10,
            _ => panic!("Invalid mnemonic {mnemonic}"),
        };
        let mut args: Vec<Argument> = Vec::new();
        for i in 1..line_components.len() {
            let cur = line_components[i];
            // Skip blanks
            if cur == "" {
                continue;
            }
            // Check if register
            if cur.contains("r") {
                let reg_num = &cur[1..];
                args.push(Argument::Register(u16::from_str_radix(reg_num, 16)
                    .expect(&("Invalid register ".to_owned() + reg_num)) as u8));
            } else if cur.contains("a") {
                let addr_num = &cur[1..];
                args.push(Argument::Address(addr_num.parse()
                    .expect(&("Invalid address ".to_owned() + addr_num))));
            } else {
                args.push(Argument::Literal(cur.parse::<u8>()
                        .expect("Invalid numeric literal argument") as u8));
            }
        }
        // If psuedo instruction
        if opcode > 15 {
            match opcode {
                // INC
                0x10 => {
                    binary.push(
                        // LDI rF 1
                        (0xE << 12) | (0xF << 8) | (0x1)
                    );
                    // Extract register
                    match args[0] {
                        Argument::Register(reg) => binary.push(
                            // ADD r1 rF r1
                            (0x1 << 12) | ((reg as u16) << 8) | (0xF << 4) | (reg as u16)
                        ),
                        _ => panic!("Invalid arg for INC")
                    }
                },
                _ => panic!("Invalid opcode"),
            }
            continue;
        }
        let mut arg_bin: u16 = 0;
        let mut offset = 12;
        for i in args {
            match i {
                Argument::Literal(v) => {
                    offset -= 8;
                    arg_bin |= (v as u16) << offset;
                },
                Argument::Address(v) => {
                    offset -= 12;
                    arg_bin |= (v as u16) << offset;
                },
                Argument::Register(v) => {
                    offset -= 4;
                    arg_bin |= (v as u16) << offset;
                },
            }
        }
        binary.push(
            (opcode & 0x0F) << 12 | arg_bin
        );
    }

    let mut out = File::create(filename_out).expect("Failed to create output file");

    let mut binary_out: [u8; 4096] = [0; 4096];
    let mut index = 0;
    for i in 0..binary.len() {
        let cur = binary[i];
        println!("{:#x}", cur);
        let first = (cur & 0xFF00) >> 8;
        let second = cur & 0x00FF;
        binary_out[index] = first as u8;
        binary_out[index + 1] = second as u8;
        index += 2;
    }
    out.write(&mut binary_out).expect("Failed to write file");
}
