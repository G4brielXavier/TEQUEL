use tequel_rs::{ TequelSHash, Tequel };


fn main() {
    let mut tequel = Tequel::new();

    let my_id: String = tequel.rand_mini();

    println!("{}", my_id);
}