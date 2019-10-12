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

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/**
 * Storage DESIGN
 *
 * Functions:
 *  - pub fn load_storage(path: &'str) -> Result<Vec<T>, String>
 *  - pub fn add_to_storage(storage: &Storage, object: StorageObject) -> Result<Ok(&StorageObject), String>
 *  -
 *  - Serialize    -|_____ Use these methods in loading
 *  - Deserialize  -|      and StorageObject.save() method
 */

/**
 * Load storage objects from path
 * If path does not exist, create it.
 * During object loading, try to:
 *  1) serialize objects
 *  2) checking schema version
 *  3) try schema update if it's needed.
 */
pub fn load_storage<T>(path: &str) -> Result<Vec<T>, String> {
    let mut result: Vec<T> = Vec::new();
    Ok(result)
}

/**
 * Serialize object<T> -> Result<String, String>
 */
pub fn serialize_object<T: Serialize>(object: &T) -> Result<String, String> {
    match serde_yaml::to_string(object) {
        Ok(result) => Ok(result),
        Err(_) => Err("Error while data object serialisation.".to_owned()),
    }
}
/**
 * Deserialize &str into object<T>
 */
pub fn deserialize_object<'a, T: ?Sized>(s: &str) -> Result<T, String>
where
    for<'de> T: Deserialize<'de> + 'a,
{
    match serde_yaml::from_str(s) {
        Ok(t) => Ok(t),
        Err(_) => Err("Error while data object deserialization.".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Serialize, Deserialize)]
    struct Demo {
        name: String,
        age: u32,
    }
    #[test]
    fn test_serialize_object() {
        let d = Demo {
            name: "Lorem Ipsum".to_owned(),
            age: 99,
        };
        assert_eq!(
            serialize_object(&d),
            Ok("---\nname: Lorem Ipsum\nage: 99".to_owned())
        );
    }
    #[test]
    fn test_deserialize_object() {
        let object: Demo = deserialize_object("---\nname: Lorem Ipsum\nage: 99").unwrap();
        assert_eq!(object.name, "Lorem Ipsum".to_owned());
        assert_eq!(object.age, 99);
    }
}
