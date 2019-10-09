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

use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Email;

// TODO: Refactor to split email for production and test use.
// For test use, it should behave like a real email service,
// but just simulate the sending process.
// In production, it should communicate with the email server
// normally.
pub fn send_new_email(
    client: &str,
    username: &str,
    password: &str,
    to: &str,
    to_name: &str,
    from: &str,
    subject: &str,
    body: &str,
) -> Result<(), String> {
    let email: Email = match Email::builder()
        // Addresses can be specified by the tuple (email, alias)
        .to((to, to_name))
        // ... or by an address only
        .from(from)
        .subject(subject)
        .text(body)
        .build()
    {
        Ok(email) => email,
        Err(_) => return Err("Error during creating email".to_owned()),
    };

    let creds = Credentials::new(username.to_string(), password.to_string());

    // Open a remote connection to gmail
    let mut mailer = SmtpClient::new_simple(client)
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email.into());

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Error while sending email.".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_send_email() {
        use super::*;
        // Ok, this is an idiotic test
        // With dummy data of course this should fail.
        assert_eq!(
            send_new_email(
                "smtp.gmail.com",
                "*@gmail.com",
                "*",
                "*@gmail.com",
                "*",
                "*@gmail.com",
                "demo",
                "This is a demo email."
            )
            .is_ok(),
            false
        );
    }
}
