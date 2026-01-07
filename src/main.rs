use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hmac::Hmac; // <--- NEU: Import für den HMAC Wrapper
use pbkdf2::pbkdf2;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use std::io::{self, Write};
use zeroize::Zeroize;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
const ITERATIONS: u32 = 600_000;

fn main() {
    println!("========================================");
    println!("   CORE SEC - VAULT V1.0 (PROTOTYPE)    ");
    println!("========================================");

    loop {
        println!("\nBefehl wählen:");
        println!("1. Nachricht verschlüsseln (Lock)");
        println!("2. Nachricht entschlüsseln (Unlock)");
        println!("3. Exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => encrypt_flow(),
            "2" => decrypt_flow(),
            "3" => break,
            _ => println!("Ungültige Eingabe."),
        }
    }
}

fn encrypt_flow() {
    let password = get_input("Passwort für den Tresor:");
    let data = get_input("Geheimnis (Text):");

    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);

    let mut key = [0u8; KEY_LEN];
    // KORREKTUR: Wir nutzen jetzt Hmac<Sha256> statt nur Sha256
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), &salt, ITERATIONS, &mut key);

    let cipher = Aes256Gcm::new(&key.into());
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    let nonce_struct = Nonce::from_slice(&nonce);

    match cipher.encrypt(nonce_struct, data.as_bytes()) {
        Ok(ciphertext) => {
            let result = format!(
                "{}{}{}",
                hex::encode(salt),
                hex::encode(nonce),
                hex::encode(ciphertext)
            );
            println!("\n[SECURED OUTPUT]:\n{}", result);
        }
        Err(_) => println!("Fehler bei der Verschlüsselung!"),
    }

    key.zeroize();
}

fn decrypt_flow() {
    let password = get_input("Passwort:");
    let encrypted_hex = get_input("Verschlüsselter String (Hex):");

    let encrypted_bytes = match hex::decode(encrypted_hex.trim()) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("Fehler: Ungültiges Format.");
            return;
        }
    };

    if encrypted_bytes.len() < SALT_LEN + NONCE_LEN {
        println!("Fehler: Daten zu kurz.");
        return;
    }

    let salt = &encrypted_bytes[..SALT_LEN];
    let nonce = &encrypted_bytes[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &encrypted_bytes[SALT_LEN + NONCE_LEN..];

    let mut key = [0u8; KEY_LEN];
    // KORREKTUR: Auch hier Hmac<Sha256>
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, ITERATIONS, &mut key);

    let cipher = Aes256Gcm::new(&key.into());
    let nonce_struct = Nonce::from_slice(nonce);

    match cipher.decrypt(nonce_struct, ciphertext) {
        Ok(plaintext) => {
            let secret = String::from_utf8_lossy(&plaintext);
            println!("\n[DECRYPTED SECRET]:\n{}", secret);
        }
        Err(_) => println!("\n[ZUGRIFF VERWEIGERT] Falsches Passwort oder manipulierte Daten."),
    }
    
    key.zeroize();
}

fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}