# 🔐 Encryption & Decryption Engine

The Tequel encryption system is built on the **Encrypt-then-MAC (EtM)** paradigm, ensuring cryptographic integrity before decryption.

## Technical Architecture
The encryption process follows a high-entropy pipeline:
1. **Initial Mix:** XOR operations combined with `Wrapping Add/Sub` using data, keys, and internal constants.
2. **Diffusion Layer:** Bit-level shuffling to ensure that a 1-bit change in the key results in a completely different ciphertext.
3. **Authentication (MAC):** A 384-bit Message Authentication Code is generated from the ciphertext, key, and salt.

## Implementation Example

```rust
use tequel::encrypt::{ TequelEncrypt, TequelEncryption };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut teq = TequelEncrypt::new()
        .with_salt("security_first");
    
    let original_data = b"Ultra sensible data";

    let real_key = "StrongKey123";
    let fake_key = "StrongKey100";

    let encrypted = teq.encrypt(original_data, real_key).unwrap();

    let result = teq.decrypt(&encrypted, fake_key); // <-- Here I set a wrong key to test

    match result {
        Ok(_) => panic!("Critical Fail: Tequel accepted a wrong key and generate trash"),
        Err(_) => println!("Integrity Security: Key Wrong Blocked")
    }
}
```


## Security guarantees

- **Constant-Time Comparison**: MAC verification uses a bitwise accumulator (`^` and `|`) to prevent timing side-channel attacks.
- **Dynamic Iterations:** While the base is 100 iterations, Tequel applies a +30% variance to the mix count to obscure the execution profile.
- **Error Handling:** Returns `InvalidMac` for integrity failures and `InvalidHex` for enconding issues.