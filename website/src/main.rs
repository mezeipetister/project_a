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
extern crate serde_derive;

use self::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};
use rocket::http::RawStr;
use rocket::response::{status, NamedFile, Redirect};
use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{handlebars, Template};
use serde::Serialize;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "Index",
            name: None,
            items: Vec::new(),
            parent: "layout",
        },
    )
}

#[get("/redirect")]
fn redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
            parent: "layout",
        },
    )
}

#[get("/submit_order?<name>&<age>")]
fn submit_order(name: &RawStr, age: usize) -> status::Accepted<String> {
    status::Accepted(Some(format!("Hello bello: {}, your age is: {}", name, age)))
}

#[get("/about")]
fn about() -> Template {
    Template::render(
        "about",
        &TemplateContext {
            title: "About",
            name: None,
            items: vec!["Four", "Five", "Six"],
            parent: "layout",
        },
    )
}

#[get("/about2")]
fn about2() -> Template {
    Template::render(
        "about",
        &TemplateContext {
            title: "About",
            name: None,
            items: vec!["Four", "Five", "Six"],
            parent: "layout2",
        },
    )
}

#[get["/login"]]
fn login() -> Template {
    #[derive(Serialize)]
    struct Data {
        title: &'static str,
        name: &'static str,
        age: u32,
        parent: &'static str,
    }
    Template::render(
        "login",
        &Data {
            title: "Login",
            name: "Peter Mezei",
            age: 30,
            parent: "layout",
        },
    )
}

#[post["/login"]]
fn login_post() -> Redirect {
    Redirect::to("/")
}

#[get["/logout"]]
fn logout() -> Template {
    #[derive(Serialize)]
    struct C {
        title: &'static str,
        parent: &'static str,
    };
    Template::render(
        "logout",
        &C {
            title: "Logout",
            parent: "layout",
        },
    )
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

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                static_file,
                index,
                login_post,
                redirect,
                hello,
                about,
                about2,
                submit_order,
                login,
                logout
            ],
        )
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
