use clap::{Parser, Subcommand};
use libcrypt::CryptFunction;

struct SerialCrypt {
    key: u8,
}

impl CryptFunction for SerialCrypt {
    fn encrypt(&mut self, data: &mut [u8]) {
        for i in data {
            *i ^= self.key;
            self.key = *i;
        }
    }

    fn decrypt(&mut self, data: &mut [u8]) {
        for i in data {
            (*i , self.key)  = (*i^self.key , *i );
        }
    }

    fn get_signature(&self) -> &[u8] {
        std::slice::from_ref(&self.key)
    }

    fn set_signature(&mut self, sig: &[u8]) {
        self.key = sig[0];
    }
}

#[derive(Parser)]
#[command(name = "serial encryptor / decryptor")]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt { file: String },
    Decrypt { file: String },
}

fn main() {
    let arg = Args::parse();
    let mut serial_enc = SerialCrypt { key: 34u8 };
    match arg.command {
        Commands::Encrypt { file } => {
            libcrypt::Cryptic::new(&file).encrypt(serial_enc);
        }
        Commands::Decrypt { file } => {
            libcrypt::Cryptic::new(&file).decrypt(&mut serial_enc);
        }
    }
}
