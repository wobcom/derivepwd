use clap::Parser;
use data_encoding::Specification;
use ring::digest::SHA256_OUTPUT_LEN;
use ring::hkdf::Algorithm;
use ring::hkdf::Okm;
use ring::hkdf::Prk;
use ring::hkdf::Salt;
use ring::hkdf::HKDF_SHA256;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// seed key
    #[arg(short, long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
    seed: String,

    /// role
    #[arg(short, long, default_value = "root")]
    role: String,

    /// hostname
    //#[arg(long)]
    hostname: String,
}

fn main() {
    // specify encoding set
    let hex = {
        let mut spec = Specification::new();
        // set of characters
        // avoids special characters
        // avoids characters that look similar, like 0 and O or 1 and l
        spec.symbols.push_str("23456789abcdefghijklmnprstuvwxyz");
        spec.encoding().unwrap()
    };

    let args = Args::parse();

    let input = args.seed.as_bytes();
    // we use a fixed salt
    // doesn't help much, but eh, why not
    let salt = Salt::new(HKDF_SHA256, b"4pKYYXOtZZa56cJkOu8tlwVd7NrH5rz6");
    // [TODO]: Normalize inputs
    let context_data = &[args.hostname.as_bytes(), args.role.as_bytes()];

    let pseudo_rand_key: Prk = salt.extract(input);
    let output_key_material: Okm<Algorithm> =
        pseudo_rand_key.expand(context_data, HKDF_SHA256).unwrap();

    let mut result = [0u8; SHA256_OUTPUT_LEN];
    output_key_material.fill(&mut result).unwrap();

    println!("{}", hex.encode(&result).split_at(16).0);
}
