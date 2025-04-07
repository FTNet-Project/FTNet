/// this function is called on startup, and initializes the FTNet directory if it doesn't exist
#[tracing::instrument(skip(client_pools))]
pub async fn init_if_required(
    dir: &std::path::Path,
    client_pools: ftnet::http::client::ConnectionPools,
) -> eyre::Result<std::path::PathBuf> {
    use eyre::WrapErr;

    if !dir.exists() {
        // TODO: create the directory in an incomplete state, e.g., in the same parent,
        //       but with a different name, so that is creation does not succeed, we can
        //       delete the partially created directory, and depending on failure we may
        //       not clean up, so the next run can delete it, and create afresh.
        //       we can store the timestamp in the temp directory, so subsequent runs
        //       know for sure the previous run failed (if the temp directory exists and
        //       is older than say 5 minutes).
        tokio::fs::create_dir_all(&dir)
            .await
            .wrap_err_with(|| format!("failed to create dotFTNet directory: {dir:?}"))?;
        let identities = ftnet::utils::mkdir(dir, "identities")?;
        ftnet::utils::mkdir(dir, "logs")?;

        super::lock_file(dir).wrap_err_with(|| "failed to create lock file")?;

        // we always create the default identity
        ftnet::Identity::create(&identities, client_pools).await?;
    }

    Ok(dir.to_path_buf())
}
