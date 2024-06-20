use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const TEST_FILE: &str = "test.ps1";
const CFG_FILE: &str = "start.cfg";
const LOG_FILE: &str = "start_app.log";
pub const DEFAULT_LOG: &str = "\nConfigure novamente passando os parâmetros:\n --user=\"nome_pc\\nome_usuário\" --pwd=\"senha_usuário\" --path=\"caminho do executável'\"";
pub const BAT: &str =
    "@echo off\nCD\nstart \" \" start_app.exe --user=\"nome_pc\\nome_usuário\" --pwd=\"senha_usuário\" --path=\"C:\\WINDOWS\\system32\\notepad.exe\"\ndel config_start_app.bat\nexit /B 0";

pub fn write_ps1(str: &str) -> () {
    let mut file = File::create(TEST_FILE).expect("Failed to read line");
    file.write_all(str.as_bytes()).expect("Failed to read line");
}

pub fn write_bat() -> () {
    let mut file = File::create("config_start_app.bat").expect("Failed to read line");
    file.write_all(BAT.as_bytes()).expect("Failed to read line");

    file = File::create("start_app.bat").expect("Failed to read line");
    file.write_all("@echo off\nCD\nstart /C /mim start_app.exe".as_bytes())
        .expect("Failed to read line");
}

pub fn write_cfg_vec8(text: &Vec<u8>) -> () {
    let mut file = File::create(CFG_FILE).expect("Failed to read line");
    file.write_all(text).expect("Failed to read line");
}

pub fn write_log(str: &str) {
    let mut file = File::create(LOG_FILE).expect("Failed to read line");
    file.write_all(&str.as_bytes())
        .expect("Failed to read line");
    write_bat()
}

pub fn read_cfg_vec8() -> Vec<u8> {
    let path = Path::new(CFG_FILE);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            let mut res = display.to_string();
            res.push_str(&": ");
            res.push_str(&why.to_string());
            res.push_str(DEFAULT_LOG);
            write_log(&res);
            panic!("couldn't open {}: {}", display, why)
        }
        Ok(file) => file,
    };

    let mut vec = Vec::new();

    match file.read_to_end(&mut vec) {
        Err(why) => {
            let mut res = display.to_string();
            res.push_str(&": ");
            res.push_str(&why.to_string());
            res.push_str(DEFAULT_LOG);
            write_log(&res);
            panic!("couldn't open {}: {}", display, why)
        }
        Ok(_) => (),
    }
    vec
}
