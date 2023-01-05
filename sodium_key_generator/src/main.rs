use anyhow::Result;
use sodiumoxide::base64::decode;
use sodiumoxide::base64::encode;
use sodiumoxide::crypto::sign;
use sodiumoxide::init;
use thiserror::Error;

pub struct EdKeyPair {
    priv_key: String,
    pub_key: String,
}

pub fn generate_key() -> Result<EdKeyPair, Error> {
    init().map_err(|_e| Error::InitializationFailedError)?;
    let (ourpk, oursk) = sodiumoxide::crypto::sign::ed25519::gen_keypair();
    let key_pair = EdKeyPair {
        priv_key: String::from(encode(oursk, sodiumoxide::base64::Variant::UrlSafe)),
        pub_key: String::from(encode(ourpk, sodiumoxide::base64::Variant::UrlSafe)),
    };
    Ok(key_pair)
}

pub fn sign_message(message: String, key: String) -> Result<String, Error> {
    init().map_err(|_e| Error::InitializationFailedError)?;
    let sk =
        decode(key, sodiumoxide::base64::Variant::UrlSafe).map_err(|_e| Error::KeyFailedError)?;
    let real_sk = match sodiumoxide::crypto::sign::ed25519::SecretKey::from_slice(&sk) {
        Some(sk) => Ok(sk),
        None => Err(Error::KeyFailedError),
    }?;

    let signature = sign::sign_detached(message.as_bytes(), &real_sk);
    Ok(signature.to_string())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Sodium library initialization failed")]
    InitializationFailedError,
    #[error("Key decoding failed")]
    KeyFailedError,
}

fn main() -> Result<(), Error> {
    let keys = generate_key()?;
    println!(
        "{:?} privatekey, {:?} publickey",
        keys.priv_key, keys.pub_key
    );
    let signature = sign_message(
        "kotki".to_owned(),
        "-teOr2Sp6c5cWYYK1MXkgYgptVhMlBW3uug2u1YStEf4W35dlq18Gb5RYLTCs_RJyfibytqKDFccLQYv5KzksA=="
            .to_owned(),
    );
    println!("{:?} signature", signature);
    Ok(())
}
