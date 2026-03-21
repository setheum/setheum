
<a name="0x1_balance"></a>

# Module `0x1::balance`

A balance module provides access to external balance-handling functionality,
which is contained within the Substrate layer in our case.

If a script wants to execute a transfer, a transaction which contains the script must receive two parameters:
- signer - A proof that the account has signed the transaction.
- cheque_amount - How many funds is the signer account allowing a script to transfer from that account.
The provided amount remains in the signer account if the script doesn't perform any transfer.


-  [Function `transfer`](#0x1_balance_transfer)
-  [Function `cheque_amount`](#0x1_balance_cheque_amount)
-  [Function `total_amount`](#0x1_balance_total_amount)


<pre><code></code></pre>



<a name="0x1_balance_transfer"></a>

## Function `transfer`

Perform a transfer.

Returns true in case the transfer executed successfully.
An error indicates insufficient funds in the provided cheque.


<pre><code><b>public</b> <b>fun</b> <a href="balance.md#0x1_balance_transfer">transfer</a>(src: &signer, dst: <b>address</b>, cheque_amount: u128): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="balance.md#0x1_balance_transfer">transfer</a>(src: &signer, dst: <b>address</b>, cheque_amount: u128): bool;
</code></pre>



</details>

<a name="0x1_balance_cheque_amount"></a>

## Function `cheque_amount`

Get the current cheque amount for the address.

The cheque amount is the amount the address is allowed to transfer within the current executing context.


<pre><code><b>public</b> <b>fun</b> <a href="balance.md#0x1_balance_cheque_amount">cheque_amount</a>(account: <b>address</b>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="balance.md#0x1_balance_cheque_amount">cheque_amount</a>(account: <b>address</b>): u128;
</code></pre>



</details>

<a name="0x1_balance_total_amount"></a>

## Function `total_amount`

Get the total balance for the address.


<pre><code><b>public</b> <b>fun</b> <a href="balance.md#0x1_balance_total_amount">total_amount</a>(account: <b>address</b>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="balance.md#0x1_balance_total_amount">total_amount</a>(account: <b>address</b>): u128;
</code></pre>



</details>
