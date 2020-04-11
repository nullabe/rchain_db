use rchain_db::model::blockchain::Blockchain;

fn main() {
    let blockchain = Blockchain::new();

    println!("{:?}", blockchain);
}
