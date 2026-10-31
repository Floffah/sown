use client::application::run;

fn main() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        run(server_address(std::env::args().skip(1))?)
    }
    #[cfg(target_arch = "wasm32")]
    {
        run()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn server_address(
    mut args: impl Iterator<Item = String>,
) -> anyhow::Result<Option<std::net::SocketAddr>> {
    use anyhow::Context;
    match (args.next().as_deref(), args.next(), args.next()) {
        (None, None, None) => Ok(None),
        (Some("--connect"), Some(address), None) => {
            Ok(Some(address.parse().context("Expected --connect IP:PORT")?))
        }
        _ => anyhow::bail!("Usage: client [--connect IP:PORT]"),
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::server_address;

    #[test]
    fn accepts_default_or_explicit_server_and_rejects_invalid_arguments() {
        let parse = |args: &[&str]| server_address(args.iter().map(|arg| (*arg).to_owned()));
        assert_eq!(parse(&[]).unwrap(), None);
        for address in ["127.0.0.1:50420", "[::1]:50420"] {
            assert_eq!(
                parse(&["--connect", address]).unwrap(),
                Some(address.parse().unwrap())
            );
        }
        for args in [
            vec!["--connect"],
            vec!["--connect", "invalid"],
            vec!["--connect", "127.0.0.1"],
            vec!["--unknown"],
            vec!["--connect", "127.0.0.1:50420", "extra"],
        ] {
            assert!(
                parse(&args).is_err(),
                "Accepted invalid arguments: {args:?}"
            );
        }
    }
}
