mod styles;

use std::collections::HashMap;
use std::convert::Infallible;

use anyhow;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use reqwest::Client;
use cached::proc_macro::cached;
use url;


#[cached(result = true)]
async fn get_emoji_png(emoji: String, style: String) -> anyhow::Result<Vec<u8>> {
    let client = Client::new();
    let resp = client
        .get(&format!("https://emojipedia.org/{}/", emoji))
        .send()
        .await?
        .text()
        .await?;

    let first_match =
        styles::Style::
            regex_from_string(&style)?
            .captures_iter(resp.as_str())
            .next();

    match first_match {
        Some(loc_url) => {
            match loc_url.get(1) {
                Some(url) => {
                    if let Some(hq_url) = url.as_str().split_whitespace().next() {
                        let emoji_data = client.get(hq_url)
                            .send()
                            .await?
                            .bytes()
                            .await?;

                        return Ok(emoji_data.to_vec());
                    }
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
        .trim_end_matches("/")
        .to_string();

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
    match get_emoji_png(emoji, style.to_lowercase()).await {
        Ok(bin) => Ok(
            Response::builder()
                .status(200)
                .header("content-type", "image/png")
                .body(bin.into())
                .unwrap()
            ),
        Err(_) => Ok(
            Response::builder()
                .status(404)
                .body("not found :'(".into())
                .unwrap()
        ),
    }
}


#[tokio::main]
async fn main() {
    let addr = std::env::var("SERVER_ADDR").unwrap();
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(view))
    });
    println!("💖 listening 💘\n\t> try: http://{}/crying-face/", addr);
    let server = Server::bind(&addr.parse().unwrap()).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}