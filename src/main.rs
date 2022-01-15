#[allow(unused_imports)]
use anyhow::Context as _;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let inkdrop_http_user =
        env::var("INKDROP_HTTP_USER").context("You should set a value for $INKDROP_HTTP_USER")?;
    let inkdrop_http_password = env::var("INKDROP_HTTP_PASSWORD")
        .context("You should set a value for $INKDROP_HTTP_PASSWORD")?;
    let inkdrop_http_port =
        env::var("INKDROP_HTTP_PORT").context("You should set a value for $INKDROP_HTTP_PORT")?;

    let basic_auth_value =
        base64::encode(format!("{}:{}", inkdrop_http_user, inkdrop_http_password));

    let text = ureq::get(&format!("http://localhost:{}", inkdrop_http_port))
        .set("Authorization", &format!("Basic {}", basic_auth_value))
        .call()
        .context("Failed to get request to http://example.com")?
        .into_string()?;
    println!("{}", text);
    Ok(())
}
