mod resizer;
mod styles;

use std::cmp::min;
use std::collections::HashMap;
use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use lazy_static::lazy_static;
use reqwest::Client;

const CACHE_CONTROL_MAX: u32 = 3600 * 24 * 10;
const MAX_SIZE: (u32, u32) = (600, 600);

async fn get_emoji_png(emoji: &str, style: &str, client: &Client) -> anyhow::Result<Vec<u8>> {
    let resp = client
        .get(&format!("https://emojipedia.org/{}/", emoji))
        .send()
        .await?
        .text()
        .await?;

    let url = styles::Style::regex_from_string(style)
        .and_then(|rexp| rexp.captures_iter(resp.as_str()).next())
        .and_then(|matches| matches.get(1))
        .and_then(|dirty_location_url| dirty_location_url.as_str().split_whitespace().next());

    match url {
        Some(hq_url) => {
            let emoji_data = client.get(hq_url).send().await?.bytes().await?;
            Ok(emoji_data.to_vec())
        }
        None => Err(anyhow::anyhow!("not found")),
    }
}

async fn view(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    lazy_static! {
        static ref CLIENT: Client = Client::new();
    };

    let required_item = request
        .uri()
        .path()
        .trim_start_matches('/')
        .trim_end_matches('/');

    if required_item == "ping" {
        return Ok(Response::new("pong".into()));
    }

    let params: HashMap<String, String> = request
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    let style = {
        let fallback = String::from("apple");
        params
            .get("style")
            .map(|s| s.to_lowercase())
            .unwrap_or(fallback)
    };

    let size = {
        let fallback: (u32, u32) = (0, 0);
        params.get("size").map_or(fallback, |val| {
            let mut iter = val.split(':');

            // we fallback to unset if w in {w}:{h} was not passed
            let first_size = min(
                MAX_SIZE.0,
                iter.next()
                    .map_or(fallback.0, |n| n.parse::<u32>().unwrap_or(fallback.0)),
            );

            // we fallback to first_size if h in {w}:{h} was not passed

            let second_size = min(
                MAX_SIZE.1,
                iter.next()
                    .map_or(first_size, |n| n.parse::<u32>().unwrap_or(first_size)),
            );

            (first_size, second_size)
        })
    };

    match get_emoji_png(required_item, &style, &CLIENT).await {
        Ok(bin) => {
            let bin = if size.0 == 0 && size.1 == 0 {
                bin
            } else {
                resizer::resize_png(size, &bin).unwrap_or(bin)
            };
            Ok(Response::builder()
                .status(200)
                .header(
                    "cache-control",
                    format!("public, max-age={}", CACHE_CONTROL_MAX),
                )
                .header("content-type", "image/png")
                .body(bin.into())
                .unwrap())
        }
        Err(_) => Ok(Response::builder()
            .status(404)
            .body("not found :'(".into())
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {
    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:8000"));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(view)) });
    let server = Server::bind(&addr.parse().unwrap()).serve(make_svc);
    println!("💖 listening 💘\n\t> try: http://{}/crying-face/", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
