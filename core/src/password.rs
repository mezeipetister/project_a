use bcrypt::{hash, verify};
use rand::Rng;
use std::char::ToUppercase;

pub fn hash_password(password: &String) -> Result<String, String> {
    //let hashed = hash("hunter2", DEFAULT_COST)?;
    //let valid = verify("hunter2", &hashed)?;
    match hash(password, 6) {
        Ok(hash) => Ok(hash),
        Err(_) => Err("Error while creating hash from password".to_owned()),
    }
}

pub fn verify_password_from_hash(password: &String, hash: &String) -> Result<bool, String> {
    match verify(password, &hash) {
        Ok(result) => Ok(result),
        Err(_) => Err("Error while trying verify password from hash".to_owned()),
    }
}

pub fn generate_random_password(length: Option<u32>) -> Result<String, String> {
    let mut rng = rand::thread_rng();
    let mut password = "".to_owned();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789"
        .to_owned()
        .chars()
        .collect();
    // Generate random string, default length is 12
    for _ in 0..length.unwrap_or(12) {
        // Gerenate random character
        let random_ch = match chars.get(rng.gen_range(0, chars.len())) {
            Some(ch) => ch,
            None => return Err("Error while generating random passowrd!".to_owned()),
        };
        // Random uppercase
        let random_ch = match rng.gen_range(0, 1) {
            1 => random_ch.to_uppercase().nth(0).unwrap(),
            _ => random_ch.clone(),
        };
        password.push(random_ch);
    }
    return Ok(password);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash_password() {
        use super::*;
        let password = "loremipsumdolorem".to_owned();
        let hash = hash_password(&password).unwrap();
        assert_ne!(hash.len(), password.len());
    }

    #[test]
    fn test_verify_password() {
        use super::*;
        let password = "loremipsumdolorem".to_owned();
        let hash = hash_password(&password).unwrap();
        assert_eq!(verify_password_from_hash(&password, &hash).unwrap(), true);
        assert_eq!(
            verify_password_from_hash(&"wrong_password".to_owned(), &hash).unwrap(),
            false
        );
    }

    #[test]
    fn test_random_generator() {
        use super::*;
        assert_eq!(generate_random_password(None).unwrap().len(), 12); // This should be true
        assert_eq!(generate_random_password(Some(5)).unwrap().len(), 5); // This should be true
        assert_eq!(generate_random_password(Some(0)).unwrap().len(), 0); // This should be true
        assert_eq!(generate_random_password(Some(7)).unwrap().len(), 7); // This should be true
    }
}
