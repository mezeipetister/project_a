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

extern crate core_lib;

use core_lib::prelude::*;
use core_lib::storage;
use core_lib::user::model::user_v1::UserV1;
use core_lib::user::User;
use core_lib::user::*;

pub fn find_users_with_name<'a>(users: &'a Vec<UserV1>, key: &str) -> Vec<&'a UserV1> {
    let mut result: Vec<&UserV1> = Vec::new();
    for user in users {
        if user.get_user_name().is_some() {
            if user.get_user_name().unwrap().contains(key) {
                result.push(&user);
            }
        }
    }
    result
}

fn init_storage() {
    let mut user_storage = storage::load_storage::<UserV1>("../data/users").unwrap();
    for i in 1..100 {
        let mut user = UserV1::new();
        user.set_user_id(&format!("user_{}", i)).unwrap();
        user.set_user_name(&format!("User Name {}", i)).unwrap();
        storage::add_to_storage(&mut user_storage, user).unwrap();
    }
}

#[test]
fn test_user_storage_a() {
    init_storage();
    let user_storage = storage::load_storage::<UserV1>("../data/users").unwrap();
    assert_eq!(user_storage.data.len(), 99);
    user_storage.remove();
}

#[test]
fn test_user_storage_b() {
    init_storage();
    let storage = storage::load_storage::<UserV1>("../data/users").unwrap();
    assert_eq!(find_users_with_name(&storage.data, "77").len(), 1);
    storage.remove();
}
