use argon2::Argon2;
use password_hash::{rand_core::OsRng, SaltString, PasswordHasher};

fn main() {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let hash = argon2.hash_password("testpassword".as_bytes(), &salt).expect("unable to hash").to_string();

    println!("Hash: {}", hash);
}
