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

use crate::email;
use crate::prelude::*;
use crate::storage;
use crate::user::password::*;
use crate::user::User;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct UserV1 {
    id: Option<String>,
    path: Option<String>,
    name: Option<String>,
    address: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    password_hash: Option<String>,
}

impl New for UserV1 {
    /// # New user
    /// generating new user with None default values.
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::model::user_v1::UserV1;
    /// use core_lib::user::User;
    /// let user = UserV1::new();
    /// ```
    fn new() -> Self {
        UserV1 {
            id: None,
            path: None,
            name: None,
            address: None,
            email: None,
            phone: None,
            password_hash: None,
        }
    }
}

impl User for UserV1 {
    /// # Get user ID
    /// Some(String) or None
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::model::user_v1::*;
    /// use core_lib::user::User;
    /// let mut user = UserV1::new();
    /// user.set_user_id("example").unwrap();
    /// let user_id = user.get_user_id();
    /// ```
    fn get_user_id(&self) -> Option<String> {
        self.id.clone()
    }
    /// # Set user ID
    /// Result<(), String>
    /// Minimum user ID length is 5 characters
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::model::user_v1::*;
    /// use core_lib::user::User;;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.set_user_id("demo_id"), Ok(()));
    /// ```
    fn set_user_id(&mut self, user_id: &str) -> Result<(), String> {
        if self.id.is_some() {
            Err("UserID already set! It can't be modified!".to_owned())
        } else if user_id.len() <= 5 {
            Err("UserID length should be bigger then 5 characters.".to_owned())
        } else {
            // Here we set ID as all lowecase
            self.id = Some(user_id.to_lowercase().to_string());
            Ok(())
        }
    }
    /// # Get user name
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::*;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.get_user_name(), None);
    /// user.set_user_name("demo_user").unwrap();
    /// assert_eq!(user.get_user_name(), Some("demo_user".to_owned()));
    /// ```
    fn get_user_name(&self) -> Option<String> {
        self.name.clone()
    }
    /// # Set user name
    /// Result<(), String>
    /// Minimum character length is 5
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::UserV1;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.set_user_name("Demo User"), Ok(()));
    /// ```
    fn set_user_name(&mut self, name: &str) -> Result<(), String> {
        if name.len() < 5 {
            Err("User name must be longer then 5 character".to_owned())
        } else {
            self.name = Some(name.to_string());
            Ok(())
        }
    }
    /// # Get user address
    /// Option<String>
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::UserV1;
    /// let user = UserV1::new();
    /// assert_eq!(user.get_user_address(), None);
    /// ```
    fn get_user_address(&self) -> Option<String> {
        self.address.clone()
    }
    /// # Set user address
    /// Result<(), String>
    /// Minimum character length is 10
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::UserV1;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.set_user_address("Lorem country Shiny city Beautiful street 35."), Ok(()));
    /// ```
    fn set_user_address(&mut self, address: &str) -> Result<(), String> {
        if address.len() > 10 {
            self.address = Some(address.to_owned());
            Ok(())
        } else {
            Err("User address must be longer then 10 characters".to_owned())
        }
    }
    /// # Get user email
    /// Option<String>
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::UserV1;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.get_user_email(), None);
    /// ```
    fn get_user_email(&self) -> Option<String> {
        self.email.clone()
    }
    /// # Set user email
    /// Result<(), String>
    /// Minimum character length is 5 + must contains the following characters:
    /// @(at sign) .(dot)
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::UserV1;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.set_user_email("user@company.com"), Ok(()));
    /// ```
    fn set_user_email(&mut self, email: &str) -> Result<(), String> {
        if email.contains('@') && email.contains('.') && email.len() > 5 {
            self.email = Some(email.to_owned());
            Ok(())
        } else {
            Err("Wrong email format! Email must contains the followings:
            @ and . . Len must be higher then 5 characters"
                .to_owned())
        }
    }
    /// # Get user phone
    /// Option<String>
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::UserV1;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.get_user_phone(), None);
    /// ```
    fn get_user_phone(&self) -> Option<String> {
        self.phone.clone()
    }
    /// # Set user phone
    /// Result<(), String>
    /// Minimum character length is 5
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::UserV1;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.set_user_phone("+749 (39) 4759 33279"), Ok(()));
    /// ```
    fn set_user_phone(&mut self, phone: &str) -> Result<(), String> {
        if phone.len() > 5 {
            self.phone = Some(phone.to_owned());
            Ok(())
        } else {
            Err(
                "Phone number must be higher then 5 characters. It seems to be wrong format."
                    .to_owned(),
            )
        }
    }
    /// # Get user password as hash
    /// Option<String>
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::UserV1;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.get_password_hash(), None);
    /// ```
    fn get_password_hash(&self) -> Option<String> {
        self.password_hash.clone()
    }
    /// # Set user password
    /// Result<(), String>
    /// Password must have a valid format (minimum 7 characters long,
    /// minimum 1 number, minimum 2 characters with lowercase, minimum 2 characters uppercase)
    /// ```rust
    /// use core_lib::prelude::New;
    /// use core_lib::user::User;
    /// use core_lib::user::model::user_v1::UserV1;
    /// let mut user = UserV1::new();
    /// assert_eq!(user.set_password("PAssword1234789"), Ok(()));
    /// ```
    fn set_password(&mut self, password: &str) -> Result<(), String> {
        let hash = match validate_password(password) {
            Ok(_) => match hash_password(password) {
                Ok(hash) => hash,
                Err(msg) => return Err(msg),
            },
            Err(msg) => return Err(msg),
        };
        self.password_hash = Some(hash);
        Ok(())
    }

    // TODO: Maybe should be at a higher level using User trait reference as input?
    // Maybe this?
    // => fn reset_password<T: User>(user: &T) -> Result<(), String> {...}
    fn reset_password(&mut self) -> Result<(), String> {
        let new_password = match generate_random_password(None) {
            Ok(password) => password,
            Err(msg) => return Err(msg),
        };
        self.password_hash = Some(new_password.clone());
        match email::send_new_email(
            env::var("E_CLIENT").unwrap().as_ref(),
            env::var("E_USERNAME").unwrap().as_ref(),
            env::var("E_PASSWORD").unwrap().as_ref(),
            &self.email.as_ref().unwrap().as_ref(),
            &self.name.as_ref().unwrap().as_ref(),
            env::var("E_FROM").unwrap().as_ref(),
            "New password",
            format!(
                "Hi {}! Your new password: {}",
                self.name.as_ref().unwrap(),
                new_password
            )
            .as_ref(),
        ) {
            Ok(_) => (),
            // TODO:
            // Use email pool, in case of email service failure.
            // Instead of using error in case of error - directly here -,
            // We should say its Ok(()) now, and in case of error, the email pool,
            // should manage the trials.
            Err(msg) => {
                return Err(format!(
                    "New password generated and set, but email send faild. Error message: {}",
                    msg
                )
                .to_owned())
            }
        }
        Ok(())
    }
}

/**
 * StorageObject implementation for UserV1
 */
impl storage::StorageObject for UserV1 {
    fn get_id(&self) -> Option<&str> {
        match &self.id {
            Some(id) => Some(id.as_ref()),
            None => None,
        }
    }
    fn save(&self) -> Result<(), String> {
        storage::save_storage_object(self)
    }
    // TODO: Fix this one!
    fn reload(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn get_path(&self) -> Option<&str> {
        match &self.path {
            Some(path) => Some(path.as_ref()),
            None => None,
        }
    }
    fn set_path(&mut self, path: &str) -> Result<(), String> {
        self.path = Some(path.to_owned());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id() {
        let mut user: UserV1 = UserV1::new();
        // At this point ID should be None;
        assert_eq!(user.get_user_id(), None);
        user.set_user_id("Demo_user").unwrap();
        // This should return an Err(..)
        // Let's test is
        assert_eq!(user.set_user_id("demo_user_new_set_new_id").is_err(), true);
        // Now the user should have Some("demo_user" as String) as ID.
        // Test that it's not overwritten, and all letter is lovercase
        assert_eq!(user.get_user_id(), Some("demo_user".to_owned()));
    }

    #[test]
    fn test_user_email() {
        let mut user: UserV1 = UserV1::new();

        // Check assertions
        assert_eq!(user.set_user_id("demo_user").is_ok(), true); // should be ok
        assert_eq!(user.set_user_email("demo@demo.com").is_ok(), true); // should be ok
        assert_eq!(user.set_user_email("wohoo").is_err(), true); // should be err
        assert_eq!(user.set_user_email("demo@company.com").is_ok(), true); // should be ok

        // Check email wether email is correct
        assert_eq!(user.get_user_email(), Some("demo@company.com".to_owned()));
    }

    #[test]
    fn test_user_name() {
        let mut user: UserV1 = UserV1::new();
        assert_eq!(user.get_user_name(), None);
        assert_eq!(user.set_user_name("abc").is_err(), true); // should be err
        assert_eq!(user.set_user_name("Demo User").is_ok(), true); // should be ok
        assert_eq!(user.set_user_name("Hello World").is_ok(), true); // should be ok
        assert_eq!(user.get_user_name(), Some("Hello World".to_owned())) // should be ok
    }

    #[test]
    fn test_user_address() {
        let mut user: UserV1 = UserV1::new();
        let address: &str = "747999 Demo Country, Great county, Hello World street 79.";
        assert_eq!(user.get_user_address(), None);
        assert_eq!(user.set_user_address(address).is_ok(), true); // should be ok
        assert_eq!(user.set_user_address("addr").is_err(), true); // should be err
        assert_eq!(user.get_user_address(), Some(address.to_owned()))
    }

    #[test]
    fn test_user_phone() {
        let mut user: UserV1 = UserV1::new();
        let phone_number: &str = "+99 (701) 479 397129";
        assert_eq!(user.get_user_phone(), None);
        assert_eq!(user.set_user_phone(phone_number).is_ok(), true); // should be ok
        assert_eq!(user.set_user_phone("phn").is_err(), true); // should be err
        assert_eq!(user.get_user_phone(), Some(phone_number.to_owned()));
    }

    #[test]
    fn test_user_set_password() {
        let mut user: UserV1 = UserV1::new();
        let password: &str = "HelloWorld749";
        assert_eq!(user.get_password_hash(), None); // should be None
        assert_eq!(user.set_password("pass").is_err(), true); // should be err
        assert_eq!(user.set_password("PAss7").is_err(), true); // should be err
        assert_eq!(user.set_password("password").is_err(), true); // should be err
        assert_eq!(user.set_password("Password").is_err(), true); // should be err
        assert_eq!(user.set_password("PAssword").is_err(), true); // should be err
        assert_eq!(user.set_password("PAssword7").is_ok(), true); // should be ok
        assert_eq!(user.set_password(password).is_ok(), true); // should be ok
        assert_eq!(
            verify_password_from_hash(password, user.get_password_hash().unwrap().as_ref())
                .unwrap(),
            true
        );
    }
    #[test]
    #[ignore]
    fn test_reset_password() {
        let mut user = UserV1::new();
        user.set_user_email(&env::var("E_TO_TEST_EMAIL").unwrap())
            .unwrap();
        user.set_user_name(&env::var("E_TO_TEST_NAME").unwrap())
            .unwrap();
        assert_eq!(user.reset_password().is_ok(), true);
    }
}
