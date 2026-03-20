use tequel_rs::hash::TequelHash;
use tequel_rs::encrypt::TequelEncrypt;
use tequel_rs::error::TequelError;
use tequel_rs::rng::TequelRng;

#[test]
fn test_dif_hash_is_different_from_string() {

    let mut teqhash = TequelHash::new();

    let hash1 = teqhash.dif_hash_string("dog");
    let hash2 = teqhash.dif_hash_string("dog");


    assert_ne!(hash1, hash2);

}

#[test]
fn test_dif_hash_is_different_from_bytes() {

    let mut teqhash = TequelHash::new();

    let hash1 = teqhash.dif_hash_bytes(b"dog");
    let hash2 = teqhash.dif_hash_bytes(b"dog");


    assert_ne!(hash1, hash2);

}

#[test]
fn test_dif_hash_is_equal_from_string() {

    let mut teqhash = TequelHash::new();

    let hash1 = teqhash.dt_hash_string("dog");
    let hash2 = teqhash.dt_hash_string("dog");


    assert_eq!(hash1, hash2);

}

#[test]
fn test_dif_hash_is_equal_from_bytes() {

    let mut teqhash = TequelHash::new();

    let hash1 = teqhash.dt_hash_bytes(b"dog");
    let hash2 = teqhash.dt_hash_bytes(b"dog");


    assert_eq!(hash1, hash2);

}

#[test]
fn test_if_hash_from_string_with_salt_is_valid() {

    let mut teq_hash = TequelHash::new()
        .with_salt("test")
        .with_iteration(50);

    let my_secret = "secret";
    let hash = teq_hash.dt_hash_string(&my_secret);

    assert!(teq_hash.is_valid_hash_from_string(&hash, &my_secret));// OK!

}

#[test]
fn test_if_hash_from_bytes_with_salt_is_valid() {

    let mut teq_hash = TequelHash::new()
        .with_salt("test")
        .with_iteration(50);

    let my_secret = b"secret";
    let hash = teq_hash.dt_hash_bytes(my_secret);

    assert!(teq_hash.is_valid_hash_from_bytes(&hash, my_secret));// OK!

}





#[test]
fn test_tequel_encrypt_full_cycle() {

    let mut teq_crypt = TequelEncrypt::new()
        .with_iteration(100)
        .with_salt("my_salt");

    let original_data = "My secret message 123";
    let key = "tequel_key";

    let encrypted = teq_crypt.encrypt(original_data.as_bytes(), key)
        .expect("Failed to encrypt");

    let decrypted = teq_crypt.decrypt(&encrypted, key)
        .expect("Failed to decrypt");

    assert_eq!(original_data, decrypted, "The encrypted data not match with original!");

}


#[test]
fn test_tequel_stress_loop_100() {


    let mut teq_crypt = TequelEncrypt::new()
        .with_iteration(100)
        .with_salt("my_salt");

    let key = "ultra_safe_key_123";

    for i in 0..100 {
        // Create a different string in each lap (ex: "Data_0", "Data_1" ...)
        let original_data = format!("Secret_Number_Message_{}", i);
        
        // 1. Encrypt (using bytes from formatted string)
        let encrypted = teq_crypt.encrypt(original_data.as_bytes(), key)
            .expect(&format!("Failed in encrypt loop {}", i));

        // 2. Decrypt
        let decrypted = teq_crypt.decrypt(&encrypted, key)
            .expect(&format!("Failed in decrypt loop {} - Erro de UTF-8?", i));

        // 3. Validação
        assert_eq!(original_data, decrypted, "Integrity error loop {}", i);
    }
    
    println!("🔥 100/100 Loop test done! Tequel is solid.");
}

#[test]
fn test_tequel_stress_loop_10000() {


    let mut teq_crypt = TequelEncrypt::new()
        .with_iteration(100)
        .with_salt("my_salt");

    let key = "ultra_safe_key_123";

    for i in 0..10000 {
        // Create a different string in each lap (ex: "Data_0", "Data_1" ...)
        let original_data = format!("Secret_{}_Message_💀_#{}", i, i * 2);
        
        // 1. Encrypt (using bytes from formatted string)
        let encrypted = teq_crypt.encrypt(original_data.as_bytes(), key)
            .expect(&format!("Failed in encrypt loop {}", i));

        // 2. Decrypt
        let decrypted = teq_crypt.decrypt(&encrypted, key)
            .expect(&format!("Failed in decrypt loop {} - Erro de UTF-8?", i));

        // 3. Validação
        assert_eq!(original_data, decrypted, "Integrity error loop {}", i);
    }
    
    println!("🔥 10000/10000 Loop test done! Tequel is solid.");
}




#[test]
fn test_tequel_fuzzing_resistance() -> Result<(), Box<dyn std::error::Error>> {

    let mut teq = TequelEncrypt::new();

    let as_big = "A".repeat(10000);

    let crazy_inputs = vec![
        "",                     // Vazio
        " ",                    // Espaço
        "\0\0\0",               // Null bytes
        "💀🚀🔥",               // Emojis (Multi-byte UTF-8)
        &as_big,      // String gigante
        "你好",                 // Mandarim
    ];


    for input in &crazy_inputs {

        let legit_encrypted = teq.encrypt(input.as_bytes(), "").map_err(|e| {
            e
        })?;

        let mut corrupted = legit_encrypted.clone();
        corrupted.mac = "ffffffffffffffffffffffffffffffff".to_string(); // MAC falso
        corrupted.salt = "00000000".to_string(); // Salt resetado
        
        let trash_data = teq.decrypt(&corrupted, "key123");

        assert!(trash_data.is_err(), "O Tequel aceitou um objeto corrompido! Erro de integridade.");
        println!("✅ Fuzzing de Objeto: Tequel stopped corromped structure.");

    }

    Ok(())

}



#[test]
fn test_tequel_key_sensitivity() {

    let mut teq = TequelEncrypt::new().with_salt("security_first");
    let original_data = b"Ultra sensible data";

    let real_key = "StrongKey123";
    let fake_key = "StrongKey100";

    let encrypted = teq.encrypt(original_data, real_key).unwrap();

    let result = teq.decrypt(&encrypted, fake_key);

    match result {
        Ok(_) => panic!("Critical Fail: Tequel accepted a wrong key and generate trash"),
        Err(_) => println!("Integrity Security: Key Wrong Blocked")
    }

}