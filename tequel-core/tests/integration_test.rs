use std::collections::HashSet;
use tequel::hash::TequelHash;
use std::time::Instant;

#[test]
fn test_dif_hash_is_equal_from_bytes() {

    let mut teqhash = TequelHash::new();

    let hash1 = teqhash.tqlhash(b"dog");
    let hash2 = teqhash.tqlhash(b"dog");


    assert_eq!(hash1, hash2);

}

#[test]
fn test_if_hash_from_bytes_with_salt_is_valid() {

    let mut teq_hash = TequelHash::new()
        .with_salt("test")
        .with_iteration(50);

    let my_secret = b"secret";
    let hash = teq_hash.tqlhash(my_secret);

    assert!(teq_hash.isv_tqlhash(&hash, my_secret));

}


#[test]
fn test_collision_resistance_optimized() {
    let iterations = 1_000_000;
    let mut seen_hashes = HashSet::with_capacity(iterations);
    let mut collisions = 0;
    let mut hasher = TequelHash::new();
    
    let mut buffer = String::with_capacity(64);
    
    println!("Starting colision test: {} iterations", iterations);
    let start = Instant::now();

    for i in 0..iterations {
        buffer.clear();

        use std::fmt::Write;
        write!(&mut buffer, "payload_id_{}", i).unwrap();
        
        let hash = hasher.tqlhash(buffer.as_bytes());


        if !seen_hashes.insert(hash.clone()) {
            collisions += 1;
            println!("💥 Colision found in index {}: {}", i, hash);
        }

        if i % 10_000 == 0 && i > 0 {
            println!("{}% ...", (i as f32 / iterations as f32) * 100.0);
        }
    }

    let duration = start.elapsed();
    println!("- Test Finish!");
    println!("Total: {} iterations", iterations);
    println!("Colisions: {}", collisions);
    println!("Total time: {:.2?}", duration);
    println!("Speed: {:.2} hash/s", iterations as f64 / duration.as_secs_f64());

    assert_eq!(collisions, 0, "Tequel failed in colision test!");
}



#[test]
fn test_tequel_avalanche_string_output() {
    let mut hasher = TequelHash::new();
    let input = b"payload_id_777_supply_chain_data_2026";
    
    let base_hash_hex = hasher.tqlhash(input);
    let base_bytes = hex::decode(&base_hash_hex).expect("Hash base deve ser um hexa válido");
    
    let mut total_bit_flips = 0;
    let total_bits_input = input.len() * 8;

    for byte_idx in 0..input.len() {
        for bit_idx in 0..8 {
            let mut modified_input = input.to_vec();
            modified_input[byte_idx] ^= 1 << bit_idx;
            
            let new_hash_hex = hasher.tqlhash(&modified_input);
            let new_bytes = hex::decode(&new_hash_hex).expect("Novo hash deve ser um hexa válido");

            for (b1, b2) in base_bytes.iter().zip(new_bytes.iter()) {
                let diff = b1 ^ b2;
                total_bit_flips += diff.count_ones();
            }
        }
    }

    let hash_bits_output = base_bytes.len() * 8; 
    let avalanche_score = (total_bit_flips as f64 / (total_bits_input * hash_bits_output) as f64) * 100.0;

    println!("Total bits: {}", total_bits_input * hash_bits_output);
    println!("Avalanche: {:.2}%", avalanche_score);

    assert!(avalanche_score > 40.0 && avalanche_score < 60.0);
}
