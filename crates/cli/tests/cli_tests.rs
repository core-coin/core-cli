#[cfg(test)]
mod tests {
    use cli::Cli;
    use structopt::StructOpt;
    use types::DEFAULT_BACKEND;

    #[test]
    fn test_cli_default_flags() {
        let args = vec!["core-cli"];
        let cli = Cli::from_iter_safe(&args).unwrap();

        assert_eq!(cli.client, "go-core");
        assert_eq!(cli.backend, DEFAULT_BACKEND);
        assert_eq!(cli.datadir, "./data");
    }

    #[test]
    fn test_cli_custom_flags() {
        let args = vec![
            "core-cli",
            "--client",
            "custom-client",
            "--backend",
            "some-backend",
            "--datadir",
            "some-datadir",
        ];
        let cli = Cli::from_iter_safe(&args).unwrap();

        assert_eq!(cli.client, "custom-client");
        assert_eq!(cli.backend, "some-backend");
        assert_eq!(cli.datadir, "some-datadir");
    }

    #[test]
    fn test_short_flags() {
        let args = vec![
            "core-cli",
            "-c",
            "custom-client",
            "-b",
            "some-backend",
            "-d",
            "some-datadir",
        ];
        let cli = Cli::from_iter_safe(&args).unwrap();

        assert_eq!(cli.client, "custom-client");
        assert_eq!(cli.backend, "some-backend");
        assert_eq!(cli.datadir, "some-datadir");
    }
}