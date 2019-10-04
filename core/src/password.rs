// Copyright (C) 2019 Peter Mezei
//
// This file is part of Project A.
//
// Project A is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Project A is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Project A.  If not, see <http://www.gnu.org/licenses/>.

use bcrypt::{hash, verify};
use rand::Rng;

/// # Hash password
/// ## Readme
/// Get a password string pointer, returns a result<String, String>
/// ```rust
/// use core_lib::password::hash_password;
/// let hash = hash_password(&"purple dog".to_owned()).unwrap();
/// ```
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
        // Generate random character
        let random_ch = match chars.get(rng.gen_range(0, chars.len())) {
            Some(ch) => ch,
            None => return Err("Error while generating random password!".to_owned()),
        };
        // Random uppercase
        let random_ch: char = match rng.gen_range(0, 1) {
            1 => random_ch.to_uppercase().nth(0).unwrap(),
            _ => random_ch.clone(),
        };
        password.push(random_ch);
    }
    return Ok(password);
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_hash_password() {
        use super::*;
        let password = "purple_dog".to_owned();
        let hash = hash_password(&password).unwrap();
        assert_ne!(hash.len(), password.len());
    }

    #[test]
    fn test_verify_password() {
        use super::*;
        let password = "purple_dog".to_owned();
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
