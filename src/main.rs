use std::time::Duration;
use std::{fs, thread};

use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use log::debug;
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional mode to run with generate QR Code with mode = qr
    #[arg(short, long, default_value = "validate")]
    mode: Option<String>,
    /// Optional issuer
    #[arg(short, long, default_value = "mytotp")]
    issuer: Option<String>,
    /// Optional account name
    #[arg(short, long, default_value = "myaccount")]
    account_name: Option<String>,
}

fn main() {
    pretty_env_logger::init();
    debug!("Starting up");
    let args = Cli::parse();
    let issuer = args.issuer.unwrap();
    let account_name = args.account_name.unwrap();
    if let Some(mode) = args.mode.as_deref() {
        match mode {
            "qr" => {
                debug!("QR Code mode");
                let input: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("YourKey")
                    .interact_text()
                    .unwrap();
                debug!("YourKey: {}", input);
                fs::write("key.txt", &input).unwrap();
                let totp = get_totp(input, issuer, account_name);

                let qr_code = totp.get_qr_png().expect("Failed to generate QR Code");
                let _rs = fs::remove_file("qr.png");
                fs::write("qr.png", qr_code).unwrap();
            }
            "validate" => {
                debug!("Validate mode");
                let key =
                    fs::read_to_string("key.txt").expect("Something went wrong reading the file");
                if key.is_empty() {
                    panic!("Key is empty");
                }
                let totp = get_totp(key,
                                    issuer,
                                    account_name);

                let ten_seconds = Duration::from_secs(10);
                loop {
                    // your loop code here
                    let code = totp.generate_current().unwrap();
                    debug!("Code: {}", code);
                    thread::sleep(ten_seconds);
                }
            }
            _ => {
                panic!("Invalid mode");
            }
        }
    }
}

fn get_totp(input: String, issuer: String, account_name: String) -> TOTP {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Raw(input.into_bytes()).to_bytes().unwrap(),
        Some(issuer),
        account_name,
    )
    .unwrap();
    totp
}
