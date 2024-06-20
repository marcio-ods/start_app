use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Arguments {
    pub user: String,
    pub pwd: String,
    pub path: String,
    pub new: bool,
}

pub fn get_args() -> Arguments {
    let mut args = Arguments {
        path: "".to_string(),
        pwd: "".to_string(),
        user: "".to_string(),
        new: true,
    };

    for argument in env::args() {
        // let parts = argument.split("=").collect::<Vec<_>>();
        let parts: Vec<String> = argument.split('=').map(|s| s.trim().to_string()).collect();

        // println!("{argument}");
        if parts[0] == "--user" {
            args.user = parts[1].to_string();
        }

        if parts[0] == "--pwd" {
            args.pwd = parts[1].to_owned();
        }

        if parts[0] == "--path" {
            args.path = parts[1].to_owned();
        }
    }

    if args.path.is_empty() || args.user.is_empty() || args.pwd.is_empty() {
        args.new = false;
    }
    args
}
