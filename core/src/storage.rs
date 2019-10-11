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

use crate::prelude::New;
use crate::user::model::user_v1::UserV1;
use crate::user::User;
use std::sync::Mutex;

pub struct ObjectStorage<T> {
    path: &'static str,
    data: Mutex<Vec<Box<T>>>,
}

// pub struct TableStorage<T> {
//     path: &'static str,
//     data: Mutex<Box<T>>,
// }

// pub struct CacheStorage<T> {
//     path: &'static str,
//     data: Mutex<Box<T>>,
// }

pub fn new_object_storage<T>(init_value: T) -> ObjectStorage<T> {
    ObjectStorage {
        path: "data",
        data: Mutex::new(vec![Box::new(init_value)]),
    }
}

pub trait Storage {
    type T;
    fn add_to_storage(&mut self, storage_object: Self::T) -> Result<(), String>;
    fn get_data(&self) -> &Mutex<Vec<Box<Self::T>>>;
}

impl<T> Storage for ObjectStorage<T> {
    type T = T;
    fn add_to_storage(&mut self, storage_object: T) -> Result<(), String> {
        self.data.lock().unwrap().push(Box::new(storage_object));
        Ok(())
    }
    fn get_data(&self) -> &Mutex<Vec<Box<Self::T>>> {
        &self.data
    }
}

// pub trait TStorageObject {
//     fn save(&self) -> Result<(), String>;
//     fn reload(&self) -> Result<(), String>;
//     fn get_storage_name(&self) -> Option<&str>;
//     fn set_storage_name(&self, storage: &str) -> Result<(), String>;
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_object_storage() {
        let mut u1 = UserV1::new();
        u1.set_user_id("demo_user").unwrap();
        u1.set_user_name("Demo User").unwrap();
        let mut u2 = UserV1::new();
        u2.set_user_id("demo_user2").unwrap();
        u2.set_user_name("Demo User2").unwrap();
        let mut u3 = UserV1::new();
        u3.set_user_id("demo_user3").unwrap();
        u3.set_user_name("Demo User3").unwrap();

        let mut storage: ObjectStorage<UserV1> = new_object_storage(u1);
        storage.add_to_storage(u2).unwrap();
        storage.add_to_storage(u3).unwrap();

        assert_eq!(storage.get_data().lock().unwrap().len(), 3);
        assert_eq!(
            &storage.get_data().lock().unwrap()[0].get_user_id(),
            &Some("demo_user".to_owned())
        );
        let stg = &*storage.get_data().lock().unwrap();
        for user in stg {
            assert_eq!(user.get_user_name().is_some(), true);
        }
    }
}
