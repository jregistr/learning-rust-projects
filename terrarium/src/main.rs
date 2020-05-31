#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate failure;
extern crate graphql_client;
#[macro_use]
extern crate http_guest;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maud;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[allow(unused_imports)]
use failure::{Error, ResultExt};
#[allow(unused_imports)]
use graphql_client::GraphQLQuery;
#[allow(unused_imports)]
use http_guest::{header, Request, RequestExt, Response};
#[allow(unused_imports)]
use maud::{DOCTYPE, Markup};
#[allow(unused_imports)]
use regex::Regex;

struct Comment {
    user_name: String,
    comment_text: String,
}

struct PullRequestDesc {
    id: String,
    title: String,
}

struct PullRequest {
    desc: PullRequestDesc,
    comments: Vec<Comment>,
}

/*
Create a web app with 3 routes.

/ root route
The root route should display a list of Strings where each string is the name of a pull request in a
github repository.
The strings should also be links.

When clicked, go to a page /pull/id
that displays another  list of strings pairs where this time the strings are comments paired with a user name, that are attached to that
PR.

If a user makes the request /pull/id/name/text,
return the pr page again but with a new comment.
 */


pub fn server(req: &Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Error> {
    let path_reg: Regex = Regex::new("/pull/([^/]+)/([^/]*)/([^/]*)").expect("create regex");

    let path = req.uri().path();

    let response_text = match path_reg.captures(path) {
        None if path == "/" || path == "" => {
            root_page(&[])
        }
        Some(captures) => {
            let pull_id = captures
                .get(1)
                .map(|matched| matched.as_str())
                .ok_or_else(|| format_err!("Could not determine comment"))?;

            let maybe_name = captures
                .get(2)
                .map(|m| m.as_str());

            let maybe_comment_text = captures
                .get(3)
                .map(|m| m.as_str());

            if let (Some(name), Some(comment)) = (maybe_name, maybe_comment_text) {
                let desc = PullRequestDesc { id: String::from("super-id"), title: String::from("Coolest Pull Request") };
                let comments = vec![Comment { user_name: String::from("jregistr"), comment_text: String::from("This is a comment") }];
                let pr = PullRequest {desc, comments };

                let bla = pr_with_comments_page(&pr);
                bla
            } else {
                get_pr_comments_and_display_page(pull_id)
            }
        }
        _ => return Err(format_err!("These are not the droids you're looking for."))
    };

    Ok(Response::builder()
        .status(200)
        .body("Hello, world!".as_bytes().to_owned())
        .unwrap())
}

fn get_pr_comments_and_display_page(pr_id: &str) -> Result<Markup, &str> {
    unimplemented!()
}

fn pr_with_comments_page(pull_request: &PullRequest) -> Result<Markup, &str> {
    return Ok(html! {
        h1 { "This is the PR page." }
    })
}

// Takes a slice of pull request descriptions
fn root_page(pulls: &[PullRequestDesc]) -> Result<Markup, &str> {
    return Ok(html! {
        h1 { "Well, Hello World! From an H1" }
    })
}

fn user_entry_point(req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    match server(req) {
        Ok(resp) => resp,
        Err(e) => {
            let body = format!("Uh oh, looks like we encountered an error: {:?}", e);
            Response::builder()
                .status(500)
                .body(body.as_bytes().to_owned())
                .unwrap()
        }
    }
}

fn main() {
    guest_app!(user_entry_point);
}
