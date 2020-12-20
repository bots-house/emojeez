use std::convert::Infallible;

use anyhow;
use hyper::{Body, Request, Response, Server, body::Bytes};
use hyper::service::{make_service_fn, service_fn};
use reqwest::Client;
use regex::Regex;
use lazy_static::lazy_static;
use cached::proc_macro::cached;


lazy_static!(
    static ref IMG_LOC_RE: regex::Regex = Regex::new(r#"<img.*?src="(.+?)""#).unwrap();
);


#[cached(result = true)]
async fn get_emoji_png(emoji: String) -> anyhow::Result<Bytes> {
    let client = Client::new();
    let resp = client
        .get(&format!("https://emojipedia.org/{}/", emoji))
        .send()
        .await?
        .text()
        .await?;

    let first_match =
        IMG_LOC_RE.captures_iter(resp.as_str()).next();

    match first_match {
        Some(loc_url) => {
            match loc_url.get(1) {
                Some(url) => {
                    let emoji_data = client.get(url.as_str())
                        .send()
                        .await?
                        .bytes()
                        .await?;

                    return Ok(emoji_data);
                },
                _ => {}
            }
        },
        _ => {}
    };

    Err(anyhow::anyhow!("not found"))
}



async fn view(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let emoji = request.uri()
        .path()
        .trim_start_matches("/")
        .trim_end_matches("/");

    match get_emoji_png(emoji.to_string()).await {
        Ok(bin) => Ok(
            Response::builder()
                .status(200)
                .body(bin.to_vec().into())
                .unwrap()
            ),
        Err(_) => Ok(
            Response::builder()
                .status(404)
                .body("not found".into())
                .unwrap()
        ),
    }
}


#[tokio::main]
async fn main() {
    let addr = std::env::var("SERVER_ADDR").unwrap().parse().unwrap();
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(view))
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}