use std::path::Path;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    server::run_dedicated(
        include_str!("../../../fixtures/minimal-room/worlds/main.world"),
        "127.0.0.1:50420".parse()?,
        Path::new(".sown"),
    )
    .await
}
