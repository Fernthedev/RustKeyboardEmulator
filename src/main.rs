use std::env;
use std::fs;
use std::io;
use std::path;
use std::io::{BufReader, BufRead};
extern crate keybd_event;


use keybd_event::KeyboardKey::{KeyA, KeyZ, Key0, KeySP1, KeyKPComma};
use keybd_event::{KeyBondingInstance, KeyboardKey};
use std::time::Duration;
use std::borrow::Borrow;
use std::thread::sleep;
use std::ptr::null;
use std::fmt::Error;
use std::ffi::NulError;
use std::num::ParseIntError;
use std::collections::LinkedList;
// use winapi::um::winuser::{VK_SHIFT, VK_RETURN};
use std::mem::size_of;
use std::os::raw::c_int;

#[link(name = "user32")]
extern "C" {
    fn SendInput(cInputs: u32, pInputs: &Vec<INPUT>, int: i32) -> u32;
}

struct KeyboardInput {
    wVk: u16,
    wScan: u16,
    dwFlags: u32,
    time: u32,
    dwExtraInfo: u64
}

struct INPUT {
    typeD: u32,
    ki: KeyboardInput,
}

const INPUT_KEYBOARD: u32 = 1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut filename = String::new();
    let mut time_between_lines: u64 = 5;

    if args.len() > 1 {
        filename = String::from(&args[1]);
    } else {
        println!("Specify the file to read. Absolute or relative (./)");

        match io::stdin().read_line(&mut filename) {
            Ok(txt) => {

            }
            Err(e) => {
               panic!(e);
            }
        }
    }

    if args.len() > 2 {
        match String::from(&args[2]).parse::<u64>() {
            Ok(i) => { time_between_lines = i }
            Err(e) => {
                panic!(e)
            }
        }
    } else {
        println!("Specify the time between each line enter (millis). Default is {} (leave empty for default)", time_between_lines);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(txt) => {
                input = input.trim().replace("\n", "");
                if !input.is_empty() {
                    match input.parse::<u64>() {
                        Ok(i) => { time_between_lines = i }
                        Err(e) => {
                            panic!(e)
                        }
                    }
                }
            }
            Err(e) => {
                panic!(e);
            }
        }
    }

    filename = filename.trim().replace("\n", "");

    if !std::path::PathBuf::from(&filename).as_path().exists() {
        panic!(format!("\nFile {} could not be found in folder {}", filename, std::path::PathBuf::from(".").canonicalize().unwrap().to_str().unwrap()));
    }

    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);


    let mut kb = KeyBondingInstance::new().unwrap();

    println!("Sleeping 4 secs");
    sleep(Duration::from_secs(4));


    kb.has_shift(false);
    for line in reader.lines() {
        if !line.is_ok() {
            continue;
        }

        if (time_between_lines > 0) {
            kb.clear();
        }

        let l_str = line.unwrap();

        println!("Doing line {}", l_str);
        for chara in l_str.chars() {
            kb.has_shift(chara.is_uppercase());
            match get_key_from_char(chara.to_ascii_lowercase()) {
                Ok(c) => { kb.add_key(c); }
                Err(e) => {
                    println!("Key error: \"{}\"", e)
                }
            }
        }

        kb.add_key(KeyboardKey::KeyENTER);

        if time_between_lines > 0 {
            sleep(Duration::from_millis(time_between_lines));
            kb.launching();
        }
    }

    if time_between_lines == 0 {
        kb.launching();
    }


    // let contents = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");
}

fn get_key_from_char(chara: char) -> Result<KeyboardKey, String> {
    return match chara {
        '1' => Ok(KeyboardKey::Key1),
        '2' => Ok(KeyboardKey::Key2),
        '3' => Ok(KeyboardKey::Key3),
        '4' => Ok(KeyboardKey::Key4),
        '5' => Ok(KeyboardKey::Key5),
        '6' => Ok(KeyboardKey::Key6),
        '7' => Ok(KeyboardKey::Key7),
        '8' => Ok(KeyboardKey::Key8),
        '9' => Ok(KeyboardKey::Key9),
        '0' => Ok(KeyboardKey::Key0),
        'q' => Ok(KeyboardKey::KeyQ),
        'w' => Ok(KeyboardKey::KeyW),
        'e' => Ok(KeyboardKey::KeyE),
        'r' => Ok(KeyboardKey::KeyR),
        't' => Ok(KeyboardKey::KeyT),
        'y' => Ok(KeyboardKey::KeyY),
        'u' => Ok(KeyboardKey::KeyU),
        'i' => Ok(KeyboardKey::KeyI),
        'o' => Ok(KeyboardKey::KeyO),
        'p' => Ok(KeyboardKey::KeyP),
        'a' => Ok(KeyboardKey::KeyA),
        's' => Ok(KeyboardKey::KeyS),
        'd' => Ok(KeyboardKey::KeyD),
        'f' => Ok(KeyboardKey::KeyF),
        'g' => Ok(KeyboardKey::KeyG),
        'h' => Ok(KeyboardKey::KeyH),
        'j' => Ok(KeyboardKey::KeyJ),
        'k' => Ok(KeyboardKey::KeyK),
        'l' => Ok(KeyboardKey::KeyL),
        'z' => Ok(KeyboardKey::KeyZ),
        'x' => Ok(KeyboardKey::KeyX),
        'c' => Ok(KeyboardKey::KeyC),
        'v' => Ok(KeyboardKey::KeyV),
        'b' => Ok(KeyboardKey::KeyB),
        'n' => Ok(KeyboardKey::KeyN),
        'm' => Ok(KeyboardKey::KeyM),
        '\n' => Ok(KeyboardKey::KeyENTER),
        ' ' => Ok(KeyboardKey::KeySPACE),
        '.' => Ok(KeyboardKey::KeyKPDot),
        ',' => Ok(KeyboardKey::KeyKPComma),
        '=' => Ok(KeyboardKey::KeyKPEqual),
        '-' => Ok(KeyboardKey::KeyKPMinus),
        '+' => Ok(KeyboardKey::KeyKPPlus),
        '/' => Ok(KeyboardKey::KeyKPSlash),
        _ => Err(String::from(format!("key not found for {}", chara)))
    }
}

