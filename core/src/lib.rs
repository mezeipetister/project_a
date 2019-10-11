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
extern crate bcrypt;
extern crate lettre;
extern crate lettre_email;
extern crate rand;

pub mod email;
pub mod error;
pub mod prelude;
pub mod storage;
pub mod user;

pub use error::*;
pub use user::login::*;
pub use user::password::*;
pub use user::user::*;
