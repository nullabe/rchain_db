use rchain_db::db::blockchain::Blockchain;

fn main() {
    let blockchain = Blockchain::new();

    println!("{:?}", blockchain);
}
