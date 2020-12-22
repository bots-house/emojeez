mod resizer;
mod styles;

use std::cmp::min;
use std::collections::HashMap;
use std::convert::Infallible;

use cached::proc_macro::cached;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use reqwest::Client;

const MAX_SIZE: (u32, u32) = (600, 600);

#[cached(result = true, time = 43200)]
async fn get_emoji_png(emoji: String, style: String) -> anyhow::Result<Vec<u8>> {
    let client = Client::new();
    let resp = client
        .get(&format!("https://emojipedia.org/{}/", emoji))
        .send()
        .await?
        .text()
        .await?;

    let first_match = styles::Style::regex_from_string(&style)?
        .captures_iter(resp.as_str())
        .next();

    if let Some(first_match) = first_match {
        if let Some(loc_url) = first_match.get(1) {
            if let Some(hq_url) = loc_url.as_str().split_whitespace().next() {
                let emoji_data = client.get(hq_url).send().await?.bytes().await?;
                return Ok(emoji_data.to_vec());
            };
        };
    };

    Err(anyhow::anyhow!("not found"))
}

async fn view(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let required_item = request
        .uri()
        .path()
        .trim_start_matches('/')
        .trim_end_matches('/')
        .to_string();

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

    let fallback = String::from("apple");
    let style = params.get("style").unwrap_or(&fallback); // todo

    let fallback: (u32, u32) = (0, 0);
    let size = params.get("size").map_or(fallback, |val| {
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
    });

    match get_emoji_png(required_item, style.to_lowercase()).await {
        Ok(bin) => {
            let bin = if size.0 == 0 && size.1 == 0 {
                bin
            } else {
                resizer::resize_png(size, &bin).unwrap_or(bin)
            };
            Ok(Response::builder()
                .status(200)
                .header("cache-control", format!("public, max-age={}", 864000))
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
