use crate::rsa::RSA;

mod prime;
mod rsa;
mod utils;

fn main() {
    let msg = "Hello world!".to_string();
    let rsa = RSA::new();

    let decrypted = rsa.decrypt(rsa.encrypt(msg));
    println!("Decrypted {}", decrypted);
}
