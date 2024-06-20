use crate::argument::Arguments;
use std::process::Command;
use std::str;

pub fn powershell(args: Arguments) {
    let pwd = str::replace(&args.pwd, '"', "'");
    let hostname = str::replace(&args.user, '"', "'");
    let filepath = str::replace(&args.path, '"', "'");

    let str = format!("$pass=ConvertTo-SecureString {} -AsPlainText -Force; $credential=New-Object -TypeName PSCredential -ArgumentList @({},$pass); Start-Process -FilePath {} -Credential $credential -PassThru; Exit-PSHostProcess", pwd,hostname,filepath);
    // write_ps1(&str);
    let status = Command::new("powershell")
        .args(["/C", &str])
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("sucesso");
    } else {
        println!("erro");
    }
}
