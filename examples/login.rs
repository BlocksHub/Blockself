use blockself::HttpError;

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    let username = std::env::var("TURBOSELF_USERNAME").expect("username not defined");
    let password = std::env::var("TURBOSELF_PASSWORD").expect("password not defined");
    let client = blockself::Client::login(username, password, None).await?;

    println!("Logged in as {}", client.host_id);
    Ok(())
}
