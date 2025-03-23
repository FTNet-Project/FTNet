/// this function is called on startup, and initializes the .ftn directory if it doesn't exist
pub async fn init_if_required(dir: Option<String>) -> eyre::Result<std::path::PathBuf> {
    use eyre::WrapErr;

    let dir = match dir {
        Some(dir) => dir.into(),
        // https://docs.rs/directories/6.0.0/directories/struct.ProjectDirs.html#method.data_dir
        None => match directories::ProjectDirs::from("com", "FifthTry", "ftn") {
            Some(dir) => dir.data_dir().to_path_buf(),
            None => {
                return Err(eyre::anyhow!(
                    "dotftn init failed: can not find data dir when dir is not provided"
                ));
            }
        },
    };

    if !dir.exists() {
        // TODO: create the directory in an incomplete state, e.g., in the same parent,
        //       but with a different name, so that is creation does not succeed, we can
        //       delete the partially created directory, and depending on failure we may
        //       not cleanup, so the next run can delete it, and create afresh.
        //       we can store the timestamp in the temp directory, so subsequent runs
        //       know for sure the previous run failed (if the temp directory exists and
        //       is older than say 5 minutes).
        tokio::fs::create_dir_all(&dir)
            .await
            .wrap_err_with(|| format!("failed to create dotftn directory: {dir:?}"))?;
        let identities = ftn::utils::mkdir(&dir, "identities")?;
        ftn::utils::mkdir(&dir, "logs")?;
        super::lock_file(&dir).wrap_err("failed to create lock file")?;

        // we always create the default identity
        ftn::Identity::create(&identities).await?;
    }

    Ok(dir)
}
