#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut app = tide::new();
    app.at("/").serve_dir("../output/")?;

    app.with(tide::utils::Before(
        |request: tide::Request<_>| async move {
            let _ = &request.url().as_str().to_string().replace("%20", " ");
            request
        },
    ));

    app.at("/measures/").serve_dir("../output/measures/")?;
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
