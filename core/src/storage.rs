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

/// # StorageObject
/// Storage can hold any StorageObject<T>.
/// Storage object ensures that an object can save and reload itself.
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
/// # Load storage objects from path
/// We use turbofish style
/// ```rust
/// use core_lib::storage::*;
/// struct Animal {
///     id: u32,
///     name: String,
/// }
/// let storage = load_storage::<Animal>("data/animals").unwrap();
/// assert_eq!(storage.data.len(), 0);
/// ```
pub fn load_storage<T>(path: &'static str) -> Result<Storage<T>, String> {
    let storage: Storage<T> = Storage {
        path,
        data: Vec::new(),
    };
    Ok(storage)
}

/// # Add StorageObject to Storage
/// Add StorageObject to Storage and returns NO reference.
/// ```rust,no_run
/// use core_lib::storage::*;
/// struct Animal {
///     id: u32,
///     name: String,
/// }
/// impl StorageObject for Animal {
///     fn get_id(&self) -> Option<&str> {
///         Some("1")
///     }
///     fn save(&self) -> Result<(), String> {
///         Ok(())
///     }
///     fn reload(&mut self) -> Result<(), String> {
///         Ok(())
///     }
///     fn get_path(&self) -> Option<&str> {
///         Some("path")
///     }
///     fn set_path(&mut self, path: &str) -> Result<(), String> {
///         Ok(())
///     }
/// }
/// let mut storage = load_storage::<Animal>("data/animals").unwrap();
/// let dog = Animal { id: 1, name: "Puppy Joe".to_owned(), };
/// let cat = Animal { id: 2, name: "Purple Rainbow".to_owned(), };
/// add_to_storage(&mut storage, cat).unwrap();
/// assert_eq!(storage.data[0].name, "Puppy Joe".to_owned());
/// assert_eq!(storage.data[1].name, "Purple Rainbow".to_owned());
/// ```
pub fn add_to_storage<T>(storage: &mut Storage<T>, mut storage_object: T) -> Result<(), String>
    where
        T: StorageObject,
{
    storage_object.set_path(storage.path).unwrap();
    storage.data.push(storage_object);
    Ok(())
}

/// # Add StorageObject to Storage and returns reference to it
/// ```rust,no_run
/// use core_lib::storage::*;
/// struct Animal {
///     id: u32,
///     name: String,
/// }
/// impl StorageObject for Animal {
///     fn get_id(&self) -> Option<&str> {
///         Some("1")
///     }
///     fn save(&self) -> Result<(), String> {
///         Ok(())
///     }
///     fn reload(&mut self) -> Result<(), String> {
///         Ok(())
///     }
///     fn get_path(&self) -> Option<&str> {
///         Some("path")
///     }
///     fn set_path(&mut self, path: &str) -> Result<(), String> {
///         Ok(())
///     }
/// }
/// let mut storage = load_storage::<Animal>("data/animals").unwrap();
/// let dog = Animal { id: 1, name: "Puppy Joe".to_owned(), };
/// let cat = Animal { id: 2, name: "Purple Rainbow".to_owned(), };
/// let mut dog_ref = add_to_storage_and_return_ref(&mut storage, dog).unwrap();
/// dog_ref.name = "Puppy Joe+".to_owned();
/// let mut cat_ref = add_to_storage_and_return_ref(&mut storage, cat).unwrap();
/// cat_ref.name = "Purple Rainbow+".to_owned();
/// assert_eq!(storage.data[0].name, "Puppy Joe+".to_owned());
/// assert_eq!(storage.data[1].name, "Purple Rainbow+".to_owned());
/// ```
pub fn add_to_storage_and_return_ref<T>(
    storage: &mut Storage<T>,
    storage_object: T,
) -> Result<&mut T, String>
    where
        T: StorageObject,
{
    let id = storage_object.get_id().unwrap().to_owned();
    storage.data.push(storage_object);
    let mut storage_result_index = 0;
    for item in &mut storage.data {
        if item.get_id().unwrap() == id {
            break;
        }
        storage_result_index += 1;
    }
    match storage.data.get_mut(storage_result_index) {
        Some(data_item) => Ok(data_item),
        None => Err(format!(
            "Error while getting reference to the new storage item."
        )),
    }
}

/// # Serialize object<T> -> Result<String, String>
/// Serialize a given object to String
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use core_lib::storage::*;
/// #[derive(Serialize, Deserialize)]
/// struct Animal {
///     id: u32,
///     name: String,
/// }
/// let dog = Animal { id: 1, name: "Puppy Joe".to_owned() };
/// let serialized_object = serialize_object(&dog).unwrap();
/// assert_eq!(serialized_object, "---\nid: 1\nname: Puppy Joe".to_owned());
/// ```
pub fn serialize_object<T: Serialize>(object: &T) -> Result<String, String> {
    match serde_yaml::to_string(object) {
        Ok(result) => Ok(result),
        Err(_) => Err("Error while data object serialisation.".to_owned()),
    }
}

/// # Deserialize &str into object<T>
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use core_lib::storage::*;
/// #[derive(Serialize, Deserialize)]
/// struct Animal {
///     id: u32,
///     name: String,
/// }
/// let animal: Animal = deserialize_object("---\nid: 1\nname: Puppy Joe").unwrap();
/// assert_eq!(animal.id, 1);
/// assert_eq!(animal.name, "Puppy Joe".to_owned());
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
        let mut storage = load_storage::<Example>("data/storage").unwrap();
        for item in 1..3 {
            storage.data.push(Example::new(
                &format!("{}", item),
                "data/storage",
                &format!("{}", item),
            ));
        }
        add_to_storage(&mut storage, Example::new("102", "", "102")).unwrap();
        add_to_storage(&mut storage, Example::new("103", "", "103")).unwrap();
        add_to_storage(&mut storage, Example::new("104", "", "104")).unwrap();

        let mut item =
            add_to_storage_and_return_ref(&mut storage, Example::new("105", "data/storage", "105"))
                .unwrap();
        item.name = "1009".to_owned();

        assert_eq!(storage.data.get(0).unwrap().name, "1");
        assert_eq!(storage.data.get(1).unwrap().name, "2");
        assert_eq!(storage.data.get(2).unwrap().name, "102");
        assert_eq!(storage.data.get(3).unwrap().name, "103");
        assert_eq!(storage.data.get(4).unwrap().name, "104");
        assert_eq!(storage.data.get(5).unwrap().name, "1009");

        assert_eq!(storage.data[5].get_path().unwrap(), "data/storage");
    }
}
