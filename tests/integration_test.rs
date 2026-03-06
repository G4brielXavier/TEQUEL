use tequel_rs::Tequel;

#[test]
fn test_dt_hash_size_return() {
    let mut tequel = Tequel::new();

    assert_eq!(tequel.dt_hash("a").len(), 80);
}