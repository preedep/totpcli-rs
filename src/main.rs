use std::fs;

use clap::Parser;
use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use log::debug;
use totp_rs::{Algorithm, Secret, TOTP};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional mode to run with generate QR Code with mode = qr
    #[arg(short, long, default_value = "validate")]
    mode: Option<String>,
}

fn main() {
    pretty_env_logger::init();
    debug!("Starting up");
    let args = Cli::parse();
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
                let totp = get_totp(input);

                let qr_code = totp.get_qr_png().expect("Failed to generate QR Code");
                let _rs = fs::remove_file("qr.png");
                fs::write("qr.png", qr_code).unwrap();


                /*
                let conf = Config {
                    // set offset
                    x: 0,
                    y: 0,
                    // set dimensions
                    width: Some(80),
                    height: Some(25),
                    ..Default::default()
                };
                print_from_file("qr.png", &conf).expect("Image printing failed.");

                 */
            }
            "validate" => {
                debug!("Validate mode");
                let key = fs::read_to_string("key.txt").expect("Something went wrong reading the file");
                if key.is_empty() {
                    panic!("Key is empty");
                }
                let totp = get_totp(key);
                let code = totp.generate_current().unwrap();
                debug!("Code: {}", code);
            }
            _ => {
                panic!("Invalid mode");
            }
        }
    }
}

fn get_totp(input: String) -> TOTP {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Raw(input.into_bytes()).to_bytes().unwrap(),
        Some("Preedee".to_string()),
        "preedee.ponchevin@gmail.com".to_string(),
    ).unwrap();
    totp
}
