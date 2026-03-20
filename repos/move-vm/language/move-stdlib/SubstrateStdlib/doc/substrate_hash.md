
<a name="0x1_substrate_hash"></a>

# Module `0x1::substrate_hash`

Cryptographic hashes:
- Keccak-256: see https://keccak.team/keccak.html

In addition, SHA2-256 and SHA3-256 are available in <code>std::hash</code>. Note that SHA3-256 is a variant of Keccak: it is
NOT the same as Keccak-256.

Non-cryptograhic hashes:
- SipHash: an add-rotate-xor (ARX) based family of pseudorandom functions created by Jean-Philippe Aumasson and Daniel J. Bernstein in 2012


-  [Function `sip_hash`](#0x1_substrate_hash_sip_hash)
-  [Function `keccak256`](#0x1_substrate_hash_keccak256)
-  [Function `sha2_512`](#0x1_substrate_hash_sha2_512)
-  [Function `sha3_512`](#0x1_substrate_hash_sha3_512)
-  [Function `ripemd160`](#0x1_substrate_hash_ripemd160)
-  [Function `blake2b_256`](#0x1_substrate_hash_blake2b_256)


<pre><code></code></pre>



<a name="0x1_substrate_hash_sip_hash"></a>

## Function `sip_hash`

Returns the (non-cryptographic) SipHash of <code>bytes</code>. See https://en.wikipedia.org/wiki/SipHash


<pre><code><b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_sip_hash">sip_hash</a>(bytes: vector&lt;u8&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_sip_hash">sip_hash</a>(bytes: vector&lt;u8&gt;): u64;
</code></pre>



</details>

<a name="0x1_substrate_hash_keccak256"></a>

## Function `keccak256`

Returns the Keccak-256 hash of <code>bytes</code>.


<pre><code><b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_keccak256">keccak256</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_keccak256">keccak256</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;;
</code></pre>



</details>

<a name="0x1_substrate_hash_sha2_512"></a>

## Function `sha2_512`

Returns the SHA2-512 hash of <code>bytes</code>.


<pre><code><b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_sha2_512">sha2_512</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_sha2_512">sha2_512</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;;
</code></pre>



</details>

<a name="0x1_substrate_hash_sha3_512"></a>

## Function `sha3_512`

Returns the SHA3-512 hash of <code>bytes</code>.


<pre><code><b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_sha3_512">sha3_512</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_sha3_512">sha3_512</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;;
</code></pre>



</details>

<a name="0x1_substrate_hash_ripemd160"></a>

## Function `ripemd160`

Returns the RIPEMD-160 hash of <code>bytes</code>.

WARNING: Only 80-bit security is provided by this function. This means an adversary who can compute roughly 2^80
hashes will, with high probability, find a collision x_1 != x_2 such that RIPEMD-160(x_1) = RIPEMD-160(x_2).


<pre><code><b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_ripemd160">ripemd160</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_ripemd160">ripemd160</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;;
</code></pre>



</details>

<a name="0x1_substrate_hash_blake2b_256"></a>

## Function `blake2b_256`

Returns the BLAKE2B-256 hash of <code>bytes</code>.


<pre><code><b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_blake2b_256">blake2b_256</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="substrate_hash.md#0x1_substrate_hash_blake2b_256">blake2b_256</a>(bytes: vector&lt;u8&gt;): vector&lt;u8&gt;;
</code></pre>



</details>
