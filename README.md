# TOTP CLI
Generate QR code for TOTP via CLI (Command Line Interface)

## Usage
After build, you can run the binary file with the following command:

```
Usage: totpcli-rs [OPTIONS]

Options:
  -m, --mode <MODE>                  Optional mode to run with generate QR Code with mode = qr [default: validate]
  -i, --issuer <ISSUER>              Optional issuer [default: mytotp]
  -a, --account-name <ACCOUNT_NAME>  Optional account name [default: myaccount]
  -h, --help                         Print help
  -V, --version                      Print version
```

####  mode: Mode to run with generate QR Code with mode = qr
```bash
 ./target/debug/totpcli-rs -m qr 
```
you will get prompts input key (least 16 characters) after than you will get qr code image file name (default: qr.png).

you can use Google Authenticator to scan the QR Code

#### mode: Mode to run with generate QR Code with mode = validate (default)
```bash
 ./target/debug/totpcli-rs
```
you will get current TOTP code



