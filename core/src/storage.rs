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

pub trait StorageObject {
    fn get_id(&self) -> Option<&str>;
    fn save(&self) -> Result<(), String>;
    fn reload(&mut self) -> Result<(), String>;
    fn get_path(&self) -> Option<&str>;
    fn set_path(&mut self, path: &str) -> Result<(), String>;
}

pub struct Storage<T> {
    path: &'static str,
    pub data: Vec<T>,
}

/**
 * Load storage objects from path
 * If path does not exist, create it.
 * During object loading, try to:
 *  1) serialize objects
 *  2) checking schema version
 *  3) try schema update if it's needed.
 */
// pub fn load_storage<T>(path: &'static str) -> Result<T, String> {
//     let storage: T = Storage {
//         path,
//         data: Vec::new(),
//     };
//     Ok(storage)
// }

/**
 * Add StorageObject to Storage and returns NO reference
 */
pub fn add_to_storage<T>(storage: &mut Storage<T>, storage_object: T) -> Result<(), String>
    where
        T: StorageObject,
{
    storage.data.push(storage_object);
    Ok(())
}

/**
 * Add StorageObject to Storage and returns reference to it
 */
pub fn add_to_storage_and_return_ref<'a, T>(
    storage: &'a mut Storage<T>,
    storage_object: T,
) -> Result<&'a mut T, String>
    where
        T: StorageObject,
{
    let id = storage_object.get_id().unwrap().to_owned();
    storage.data.push(storage_object);
    for item in &mut storage.data {
        if item.get_id().unwrap() == id {
            return Ok(&mut item);
        }
    }
    Err("Error while inser & find reference to it in storage.".to_owned())
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

/// # Deserialize &str into object<T>
/// ```rust
/// ```
/// IMPORTANT: deserializable struct currently cannot have &str field.
//  TODO: Lifetime fix for `&str field type.
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

    #[test]
    fn test_storage() {
        struct Example {
            id: String,
            path: String,
            name: String,
        }
        impl Example {
            fn new(id: &str, path: &str, name: &str) -> Example {
                Example {
                    id: id.to_owned(),
                    path: path.to_owned(),
                    name: name.to_owned(),
                }
            }
        }
        impl StorageObject for Example {
            fn get_id(&self) -> Option<&str> {
                Some(&self.id)
            }
            fn save(&self) -> Result<(), String> {
                Ok(())
            }
            fn reload(&mut self) -> Result<(), String> {
                Ok(())
            }
            fn get_path(&self) -> Option<&str> {
                Some(&self.path)
            }
            fn set_path(&mut self, path: &str) -> Result<(), String> {
                self.path = path.to_owned();
                Ok(())
            }
        }
        let mut storage: Storage<Example> = Storage {
            path: "data/storage",
            data: Vec::new(),
        };
        for item in 1..3 {
            storage.data.push(Example::new(&format!("{}", item),
                                           "data/storage", &format!("Name_{}", item)));
        }
        add_to_storage(&mut storage, Example::new("102",
                                                  "data/storage", "102")).unwrap();
        add_to_storage(&mut storage, Example::new("103",
                                                  "data/storage", "103")).unwrap();
        add_to_storage(&mut storage, Example::new("104",
                                                  "data/storage", "104")).unwrap();

        let mut item = add_to_storage_and_return_ref(&mut storage, &mut Example::new("105",
                                                                                     "data/storage", "105")).unwrap();
        item.name = "1009".to_owned();

        for _item in &storage.data {
            let item = item.get_id().unwrap();
            assert_eq!(item, "1");
            assert_eq!(item, "2");
            assert_eq!(item, "3");
            assert_eq!(item, "102");
            assert_eq!(item, "103");
            assert_eq!(item, "1009");
        }
    }
}
