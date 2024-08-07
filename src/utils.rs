use half::f16;
use lazy_static::lazy_static;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::{env, fs, str};

pub const END_PRG: u32 = 0xbfb00000;
lazy_static::lazy_static! {
    pub static ref CI: bool = env::var("CI").map(|v| v == "1").unwrap_or(false);
    pub static ref PROFILE: bool = env::var("PROFILE").map(|v| v == "1").unwrap_or(false);
    pub static ref OSX: bool = env::var("OSX").map(|v| v == "1").unwrap_or(false);
    pub static ref GLOBAL_DEBUG: bool = env::var("DEBUG").map(|v| v == "1").unwrap_or(false);
}

pub fn nth(val: u32, pos: usize) -> u32 {
    return (val >> (31 - pos as u32)) & 1;
}
pub fn f16_lo(val: u32) -> f16 {
    f16::from_bits((val & 0xffff) as u16)
}
pub fn f16_hi(val: u32) -> f16 {
    f16::from_bits(((val >> 16) & 0xffff) as u16)
}

pub fn read_asm(lib: &Vec<u8>) -> (Vec<u32>, String) {
    if std::env::consts::OS == "macos" {
        return _read_literal_asm(lib);
    }
    let mut child = Command::new("/opt/rocm/llvm/bin/llvm-objdump")
        .args(&["-d", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(lib).unwrap()
    }
    let output = child.wait_with_output().unwrap();
    let asm = String::from_utf8_lossy(&output.stdout);
    parse_rdna3(&asm.to_string())
}
fn parse_rdna3(content: &str) -> (Vec<u32>, String) {
    let mut kernel = content.lines().skip(5);
    let name = kernel
        .nth(0)
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .replace(":", "")
        .replace("<", "")
        .replace(">", "");
    let instructions = kernel
        .map(|line| {
            line.split_whitespace()
                .filter(|p| u32::from_str_radix(p, 16).is_ok() && p.len() == 8)
                .collect::<Vec<&str>>()
        })
        .flatten()
        .map(|x| u32::from_str_radix(x, 16).unwrap())
        .collect::<Vec<u32>>();

    return (instructions, name.to_string());
}

pub fn sign_ext(num: u64, bits: usize) -> i64 {
    let mut value = num;
    let is_negative = (value >> (bits - 1)) & 1 != 0;
    if is_negative {
        value |= !0 << bits;
    }
    value as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_rdna3() {
        let instructions = parse_rdna3(
            "
<stdin>:	file format elf64-amdgpu

Disassembly of section .text:

0000000000001600 <E_4>:
	s_load_b64 s[0:1], s[0:1], null                            // 000000001600: F4040000 F8000000
	v_dual_mov_b32 v0, 0 :: v_dual_mov_b32 v1, 4               // 000000001608: CA100080 00000084
",
        )
        .0;
        assert_eq!(instructions.len(), 4);
        let hexed = instructions
            .iter()
            .map(|i| format!("0x{:08x}", i))
            .collect::<Vec<String>>();
        assert_eq!(
            hexed,
            ["0xf4040000", "0xf8000000", "0xca100080", "0x00000084",]
        );
    }

    #[test]
    fn test_custom_signed_bits() {
        assert_eq!(sign_ext(0b000000000000000101000, 21), 40);
        assert_eq!(sign_ext(0b111111111111111011000, 21), -40);
        assert_eq!(sign_ext(0b000000000000000000000, 21), 0);
        assert_eq!(sign_ext(0b111111111111111111111, 21), -1);
        assert_eq!(sign_ext(0b111000000000000000000, 21), -262144);
        assert_eq!(sign_ext(0b000111111111111111111, 21), 262143);

        assert_eq!(sign_ext(7608, 13), -584);
    }
}

pub trait Colorize {
    fn color(self, color: &str) -> String;
}
impl<'a> Colorize for &'a str {
    fn color(self, color: &str) -> String {
        let ansi_code = match color {
            "blue" => format!("\x1b[{};2;112;184;255m", 38),
            "green" => format!("\x1b[{};2;39;176;139m", 38),
            "gray" => format!("\x1b[{};2;169;169;169m", 38),
            _ => format!("\x1b[{};2;255;255;255m", 38),
        };
        format!("{}{}{}", ansi_code, self, "\x1b[0m")
    }
}

#[macro_export]
macro_rules! todo_instr {
    ($x:expr) => {{
        let instr = format!("{:08X}", $x);
        use std::env;
        use std::io::Write;
        use std::process::{Command, Stdio};
        let _ = Command::new("pbcopy")
            .stdin(Stdio::piped())
            .spawn()
            .and_then(|mut process| process.stdin.as_mut().unwrap().write_all(instr.as_bytes()));
        if env::var("OSX").map(|v| v == "1").unwrap_or(false) {
            panic!("{:08X}", $x)
        }
        Err(1)
    }};
}

fn _read_literal_asm(lib: &Vec<u8>) -> (Vec<u32>, String) {
    let asm = String::from_utf8(lib.to_vec()).unwrap();
    let mut prg = parse_rdna3(&asm);
    let isolate = env::var("REMU_ISOLATE")
        .unwrap_or_default()
        .parse::<i32>()
        .unwrap_or(0);

    let fp = format!("/tmp/{}.s", prg.1);
    if isolate == 1 {
        prg = match std::fs::metadata(&fp) {
            Ok(_) => parse_rdna3(&fs::read_to_string(fp).unwrap()),
            Err(_) => {
                fs::write(fp, asm).unwrap();
                prg
            }
        };
    } else {
        fs::write(fp, asm).unwrap();
    }
    return prg;
}

#[derive(Debug)]
pub struct GlobalCounter {
    pub vgpr_used: usize,
    pub gds_ops: usize,
    pub lds_ops: usize,
    pub wmma: usize,
    pub wave_syncs: usize,
}
lazy_static! {
    pub static ref GLOBAL_COUNTER: Mutex<GlobalCounter> = Mutex::new(GlobalCounter {
        vgpr_used: 0,
        gds_ops: 0,
        lds_ops: 0,
        wmma: 0,
        wave_syncs: 0
    });
}
