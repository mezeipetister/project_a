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

// use crate::prelude::*;
// use crate::user::model::user_v1::UserV1;
// use crate::user::User;

/// # Login function
/// Logically manage login process. Once the user found, and the password
/// is valid, then we return the user id, or an error message.
/// ```rust
/// use core_lib::user::login::login;
/// let login = login("demo@user.com", "demo_password");
/// ```
pub fn login<'a>(email: &'a str, password: &'a str) -> Result<&'a str, String> {
    let _ = email;
    let _ = password;
    Err("Not implemented".to_owned())
}

/// # Logout function
/// Check the user login status, and try to log out. If the user is valid,
/// and logged in, then removes from the logged-in list. The controller
/// should delete the user-token from the browser. If the user tries to
/// access the system using the
pub fn logout(token: &str) -> Result<String, String> {
    let _ = token;
    Err("Not implemented".to_owned())
}

/// # Validate access token
/// Get a user access token, and validate it. If the token valid,
/// and its in the logged-in list, then return Ok(user-id), if the
/// token is unvalid, or its not in the logged-in list, then return
/// Err("Error message").
pub fn validate_access_token(token: &str) -> Result<String, String> {
    let _ = token;
    Err("Not implemented".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_login() {
        assert_eq!(login("email", "password").is_ok(), false); // Should be false.
    }

    #[test]
    fn test_logout() {
        assert_eq!(logout("token").is_ok(), false);
    }

    #[test]
    fn test_validate_token() {
        assert_eq!(validate_access_token("token").is_ok(), false);
    }
}
