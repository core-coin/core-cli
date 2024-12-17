#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::{env, io::Write, path::PathBuf};
    use types::DEFAULT_BACKEND;
    use utils::utils::{create_tmp_dir, remove_tmp_dir};

    struct TestContext {
        datadir: PathBuf,
        cmd: Command,
    }

    impl TestContext {
        fn new() -> Self {
            let datadir = create_tmp_dir(None).to_path_buf();
            let mut cmd = Command::cargo_bin("core-cli").unwrap();

            cmd.env("CARGO_TARGET_DIR", "target/release");
            cmd.arg(format!("-b={}", DEFAULT_BACKEND));
            cmd.arg(format!("-d={}", datadir.display()));

            TestContext { datadir, cmd }
        }

        fn run_with_input(&mut self, input: String) -> &mut Command {
            let tmfile_path = self.create_tempfile_with_input(input.as_bytes());
            self.cmd.pipe_stdin(tmfile_path).unwrap()
        }

        pub fn datadir(&self) -> PathBuf {
            self.datadir.clone()
        }

        pub fn create_tempfile_with_input(&self, input: &[u8]) -> PathBuf {
            let mut tmpfile = std::fs::File::create(self.datadir().join("input.txt")).unwrap();
            tmpfile.write_all(input).unwrap();

            self.datadir().join("input.txt")
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            let current = env::current_dir().unwrap();
            remove_tmp_dir(current.join(self.datadir.clone())).unwrap();
        }
    }

    #[test]
    fn test_cli_help() {
        let mut cmd = Command::cargo_bin("core-cli").unwrap();
        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("USAGE"));
    }
    #[test]
    fn test_cli_exit_command() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("exit\n".to_string())
            .assert()
            .success();

        cmd.stdout(predicate::str::contains("Exiting..."));
    }

    #[test]
    fn test_cli_help_command() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("help\n".to_string())
            .assert()
            .success();

        cmd.stdout(predicate::str::contains("Available modules"));
        drop(context)
    }

    #[test]
    fn test_cli_invalid_command() {
        let mut context = TestContext::new();

        context
            .run_with_input("invalid_command\n".to_string())
            .assert()
            .success()
            .stdout(predicate::str::contains("Error: Unknown command."));
    }

    #[test]
    fn test_cli_get_block_height() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("xcb.get_block_height()\n".to_string())
            .assert()
            .success();

        // Check if the output is a number
        cmd.stdout(predicate::str::is_match(r"\d+\n$").unwrap());
    }

    #[test]
    fn test_cli_get_block_height_json() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("xcb.get_block_height(json)\n".to_string())
            .assert()
            .success();

        // Check if the output is a valid JSON
        cmd.stdout(predicate::str::is_match(r#"\{\s*\".*\"\s*:\s*.*\s*\}\n$"#).unwrap());
    }

    #[test]
    fn test_cli_get_block_latest_human() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("xcb.get_block(latest, human)\n".to_string())
            .assert()
            .success();

        // Check if the output is a block in human readable format
        cmd.stdout(predicate::str::contains(
            "Block {
    header: Header {",
        ));
    }

    #[test]
    fn test_cli_get_energy_price_string() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("xcb.get_energy_price(string)\n".to_string())
            .assert()
            .success();

        // Check if the output is a number
        cmd.stdout(predicate::str::is_match(r"\d+\n$").unwrap());
    }

    #[test]
    fn test_cli_unknown_command() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("xcb.unknown_command()\n".to_string())
            .assert()
            .success();

        cmd.stdout(predicate::str::contains("Unknown command"));
    }

    #[test]
    fn test_cli_unknown_module() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("unknown_module.unknown_command()\n".to_string())
            .assert()
            .success();

        cmd.stdout(predicate::str::contains("Invalid module name:"));
    }

    #[test]
    fn test_cli_xcbkey_new_prompt() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("xcbkey.new()\n123\n123\n".to_string())
            .assert()
            .success();

        // output must be like
        // Address: cb30f1cab89a38fceee3dd7201945baca7c04525e66b
        // Public key: 99688cd0fdf2864ef4a4966a55324a069a4dc5fc0879f17d61da697ab80e4b7e83f4ffa8e9bf4b34c1cc8c90c5c397cff11147cb3e1fd3b400
        // Private key: b44dc7245cd9325e6900740c6c64eb7b311236375354e268314f96bf9880d4632bad4089474d651252ec358f30ea01d6ed400229d7b6cb3d31
        cmd.stdout(
            predicate::str::is_match(
                r"Address: [a-f0-9]{44}\nPublic key: [a-f0-9]{114}\nPrivate key: [a-f0-9]{114}\n",
            )
            .unwrap(),
        );
    }

    #[test]
    fn test_cli_xcbkey_new_from_key_prompt() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("xcbkey.new_from_key()\nb44dc7245cd9325e6900740c6c64eb7b311236375354e268314f96bf9880d4632bad4089474d651252ec358f30ea01d6ed400229d7b6cb3d31\n123\n123\n".to_string()).assert().success();

        cmd.stdout(
            predicate::str::is_match(
                r"Address: cb30f1cab89a38fceee3dd7201945baca7c04525e66b\nPublic key: 99688cd0fdf2864ef4a4966a55324a069a4dc5fc0879f17d61da697ab80e4b7e83f4ffa8e9bf4b34c1cc8c90c5c397cff11147cb3e1fd3b400\nPrivate key: b44dc7245cd9325e6900740c6c64eb7b311236375354e268314f96bf9880d4632bad4089474d651252ec358f30ea01d6ed400229d7b6cb3d31\n",
            ).unwrap(),
        );
    }

    #[test]
    fn test_cli_xcbkey_new_from_key_prompt_only_password() {
        let mut context = TestContext::new();

        let cmd = context
            .run_with_input("xcbkey.new_from_key(\"b44dc7245cd9325e6900740c6c64eb7b311236375354e268314f96bf9880d4632bad4089474d651252ec358f30ea01d6ed400229d7b6cb3d31\", \"json\")\n123\n123\n".to_string()).assert().success();

        cmd.stdout(
            predicate::str::contains(
                r#"{"Keyfile":{"address":"cb30f1cab89a38fceee3dd7201945baca7c04525e66b","public_key":"99688cd0fdf2864ef4a4966a55324a069a4dc5fc0879f17d61da697ab80e4b7e83f4ffa8e9bf4b34c1cc8c90c5c397cff11147cb3e1fd3b400","private_key":"b44dc7245cd9325e6900740c6c64eb7b311236375354e268314f96bf9880d4632bad4089474d651252ec358f30ea01d6ed400229d7b6cb3d31"}}"#,
            )
        );
    }
}
