#[tokio::main]
async fn main() -> anyhow::Result<()> {
    localsend_tui::run_tui(localsend_tui::TuiOptions::default()).await
}
