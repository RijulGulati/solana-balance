use std::str::FromStr;

use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

#[derive(Serialize)]
pub struct SolanaBalance {
    lamports: u64,
    sol: f64,
}

#[derive(Debug, Serialize)]
pub struct SolanaError {
    pub error: String,
}

pub enum Cluster {
    Testnet,
    Devnet,
    MainnetBeta,
}

impl Cluster {
    pub fn value(&self) -> &str {
        match self {
            &Cluster::Devnet => "https://api.devnet.solana.com",
            &Cluster::MainnetBeta => "https://api.mainnet-beta.solana.com",
            &Cluster::Testnet => "https://api.testnet.solana.com",
        }
    }
}

pub fn get_solana_balance(pubkey: &str, cluster: Cluster) -> Result<SolanaBalance, SolanaError> {
    let rpc = RpcClient::new(String::from(cluster.value()));
    let pubkey = match Pubkey::from_str(pubkey) {
        Ok(key) => key,
        Err(err) => {
            println!("{:?}", err);
            return Err(SolanaError {
                error: String::from("invalid pubkey"),
            });
        }
    };

    match rpc.get_account(&pubkey) {
        Ok(acc) => {
            let balance: SolanaBalance = SolanaBalance {
                lamports: acc.lamports,
                sol: (acc.lamports as f64) / 1000000000.0,
            };
            Ok(balance)
        }

        Err(err) => {
            println!("{:?}", err);
            return Err(SolanaError {
                error: err.to_string(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CORRECT_ACC_ADDRESS: &str = "9aavjzd4iAbiJHawgS7kunfCJefSRRVKso61vzAX9Ho5";
    const INCORRECT_ACC_ADDRESS: &str = "wrongaddress";
    const ACCOUNT_NOT_FOUND: &str = "9aavjzd4iAbiJHawgS7kunfCJefSRRVKso61vzAX9Ho6"; // notice 6 at the end

    #[test]
    fn get_balance() {
        let result = get_solana_balance(CORRECT_ACC_ADDRESS, Cluster::Devnet).unwrap();
        assert_eq!(result.lamports, 599985000);
        assert_eq!(result.sol, 0.599985);
    }

    #[test]
    fn invalid_pubkey() {
        let result = get_solana_balance(INCORRECT_ACC_ADDRESS, Cluster::Devnet)
            .err()
            .unwrap();
        assert_eq!(result.error, "invalid pubkey");
    }

    #[test]
    fn acc_not_found() {
        let result = get_solana_balance(ACCOUNT_NOT_FOUND, Cluster::Devnet)
            .err()
            .unwrap();
        assert_eq!(
            result.error,
            format!("AccountNotFound: pubkey={}", ACCOUNT_NOT_FOUND)
        );
    }
}
