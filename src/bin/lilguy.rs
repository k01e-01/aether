use aether_macros::default_endpoint;
use askama::Template;
use lambda_http::{Body, Error, Request, Response};

#[derive(Debug, Template)]
#[template(path = "lilguy.html")]
struct Page {}

#[default_endpoint]
async fn handler(_event: Request) -> Result<Response<Body>, Error> {
    let template = Page {};

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(template.render()?.into())
        .map_err(Box::new)?;

    Ok(resp)
}
