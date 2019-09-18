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

use super::{rocket, TemplateContext};

use rocket::http::Method::*;
use rocket::http::Status;
use rocket::local::{Client, LocalResponse};
use rocket_contrib::templates::Template;

macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => {{
        let client = Client::new(rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    }};
}

#[test]
fn test_root() {
    // Check that the redirect works.
    for method in &[Get, Head] {
        dispatch!(
            *method,
            "/",
            |_: &Client, mut response: LocalResponse<'_>| {
                assert_eq!(response.status(), Status::SeeOther);
                assert!(response.body().is_none());

                let location: Vec<_> = response.headers().get("Location").collect();
                assert_eq!(location, vec!["/hello/Unknown"]);
            }
        );
    }

    // Check that other request methods are not accepted (and instead caught).
    for method in &[Post, Put, Delete, Options, Trace, Connect, Patch] {
        dispatch!(
            *method,
            "/",
            |client: &Client, mut response: LocalResponse<'_>| {
                let mut map = std::collections::HashMap::new();
                map.insert("path", "/");
                let expected = Template::show(client.rocket(), "error/404", &map).unwrap();

                assert_eq!(response.status(), Status::NotFound);
                assert_eq!(response.body_string(), Some(expected));
            }
        );
    }
}

#[test]
fn test_name() {
    // Check that the /hello/<name> route works.
    dispatch!(
        Get,
        "/hello/Jack%20Daniels",
        |client: &Client, mut response: LocalResponse<'_>| {
            let context = TemplateContext {
                title: "Hello",
                name: Some("Jack Daniels".into()),
                items: vec!["One", "Two", "Three"],
                parent: "layout",
            };

            let expected = Template::show(client.rocket(), "index", &context).unwrap();
            assert_eq!(response.status(), Status::Ok);
            assert_eq!(response.body_string(), Some(expected));
        }
    );
}

#[test]
fn test_404() {
    // Check that the error catcher works.
    dispatch!(
        Get,
        "/hello/",
        |client: &Client, mut response: LocalResponse<'_>| {
            let mut map = std::collections::HashMap::new();
            map.insert("path", "/hello/");

            let expected = Template::show(client.rocket(), "error/404", &map).unwrap();
            assert_eq!(response.status(), Status::NotFound);
            assert_eq!(response.body_string(), Some(expected));
        }
    );
}
