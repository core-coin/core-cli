#[cfg(test)]
mod tests {
    use atoms_signer_wallet as wallet;
    use cli_error::CliError;
    use serde::ser::Error;
    use std::io;

    #[test]
    fn test_rpc_error() {
        let error_message = "RPC error occurred";
        let error = CliError::RpcError(error_message.to_string());
        assert_eq!(
            format!("{}", error),
            format!("RPC error: {}", error_message)
        );
    }

    #[test]
    fn test_unknown_module_error() {
        let module_name = "unknown_module";
        let error = CliError::UnknownModule(module_name.to_string());
        assert_eq!(
            format!("{}", error),
            format!(
                "Invalid module name: {}. Please write 'list' to get list of all possible modules and commands",
                module_name
            )
        );
    }

    #[test]
    fn test_unknown_command_error() {
        let error = CliError::UnknownCommand;
        assert_eq!(
            format!("{}", error),
            "Unknown command. Please write 'list' to get list of all possible commands"
        );
    }

    #[test]
    fn test_unknown_client_error() {
        let client_name = "unknown_client";
        let error = CliError::UnknownClient(client_name.to_string());
        assert_eq!(
            format!("{}", error),
            format!("Unknown client: {}", client_name)
        );
    }

    #[test]
    fn test_invalid_number_of_arguments_error() {
        let expected_args = "2";
        let error = CliError::InvalidNumberOfArguments(expected_args.to_string());
        assert_eq!(
            format!("{}", error),
            format!("Invalid number of arguments: must be {}", expected_args)
        );
    }

    #[test]
    fn test_invalid_hex_argument_error() {
        let hex_arg = "0xZZZ";
        let error = CliError::InvalidHexArgument(hex_arg.to_string());
        assert_eq!(
            format!("{}", error),
            format!("Invalid hex argument: {}", hex_arg)
        );
    }

    #[test]
    fn test_invalid_argument_error() {
        let arg = "arg1";
        let expected = "expected_value";
        let error = CliError::InvalidArgument(arg.to_string(), expected.to_string());
        assert_eq!(
            format!("{}", error),
            format!("Invalid argument: {}. Must be {}", arg, expected)
        );
    }

    #[test]
    fn test_wallet_error() {
        let wallet_error =
            wallet::WalletError::IoError(io::Error::new(io::ErrorKind::Other, "IO error"));
        let error = CliError::WalletError(wallet_error);
        assert_eq!(format!("{}", error), "Wallet error: IO error");
    }

    #[test]
    fn test_account_not_found_error() {
        let address = "0x123";
        let error = CliError::AccountNotFound(address.to_string());
        assert_eq!(
            format!("{}", error),
            format!("Account with address {} not found", address)
        );
    }

    #[test]
    fn test_account_not_unlocked_error() {
        let address = "0x123";
        let error = CliError::AccountNotUnlocked(address.to_string());
        assert_eq!(
            format!("{}", error),
            format!("Account with address {} is not unlocked", address)
        );
    }

    #[test]
    fn test_io_error() {
        let io_error = io::Error::new(io::ErrorKind::Other, "IO error");
        let error = CliError::IoError(io_error);
        assert!(format!("{}", error).contains("Error: IO error"));
    }

    #[test]
    fn test_serde_error() {
        let serde_error = serde_json::Error::custom("Serde error");
        let error = CliError::SerdeError(serde_error);
        assert!(format!("{}", error).contains("Error: Serde error"));
    }

    #[test]
    fn test_atoms_signer_error() {
        let signer_error =
            atoms_signer::Error::HexError(base_primitives::hex::FromHexError::OddLength);
        let error = CliError::AtomsSignerError(signer_error);
        assert!(format!("{}", error).contains("odd number of digits"));
    }
}
