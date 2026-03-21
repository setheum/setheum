/// A balance module provides access to external balance-handling functionality,
/// which is contained within the Substrate layer in our case.
///
/// If a script wants to execute a transfer, a transaction which contains the script must receive two parameters:
/// - signer - A proof that the account has signed the transaction.
/// - cheque_amount - How many funds is the signer account allowing a script to transfer from that account.
///                   The provided amount remains in the signer account if the script doesn't perform any transfer.
module substrate::balance {

    /// Perform a transfer.
    ///
    /// Returns true in case the transfer executed successfully.
    /// An error indicates insufficient funds in the provided cheque.
    native public fun transfer(src: &signer, dst: address, cheque_amount: u128): bool;

    /// Get the current cheque amount for the address.
    ///
    /// The cheque amount is the amount the address is allowed to transfer within the current executing context.
    native public fun cheque_amount(account: address): u128;

    /// Get the total balance for the address.
    native public fun total_amount(account: address): u128;
}
