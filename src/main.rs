// use futures::executor::block_on;
use serde_json::Value;
use simple_crypt::{decrypt, encrypt};
use start_app::{
    argument::{get_args, Arguments},
    cfg::{read_cfg_vec8, write_cfg_vec8, write_log, DEFAULT_LOG},
    powershell::powershell,
};
use std::str;
const KEY: &[u8] = "86403d0db97fa93a366fba5a1c0734838baddec27d9a02c55d71a9c35b65b585".as_bytes();

fn main() {
    let args = get_args();
    // https://docs.rs/orion/latest/orion/aead/index.html#
    if args.new {
        let args_json = serde_json::to_string(&args).expect("Failed to encrypt");
        let encrypted_data = encrypt(args_json.as_bytes(), KEY).expect("Failed to encrypt");
        write_cfg_vec8(&encrypted_data);
    }

    let cfg_vec = read_cfg_vec8();

    let decrypt_vec = match decrypt(&cfg_vec, KEY) {
        Ok(res) => res,
        Err(why) => {
            let mut res = "Error: ".to_string();
            res.push_str(&why.to_string());
            res.push_str(DEFAULT_LOG);
            write_log(&res);
            panic!("{}", why)
        }
    };
    let cfg_str = str::from_utf8(&decrypt_vec).unwrap();
    let cfg_data: Value = serde_json::from_str(cfg_str).unwrap();

    // block_on(
    powershell(Arguments {
        new: false,
        path: str::replace(&cfg_data["path"].to_string(), "\\\\", "\\"),
        pwd: cfg_data["pwd"].to_string(),
        user: str::replace(&cfg_data["user"].to_string(), "\\\\", "\\"),
    })
    // );
}
