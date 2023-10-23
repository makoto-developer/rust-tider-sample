#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/html").get(|_| async {
        Ok(tide::Response::builder(200)
            .body("<html><h2>Hello, tide framework!</h2></html>")
            .header("Server", "tide")
            .content_type(tide::http::mime::HTML)
            .build())
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}