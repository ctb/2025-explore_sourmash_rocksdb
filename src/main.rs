use rocksdb::{ColumnFamilyDescriptor, Options};
use rocksdb::{DB, Direction, IteratorMode};
use rocksdb::MergeOperands;

use sourmash::encodings::{Color, Colors, Idx};
use sourmash::index::revindex::RevIndex;
// use sourmash::index::revindex::disk_revindex::{ RevIndex };

pub const HASHES: &str = "hashes";

fn main() {
    let revindex = RevIndex::open("tst.rocksdb", true, None).ok().expect("foo");

    let revindex = match revindex {
        RevIndex::Plain(db) => db,
    };

    let db = revindex.db;
    let cf_handle = db.cf_handle(HASHES).expect("foo");

    let mut iter = db.iterator_cf(&cf_handle, IteratorMode::Start); // Always iterates forward
    for item in iter {
        let (key, value) = item.unwrap();
        println!("Saw {:?} {:?}", key, value);
    }
    println!("Hello, world!");
}
