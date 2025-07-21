use aether::db;
use aether_macros::default_endpoint;
use askama::Template;
use lambda_http::{Body, Error, Request, Response};
use serde::Deserialize;

#[derive(Debug, Template)]
#[template(path = "partials/user_list.html")]
struct Page {
    users: Vec<String>,
}

#[derive(Deserialize)]
struct Form {
    add_user: String,
}

#[default_endpoint]
async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let body = std::str::from_utf8(event.body())?;
    let form = serde_urlencoded::from_str::<Form>(body)?;

    let users = db::add_user(form.add_user).await?;

    let template = Page { users };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(template.render()?.into())
        .map_err(Box::new)?;

    Ok(resp)
}
