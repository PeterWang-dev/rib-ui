use rib_ui::app;

#[tokio::main]
async fn main() {
    match app::run().await {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
