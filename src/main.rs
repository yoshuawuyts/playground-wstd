use wstd::http;
use wstd::http::Request;
use wstd::io;
use wstd::iter::AsyncIterator;
use wstd::net::TcpListener;
use wstd::runtime::block_on;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), Error> {
    block_on(|reactor| async move {
        let client = http::Client::new(&reactor);
        let req = Request::new(http::Method::Get, "https://example.com".parse()?);
        let res = client.send(req).await?;
        dbg!(res);
        Ok(())
    })
}
