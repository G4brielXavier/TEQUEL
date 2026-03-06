use tequel_rs::{Tequel, TequelEncryption};

#[test]
fn test_dt_hash_size_return() {
    let mut tequel = Tequel::new();

    assert_eq!(tequel.dt_hash("a").len(), 80); // OK!
}


// Test if df_hash returns a different HASH even that input is equal
#[test]
fn test_df_hash_is_different() {
    let mut tequel = Tequel::new();

    let hash1 = tequel.df_hash("a");
    let hash2 = tequel.df_hash("a");

    assert_ne!(hash1, hash2); // OK!
}



// Test if the dt_hash returns the same HASH even that input is equal
#[test]
fn test_dt_hash_is_equal() {
    let mut tequel = Tequel::new();

    let hash1 = tequel.dt_hash("a");
    let hash2 = tequel.dt_hash("a");

    assert_eq!(hash1, hash2); // OK!
}



// Test if HASH is valid with the same SECRET_INPUT
#[test]
fn test_if_hash_with_salt_is_valid() {

    let mut tequel = Tequel::new();

    let my_secret = "secret";
    let hash = tequel.slgen_hash(&my_secret);

    assert!(tequel.is_valid_sl_hash(&my_secret, &hash)) // OK!
}



#[test]
fn test_encrypt() {

    let mut tequel = Tequel::new();

    let my_password = "gx_pass202@!";
    let my_secret_key = "12345";

    let pass_encrypted: TequelEncryption = tequel.teq_encrypt(&my_password, &my_secret_key);

    println!("hash: {}", pass_encrypted.data);
    println!("mac: {}", pass_encrypted.salt);
    println!("salt: {}", pass_encrypted.mac);

    // OK!

}

