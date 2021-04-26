#[tokio::main]
async fn main() {
    let lol = sqlx::migrate!();
}
