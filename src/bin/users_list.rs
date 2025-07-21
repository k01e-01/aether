use aether::db;
use aether_macros::default_endpoint;
use askama::Template;
use lambda_http::{Body, Error, Request, Response};

#[derive(Debug, Template)]
#[template(path = "partials/user_list.html")]
struct Page {
    users: Vec<String>,
}

#[default_endpoint]
async fn handler(_event: Request) -> Result<Response<Body>, Error> {
    let users = db::get_users().await?;

    let template = Page { users };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(template.render()?.into())
        .map_err(Box::new)?;

    Ok(resp)
}
