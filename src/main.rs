// Copyright 2019 peti
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use]
extern crate rocket;
extern crate serde_derive;

#[cfg(test)]
mod tests;

use rocket::response::Redirect;
use rocket::Request;
use rocket_contrib::templates::{handlebars, Template};
use serde::Serialize;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/hello/Unknown")
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

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

use self::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};

fn wow_helper(
    h: &Helper<'_, '_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_>,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, hello, about, about2])
        .register(catchers![not_found])
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("wow", Box::new(wow_helper));
        }))
}

fn main() {
    rocket().launch();
}
