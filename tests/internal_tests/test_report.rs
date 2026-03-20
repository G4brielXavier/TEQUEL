// use tequel_rs::hash::TequelHash;
use std::num::ParseIntError;


pub fn get_entropy(data: &[u8]) -> f64 {
    let mut frequen = [0usize; 256];
    let len = data.len() as f64;

    // 1. Count how many times each byte appear
    for &byte in data {
        frequen[byte as usize] += 1;
    }

    // 2. Apply Shannon formula: H = -sum(P(i) * log2(P(i)))
    let mut entropy = 0.0;
    for &freq in frequen.iter() {
        if freq > 0 {
            let p = freq as f64 / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

pub fn test_entropy() {
    let mut teq = TequelHash::new();
    let mut mar_de_dados = Vec::new();

    println!("Generating 10MB encrypted data...");

    for i in 0..240000 { // Gerando aprox. 10MB de hashes
        let input = format!("test_seed{}", i);
        let hash_hex = teq.dt_hash_string(&input);
        
        // Convertemos o hex de volta para bytes para medir a entropia real
        if let Ok(bytes) = decode_hex(&hash_hex) {
            mar_de_dados.extend(bytes);
        }
    }

    let resultado = get_entropy(&mar_de_dados);
    
    println!("--- Tequel Entropy Report ---");
    println!("Total analyzed bytes: {}", mar_de_dados.len());
    println!("Entropy: {:.6} bits by byte", resultado);
    
    if resultado > 7.9 {
        println!("Result: EXCELLENT (Quase caos perfeito)");
    } else if resultado > 7.5 {
        println!("Result: GOOD (Mas pode haver padrões sutis)");
    } else {
        println!("Result: WEAK (O algoritmo está enviesado)");
    }
}



fn decode_hex(val: &str) -> Result<Vec<u8>, ParseIntError> {
    if val.len() % 2 != 0 {
        return Err("Hex string has an odd length".parse::<u8>().unwrap_err()); 
    }

    (0..val.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&val[i..i + 2], 16))
        .collect()
}