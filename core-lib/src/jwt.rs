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

use crypto::sha2::Sha256;
use jsonwebtoken::{Header, Registered, Token};
use std::default::Default;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Claim {
    username: String,
    exp: usize,
}

impl Claim {
    fn new() -> Claim {
        Claim {
            username: "username".to_string(),
            exp: 100000000,
        }
    }
    fn get_username(&self) -> String {
        self.username.clone()
    }
}

fn jwt_create_token(userid: String) -> Result<String, String> {
    let header: Header = Default::default();
    let claim = Registered {
        iss: Some(userid),
        ..Default::default()
    };
    let token = Token::new(header, claim);
    Ok(token.signed(b"secret", Sha256::new()).unwrap())
}

fn jwt_decode_token(token: String) -> String {
    let token = Token::<Header, Registered>::parse(&token).unwrap();
    if (token.verify(b"secret", Sha256::new())) {
        token.claims.iss.unwrap()
    } else {
        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt() {
        let token = jwt_create_token("helloworld".to_owned()).unwrap();
        let result = jwt_decode_token(token);
        assert_eq!(result, "helloworld".to_owned());
        assert_ne!(result, "asdasdad".to_owned());
    }
}
