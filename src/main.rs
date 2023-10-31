use clap::Parser;
use std::fs;
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
    #[clap(flatten)]
    seed: Seed,

    /// role
    #[arg(short, long, default_value = "root")]
    role: String,

    /// hostname
    //#[arg(long)]
    hostname: String,
}

#[derive(Parser)]
#[group(required = true, multiple = false)]
struct Seed {
    /// seed key as arugment
    #[arg(short, long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
    seed: Option<String>,

    /// seed key as file
    #[arg(long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
    seed_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let seed_raw: String;

    if args.seed.seed.is_some() {
        seed_raw = args.seed.seed.unwrap();
    } else {
        seed_raw = fs::read_to_string(args.seed.seed_file.unwrap()).expect("Unable to read seedfile");
    }

    // Normalize inputs
    let seed = seed_raw.trim_end().as_bytes();
    let hostname_raw = args.hostname.to_lowercase();
    let hostname = hostname_raw.trim_end().as_bytes();
    let role_raw = args.role.to_lowercase();
    let role = role_raw.trim_end().as_bytes();

    let context_data = &[hostname, b"/", role];

    println!("{}", derive_pwd(seed, context_data));
}

fn derive_pwd(seed: &[u8], context: &[&[u8]; 3]) -> String {

    // we use a static salt
    // doesn't help much, but eh, why not
    let salt = Salt::new(HKDF_SHA256, b"4pKYYXOtZZa56cJkOu8tlwVd7NrH5rz6");

    let pseudo_rand_key: Prk = salt.extract(seed);
    let output_key_material: Okm<Algorithm> =
        pseudo_rand_key.expand(context, HKDF_SHA256).expect("Failed to expand key material");

    let mut result = [0u8; SHA256_OUTPUT_LEN];
    output_key_material.fill(&mut result).expect("Failed to generate key");

    encode_pwd(&result)
}

fn encode_pwd(pwd_raw: &[u8]) -> String {
    // specify encoding set
    let hex = {
        let mut spec = Specification::new();
        // set of characters
        // avoids special characters
        // avoids characters that look similar, like 0 and O or 1 and l
        spec.symbols.push_str("23456789abcdefghijklmnprstuvwxyz");
        spec.encoding().expect("Failed to encode generated password")
    };

    hex.encode(pwd_raw).split_at(16).0.to_string()
}
