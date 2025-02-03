use std::fs::File;
use std::io::Write;
use byteorder::ReadBytesExt;
use byteorder::LittleEndian;
use clap::Parser;
use std::path::PathBuf;

use rocksdb::IteratorMode;

use sourmash::index::revindex::RevIndex;
// use sourmash::index::revindex::disk_revindex::{ RevIndex };

pub const HASHES: &str = "hashes";

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    dbpath: PathBuf,

    /// Number of times to greet
    #[arg(short, long)]
    outpath: PathBuf,
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

    let iter = db.iterator_cf(&cf_handle, IteratorMode::Start); // Always iterates forward
    for item in iter {
        let (key, value) = item.unwrap();

        let k = (&key[..]).read_u64::<LittleEndian>().unwrap();
        //println!("Saw k={:?} k2={:?} v={:?}", key, k, value);
        writeln!(data_file, "{}", k).ok();
        println!("{:?}", value);
    }
}
