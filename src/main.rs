#![allow(unused)]

use std::borrow::Cow;

use wstd::http::{self, Body, Client, Response};
use wstd::http::{Headers, Request};
use wstd::iter::AsyncIterator;
use wstd::runtime::block_on;

mod schema;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), Error> {
    block_on(|reactor| async move {
        let client = http::Client::new(&reactor);
        // example(&client).await?;
        ollama(&client).await?;
        Ok(())
    })
}

async fn example(client: &Client<'_>) -> Result<(), Error> {
    let req = Request::new(http::Method::Get, "http://example.com".parse()?);
    let res = client.send(req).await?;
    println!("{}", res.status_code());
    println!("{:#?}", res.headers());
    Ok(())
}

async fn ollama(client: &Client<'_>) -> Result<(), Error> {
    let req = Request::new(
        http::Method::Post,
        "http://localhost:11434/api/chat".parse()?,
    );

    let body = schema::Request {
        model: "llama3.1:8b".to_string(),
        messages: vec![schema::Message {
            role: schema::Role::User,
            content: "1 + 1".to_string(),
        }],
        stream: false,
    };
    let body = serde_json::to_string(&body)?;
    let req = req.set_body(body);
    let mut res = client.send(req).await?;

    let output = read_to_end(&mut res).await?;
    let response: schema::Response = serde_json::from_slice(&output)?;
    println!("{response:?}");
    Ok(())
}

async fn read_to_end<B: Body>(mut res: &mut Response<B>) -> Result<Vec<u8>, Error> {
    let content_length = parse_content_length(&res.headers()) as usize;
    let mut output = vec![];
    let mut body = res.body();
    let mut total = 0;
    let mut buf = [0; 1024];
    loop {
        let read = body.read(&mut buf).await?;
        total += read;
        if read == 0 {
            break;
        }
        output.extend_from_slice(&buf[0..read]);
        if total == content_length {
            break;
        }
    }
    Ok(output)
}

fn parse_content_length(headers: &Headers) -> u64 {
    let value = headers
        .get(&Cow::Borrowed("content-length"))
        .unwrap()
        .get(0)
        .unwrap();
    let content_length = String::from_utf8(value.to_owned())
        .unwrap()
        .parse::<u64>()
        .unwrap();
    content_length
}
