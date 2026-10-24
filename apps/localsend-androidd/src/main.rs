fn main() -> anyhow::Result<()> {
    let options = localsend_androidd::Options::from_env()?;
    localsend_androidd::init_logging(&options.log_file)?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    runtime.block_on(localsend_androidd::run(options))
}
