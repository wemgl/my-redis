use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result = {:?}", result);

    Ok(())
}
