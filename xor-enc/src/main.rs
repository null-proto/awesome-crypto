use clap::{Parser, Subcommand};
use libcrypt::CryptFunction;

struct XorCrypt {
    key : u8
}

impl CryptFunction for XorCrypt {
    fn encrypt(&mut self,data:&mut [u8]) {
        for i in data {
            *i ^= self.key;
        }
    }

    fn decrypt(&mut self,data:&mut [u8]) {
        for i in data {
            *i ^= self.key;
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
#[command(name="xor encryptor / decryptor")]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    file: Option<String>
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {file : String},
    Decrypt {file : String}
}

fn main() {
    let arg = Args::parse();
    let mut  xor_enc = XorCrypt { key: 34u8 };
    match arg.command {
        Commands::Encrypt { file } => {
            libcrypt::Cryptic::new(&file)
                .encrypt(xor_enc);
        },
        Commands::Decrypt { file } => {
            libcrypt::Cryptic::new(&file)
                .decrypt(&mut xor_enc);
        },
    }
}
