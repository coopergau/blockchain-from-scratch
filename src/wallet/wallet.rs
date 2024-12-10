use primitive_types::U256;

//impl
// 1. Generate private key via password and salt
// 2. Generate public key via private key
// 3. Generate address via private key
// 4. Submit a transaction to mempool - sign and send

pub struct Wallet {
    account: Account,
    private_key: U256,
}