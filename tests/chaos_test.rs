#[cfg(test)]
mod tests {
    use tequel::hash::TequelHash; // Ajuste para o nome real da sua struct/função

    // Função auxiliar para contar a diferença de bits entre dois hashes
    fn bit_diff(h1: &[u8], h2: &[u8]) -> f64 {
        let mut diff_bits = 0;
        let total_bits = h1.len() * 8;
        
        for (b1, b2) in h1.iter().zip(h2.iter()) {
            diff_bits += (b1 ^ b2).count_ones(); // XOR e conta quantos bits são 1
        }
        
        (diff_bits as f64 / total_bits as f64) * 100.0
    }

    #[test]
    fn test_capeta_avalanche() {
        let input1 = "A".repeat(1000);
        let mut input2 = "A".repeat(1000);
        input2.replace_range(999..1000, "B");

        let h1_hex = TequelHash::new().tqlhash(input1.as_bytes());
        let h2_hex = TequelHash::new().tqlhash(input2.as_bytes());

        // CONVERSÃO CRÍTICA: Transforma Hex String em Vec<u8> real
        let h1_bytes = hex::decode(&h1_hex).expect("Hex inválido 1");
        let h2_bytes = hex::decode(&h2_hex).expect("Hex inválido 2");

        let diff = bit_diff(&h1_bytes, &h2_bytes);
        
        println!("Avalanche Real: {:.2}%", diff);
        assert!(diff > 45.0 && diff < 55.0, "Agora sim o teste é justo! Diff: {:.2}%", diff);
    }

    #[test]
    fn test_capeta_null_vector() {
        // O "Deserto de Zeros": Tudo zero, inclusive o Salt
        let null_input = vec![0u8; 1024];
        let salt = "0".repeat(16);
        
        let hash = TequelHash::new().with_salt(&salt).tqlhash(&null_input);
        
        // Verifica se o hash não é uma sequência repetida ou cheia de zeros
        let zero_count = hash.as_bytes().iter().filter(|&&b| b == 0).count();
        
        println!("Zero bytes in null hash: {}/{}", zero_count, hash.len());
        
        // Se mais de 20% do hash for zero em um input nulo, a matemática tá fraca
        assert!(zero_count < (hash.len() / 5), "O Tequel se entregou ao vazio!");
    
    }
}