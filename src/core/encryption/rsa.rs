use base64::engine::general_purpose;
use base64::Engine;
use rand::rngs::OsRng;
use rsa::pkcs1::{
	DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey,
};
use rsa::pkcs8::der::zeroize::Zeroizing;
use rsa::pkcs8::LineEnding;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::fs::{read_to_string, File};
use std::io::Write;

pub fn decrypt(text: &str, client_key: &str) -> String {
	let file: String =
		read_to_string(&format!("clients/{}/private-key.pem", client_key)).unwrap_or_default();
	let private_key: RsaPrivateKey =
		RsaPrivateKey::from_pkcs1_pem(&file).expect("Failed to load RSA private key");

	let decoded: Vec<u8> = general_purpose::STANDARD.decode(text).unwrap_or_default();
	let decrypted: Vec<u8> = private_key
		.decrypt(Pkcs1v15Encrypt, &decoded)
		.unwrap_or_default();

	String::from_utf8_lossy(&decrypted).to_string()
}

pub fn encrypt(text: &str, client_key: &str) -> String {
	let file: String =
		read_to_string(&format!("clients/{}/public-key.pem", client_key)).unwrap_or_default();
	let public_key: RsaPublicKey =
		RsaPublicKey::from_pkcs1_pem(&file).expect("Failed to load RSA public key");

	let encrypted: Vec<u8> = public_key
		.encrypt(&mut OsRng, Pkcs1v15Encrypt, text.as_bytes())
		.unwrap_or_default();

	general_purpose::STANDARD.encode(encrypted)
}

pub fn generate_key(size: &usize) {
	let private_key: RsaPrivateKey =
		RsaPrivateKey::new(&mut OsRng, *size).expect("Failed to generate private key");
	let pem: Zeroizing<String> = private_key.to_pkcs1_pem(LineEnding::LF).unwrap_or_default();

	let mut file: File =
		File::create("../../../clients/EBFF467D15CAA7DC6B8CAC2586DCF/rsa-private-key.pem").expect("Failed to create RSA private key file");
	file.write_all(pem.as_bytes())
		.expect("Failed to write RSA private key file");

	let public_key: RsaPublicKey = RsaPublicKey::from(&private_key);
	let string: String = public_key
		.to_pkcs1_pem(LineEnding::LF)
		.unwrap_or_default()
		.to_string();

	file =
		File::create("../../../clients/EBFF467D15CAA7DC6B8CAC2586DCF/rsa-public-key.pem").expect("Failed to create RSA public key file");
	file.write_all(string.as_bytes())
		.expect("Failed to write RSA public key file");
}
