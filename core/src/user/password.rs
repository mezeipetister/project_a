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
/// Get a password string pointer, returns a Result<String, String>
/// ```rust
/// use core_lib::user::password::hash_password;
/// let hash = hash_password("purple dog").unwrap();
/// ```
pub fn hash_password(password: &str) -> Result<String, String> {
    //let hashed = hash("hunter2", DEFAULT_COST)?;
    //let valid = verify("hunter2", &hashed)?;
    match hash(password, 6) {
        Ok(hash) => Ok(hash),
        Err(_) => Err("Error while creating hash from password".to_owned()),
    }
}

/// # Verify password from hash
/// Gets a password and hash pointer and returns a Result<bool, String>
/// True if verify succeed, false otherwise.
/// ```rust
/// use core_lib::user::password::{verify_password_from_hash, hash_password};
/// let hash = hash_password("purple_dog").unwrap();
/// let result: bool = verify_password_from_hash(
///                         "purple_dog",
///                         &hash).unwrap();
/// ```
pub fn verify_password_from_hash<'a>(password: &'a str, hash: &'a str) -> Result<bool, String> {
    match verify(password, &hash) {
        Ok(result) => Ok(result),
        Err(_) => Err("Error while trying verify password from hash".to_owned()),
    }
}

/// # Generate random password
/// Set a length or leave it None.
/// Returns a random password aA-zZ, 0-9
/// ```rust
/// use core_lib::user::password::generate_random_password;
/// let password = generate_random_password(None).unwrap();
/// ```
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
        // TODO: uppercase does not work!
        let random_ch: char = match rng.gen_range(0, 1) {
            1 => *random_ch,
            _ => *random_ch,
        };
        password.push(random_ch);
    }
    Ok(password)
}

/// # Validate password
/// Validate password to check it is strong enough.
/// What we check is *password length*, *uppercase character frequency*,
/// *lowercase character frequency* and *number frequency*.
/// ```rust
/// use core_lib::user::password::validate_password;
/// assert_eq!(validate_password("DEmoPassWord1234789").is_ok(), true);
/// ```
pub fn validate_password(password: &str) -> Result<(), String> {
    let min_password_len = 7;
    let min_character_lowercase = 2;
    let min_character_uppercase = 2;
    let min_numeric_character = 1;
    let mut character_lowercase: u32 = 0;
    let mut character_uppercase: u32 = 0;
    let mut character_numeric: u32 = 0;
    for ch in password.chars() {
        // count numeric characters
        if ch.is_numeric() {
            character_numeric += 1;
        }
        // count lowercase characters
        if ch.is_lowercase() {
            character_lowercase += 1;
        }
        // count uppercase characters
        if ch.is_uppercase() {
            character_uppercase += 1;
        }
    }
    if password.len() >= min_password_len
        && character_numeric >= min_numeric_character
        && character_lowercase >= min_character_lowercase
        && character_uppercase >= min_character_uppercase
    {
        Ok(())
    } else {
        Err(format!(
            "Password should be min {} length, should contain min {}
            lowercase letter, min {} uppercase letter, min {} number",
            min_password_len,
            min_character_lowercase,
            min_character_uppercase,
            min_numeric_character
        ))
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_password() {
        let password = "purple_dog";
        let hash = hash_password(password).unwrap();
        assert_ne!(hash.len(), password.len());
    }

    #[test]
    fn test_verify_password() {
        let password = "purple_dog";
        let hash = hash_password(password).unwrap();
        assert_eq!(verify_password_from_hash(password, &hash), Ok(true));
        assert_eq!(
            verify_password_from_hash("wrong_password", &hash),
            Ok(false)
        );
    }

    #[test]
    fn test_random_generator() {
        assert_eq!(generate_random_password(None).unwrap().len(), 12); // This should be true
        assert_eq!(generate_random_password(Some(5)).unwrap().len(), 5); // This should be true
        assert_eq!(generate_random_password(Some(0)).unwrap().len(), 0); // This should be true
        assert_eq!(generate_random_password(Some(7)).unwrap().len(), 7); // This should be true
    }
    #[test]
    fn test_validate_password() {
        assert_eq!(validate_password("pass").is_err(), true); // should be err
        assert_eq!(validate_password("PAss1").is_err(), true); // should be err
        assert_eq!(validate_password("password").is_err(), true); // should be err
        assert_eq!(validate_password("Password").is_err(), true); // should be err
        assert_eq!(validate_password("PASsword").is_err(), true); // should be err
        assert_eq!(validate_password("Password12").is_err(), true); // should be err
        assert_eq!(validate_password("PAssword12").is_ok(), true); // should be ok
    }
}
