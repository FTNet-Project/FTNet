//! The identity folder
//!
//! The identity folder is stored in $FTN/identities/<identity-id>.
//!
//! The identity-id is the public key of the identity, it is a 64 character long string.
//!
//! The private key is stored in the platform specific keychain, and the public key is used as the
//! identity-id.
//!
//! This folder contains the `db.sqlite` file which corresponds to the DB for the fastn package for
//! this identity.
//!
//! `package` is the folder that contains the fastn package for this identity.
//!
//! `package-template` is the folder that contains the original version of fastn package that was
//! used to create the `package` folder. This is stored so if the fastn package template is updated
//! in future, we can do three way merge and auto update the `package` folder, or show a conflict
//! resolution screen to the user. Inside the package-template we store the version of the fastn
//! template in a file called `version`, and the actual template in a folder called `template`.
//!
//! `devices` is the folder that contains the device drivers for this identity. The structure of
//! this folder is described in `device/run.rs` (TODO).
//!
//! `logs` is the folder that contains the logs for this identity. This contains fastn access logs
//! and other device access logs etc.
impl ftnet::Identity {
    pub async fn create(identities_folder: &std::path::Path) -> eyre::Result<Self> {
        use eyre::WrapErr;

        let public_key = {
            let mut rng = rand::rngs::OsRng;
            let secret_key = iroh::SecretKey::generate(&mut rng);
            // we do not want to keep secret key in memory, only in keychain
            ftnet::utils::save_secret(&secret_key)
                .wrap_err_with(|| "failed to store secret key to keychain")?;
            secret_key.public()
        };

        let now = std::time::SystemTime::now();
        let unixtime = now
            .duration_since(std::time::UNIX_EPOCH)
            .wrap_err_with(|| "failed to get unix time")?
            .as_secs();
        let tmp_dir = identities_folder.join(format!(
            "temp-{public_key}-{unixtime}",
            public_key = data_encoding::BASE32_DNSSEC.encode(public_key.as_bytes())
        ));

        ftnet::utils::mkdir(&tmp_dir, "package")?;
        ftnet::utils::mkdir(&tmp_dir, "package-template")?;
        // TODO: initialise the package directory with default fastn package template
        //       which is fetched from ftn-template.fifthtry.site (zip download)
        // TODO: let user specify the template URL, and download it from there
        // TODO: call `fastn update` in the folder to ensure all dependencies are downloaded

        // TODO: should we encrypt the contents of this folder to prevent tampering / snooping?

        ftnet::utils::mkdir(&tmp_dir, "devices")?;
        ftnet::utils::mkdir(&tmp_dir, "logs")?;

        let dir =
            identities_folder.join(data_encoding::BASE32_DNSSEC.encode(public_key.as_bytes()));
        std::fs::rename(&tmp_dir, dir)
            .wrap_err_with(|| "failed to rename {tmp_dir:?} to {dir:?}")?;

        Ok(Self {
            id: data_encoding::BASE32_DNSSEC.encode(public_key.as_bytes()),
            public_key,
        })
    }
}
