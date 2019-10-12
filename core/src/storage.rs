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
