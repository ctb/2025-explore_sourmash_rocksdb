use std::fs::File;
use std::io::Write;
use byteorder::ReadBytesExt;
use byteorder::LittleEndian;
use clap::Parser;
use std::path::PathBuf;

use rocksdb::IteratorMode;

use sourmash::index::revindex::{ RevIndex, Datasets };
use sourmash::sketch::minhash::{
    max_hash_for_scaled,
};
use sourmash::ScaledType;
// use sourmash::index::revindex::disk_revindex::{ RevIndex };

pub const HASHES: &str = "hashes";

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    dbpath: PathBuf,

    #[arg(short, long)]
    outpath: PathBuf,

    #[arg(short, long, default_value_t=1000)]
    scaled: ScaledType,
}

fn main() {
    let cli = Args::parse();

    let revindex = RevIndex::open(cli.dbpath, true, None).ok().expect("foo");

    let revindex = match revindex {
        RevIndex::Plain(db) => db,
    };

    let db = revindex.db;
    let cf_handle = db.cf_handle(HASHES).expect("foo");

    let mut data_file = File::create(cli.outpath).expect("creation failed");

    let max_hash = max_hash_for_scaled(cli.scaled);

    let iter = db.iterator_cf(&cf_handle, IteratorMode::Start); // Always iterates forward
    for item in iter {
        let (key, value) = item.unwrap();

        let k = (&key[..]).read_u64::<LittleEndian>().unwrap();

        if k > max_hash {
            continue;
        }

        //println!("Saw k={:?} k2={:?} v={:?}", key, k, value);
        let v = Datasets::from_slice(&value).expect("Error with value");
        writeln!(data_file, "{} {}", k, v.len()).ok();
    }
}
