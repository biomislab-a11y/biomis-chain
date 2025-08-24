use rocksdb::DB;

pub fn init_ledger() {
    let db = DB::open_default("ledger_db").unwrap();
    println!("Ledger initialized: {:?}", db.path());
}
