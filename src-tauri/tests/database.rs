use coin_tracker::coin::Coin;
use coin_tracker::database as db;

#[test]
fn database() {
    let connection = db::init(None).expect("Could not init db");
    let coin = Coin::new();
    let new_coin = Coin::example();
    let id = db::insert_coin(&connection, &coin).expect("Could not insert coin to db");
    db::update_coin(&connection, &id, &new_coin).expect("Could not update coin in db");
    let coin2 = db::get_coin_by_id(&connection, &id).expect("Could not get coin from db");
    println!("{coin2:?}");
}
