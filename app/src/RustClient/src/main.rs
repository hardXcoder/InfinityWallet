mod instructions;
use anchor_client::solana_sdk::signer::keypair;
use instructions::instructions::*;
fn main() {
    let msw_keypair = keypair::Keypair::new();
    sign_transaction().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let a = get_transaction_acc().unwrap();
        assert_eq!(a.sigreceived, 4u16);
    }
}
