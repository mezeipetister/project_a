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

#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use]
extern crate rocket;
extern crate core_lib;
extern crate serde_derive;

use self::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};
use core_lib::prelude::*;
use core_lib::storage::*;
use core_lib::user::model::user_v1::UserV1;
use core_lib::user::User;
use core_lib::user::*;
use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::{status, NamedFile, Redirect};
use rocket::Request;
use rocket::{Data, State};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{handlebars, Template};
use serde::Serialize;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Serialize)]
struct TemplateContext<'a, T> {
    title: &'static str,
    name: Option<String>,
    items: &'a Vec<T>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/user")
}

#[get("/user")]
fn user(data: State<DataLoad>) -> Template {
    Template::render(
        "user",
        &TemplateContext {
            title: "User list",
            name: None,
            items: &data.inner().users.lock().unwrap().data,
            parent: "layout",
        },
    )
}

#[derive(FromForm)]
struct UserPost {
    name: Option<String>,
    email: Option<String>,
}

#[post("/user", data = "<user>")]
fn user_post(user: Form<UserPost>, data: State<DataLoad>) -> Redirect {
    let mut new_user = UserV1::new();
    new_user
        .set_user_id(&format!(
            "myidis_{}",
            data.inner().users.lock().unwrap().data.len() + 1
        ))
        .unwrap();
    new_user.set_user_name(user.name.as_ref().unwrap()).unwrap();
    new_user
        .set_user_email(user.email.as_ref().unwrap())
        .unwrap();
    add_to_storage(&mut data.inner().users.lock().unwrap(), new_user).unwrap();
    Redirect::to("/user")
}

#[get("/static/<file..>")]
pub fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

struct DataLoad {
    users: Mutex<Storage<UserV1>>,
}

fn rocket(data: DataLoad) -> rocket::Rocket {
    rocket::ignite()
        .manage(data)
        .mount("/", routes![static_file, index, user, user_post,])
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    let user_storage = load_storage::<UserV1>("../data/users").unwrap();
    let data = DataLoad {
        users: Mutex::new(user_storage),
    };
    rocket(data).launch();
}
