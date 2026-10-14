use std::{
    fs,
    fs::OpenOptions,
    io::{ErrorKind, Write},
    path::Path,
};

use anyhow::{Context, Result};
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};

const FILE_NAME: &str = "server-tls.pem";

pub(super) fn load_or_generate(
    directory: &Path,
) -> Result<(CertificateDer<'static>, PrivateKeyDer<'static>)> {
    let path = directory.join(FILE_NAME);
    let pem = match fs::read(&path) {
        Ok(pem) => pem,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            fs::create_dir_all(directory).context("Failed to create TLS state directory")?;
            let certificate = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
            let pem = format!(
                "{}{}",
                certificate.cert.pem(),
                certificate.signing_key.serialize_pem()
            );
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options
                .open(&path)
                .context("Failed to create TLS identity file")?;
            file.write_all(pem.as_bytes())
                .context("Failed to write TLS identity")?;
            file.sync_all().context("Failed to save TLS identity")?;
            pem.into_bytes()
        }
        Err(error) => return Err(error).context("Failed to read TLS identity"),
    };
    let certificate = CertificateDer::from_pem_slice(&pem).context("Invalid TLS certificate")?;
    let key = PrivateKeyDer::from_pem_slice(&pem).context("Invalid TLS private key")?;
    Ok((certificate, key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn persists_identity_and_preserves_invalid_files() -> Result<()> {
        let directory = std::env::temp_dir().join(format!(
            "sown-tls-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
        ));
        let (certificate, key) = load_or_generate(&directory)?;
        let (loaded_certificate, loaded_key) = load_or_generate(&directory)?;
        assert_eq!(certificate, loaded_certificate);
        assert_eq!(key.secret_der(), loaded_key.secret_der());
        let path = directory.join(FILE_NAME);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(fs::metadata(&path)?.permissions().mode() & 0o777, 0o600);
        }
        fs::write(&path, b"invalid certificate")?;
        assert!(load_or_generate(&directory).is_err());
        assert_eq!(fs::read(&path)?, b"invalid certificate");
        fs::remove_dir_all(directory)?;
        Ok(())
    }
}
