#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if let Err(e) = shuru_cli::run().await {
        eprintln!("\x1b[31mError:\x1b[0m {}", e);
        std::process::exit(shuru_core::utils::get_error_code(e));
    }
}
