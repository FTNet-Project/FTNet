use eyre::WrapErr;

impl ftnet::Config {
    pub async fn lock(&self) -> eyre::Result<file_guard::FileGuard<&std::fs::File>> {
        ftnet::config::dotftnet::exclusive(&self.lock_file)
            .await
            .wrap_err_with(|| "Config::lock(): failed to take exclusive lock")
    }

    pub async fn read(
        dir: &std::path::Path,
        client_pools: ftnet::http::client::ConnectionPools,
    ) -> eyre::Result<Self> {
        let dir = ftnet::config::dotftnet::init_if_required(dir, client_pools)
            .await
            .wrap_err_with(|| "Config: failed to get init directory")?;
        let lock_file = ftnet::config::dotftnet::lock_file(&dir)
            .wrap_err_with(|| "failed to create lock file")?;
        Ok(Self { dir, lock_file })
    }
}
