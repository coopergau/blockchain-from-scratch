
// use a markle patricia tree that uses the address as the key to store the accounts

#[derive(Debug)]
struct Account {
    nonce: u64,
    balance: u128, // balance in ceej
}