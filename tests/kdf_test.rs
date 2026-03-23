use tequel_rs::hash::TequelHash;

#[cfg(test)]
mod kdf_tests {
    use super::*;

    #[test]
    fn test_myway_key_derivation() {
        let mut tequel = TequelHash::new()
            .with_salt("project_name");

        let password = "senha_super_secreta_do_gabriel";
        let project_name = "meu_projeto_hova"; // Usado como SALT
        
        // 1. Gera a chave original (1000 iterações para segurança)
        let key1 = tequel.derive_key(password, 1000);
        
        // 2. Gera a chave com uma pequena mudança (troca o 'a' por 'b' no final)
        let password_alt = "senha_super_secreta_do_gabrielb";
        let key2 = tequel.derive_key(password_alt, 1000);

        // O teste da verdade: As chaves DEVEM ser totalmente diferentes
        assert_ne!(key1, key2, "As chaves não podem ser iguais!");

        // Calcula a diferença de bits (Avalanche na chave derivada)
        let mut diff_bits = 0;
        for i in 0..32 {
            diff_bits += (key1[i] ^ key2[i]).count_ones();
        }
        
        let avalanche = (diff_bits as f32 / 256.0) * 100.0;
        println!("--- My Way KDF Report ---");
        println!("Avalanche na Chave: {:.2}%", avalanche);
        
        // Se estiver perto de 50%, seu My Way está blindado
        assert!(avalanche > 40.0 && avalanche < 60.0);
    }
}