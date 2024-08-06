#![allow(unused)]

use std::borrow::Cow;

use wstd::http::{self, Body, Client, Response};
use wstd::http::{Headers, Request};
use wstd::iter::AsyncIterator;
use wstd::runtime::{block_on, Reactor};

mod bindings;
mod schema;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

struct Component{
}

impl bindings::exports::yosh::llm::llm::Guest for Component {
    fn infer(
        model: bindings::exports::yosh::llm::llm::InferencingModel,
        prompt: String,
        params: Option<bindings::exports::yosh::llm::llm::InferencingParams>,
    ) -> Result<bindings::exports::yosh::llm::llm::InferencingResult, bindings::exports::yosh::llm::llm::Error> {
        todo!()
    }
}

#[non_exhaustive]
enum Model {
    Llama3_1_8b,
}

async fn infer(reactor: &Reactor, model: Model, query: &str) -> Result<(), Error> {
    let client = http::Client::new(&reactor);
    let res = match model {
        Model::Llama3_1_8b => ollama(&client, "llama3.1:8b", query).await?,
    };
    println!("{res}");
    Ok(())
}

async fn ollama(client: &Client<'_>, model: &str, query: &str) -> Result<String, Error> {
    let req = Request::new(
        http::Method::Post,
        "http://localhost:11434/api/chat".parse()?,
    );

    let body = schema::Request {
        model: model.to_string(),
        messages: vec![schema::Message {
            role: schema::Role::User,
            content: query.to_string(),
        }],
        stream: false,
    };
    let body = serde_json::to_string(&body)?;
    let req = req.set_body(body);
    let mut res = client.send(req).await?;

    let output = read_to_end(&mut res).await?;
    let response: schema::Response = serde_json::from_slice(&output)?;
    Ok(response.message.content)
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
