#![feature(test)]
extern crate test;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate rand;
extern crate tempdir;
extern crate lmdb;
extern crate serde;
extern crate serde_json;

use test::Bencher;
use rand::{Rng, SeedableRng, StdRng};
use lmdb::RwTransaction;
use lmdb::Transaction;
use lmdb::WriteFlags;


#[bench]
fn benchmark_aa_empty(b: &mut Bencher) {
    b.iter(|| 1)
}


#[derive(Serialize)]
struct Datastructure {
    ts: i64,
    n: u64,
}


fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4];
}

#[bench]
fn benchmark_only_preparation_forrandom_writes(b: &mut Bencher) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    b.iter(|| {
        let dir = tempdir::TempDir::new("temptestdir").unwrap();
        let db_flags: lmdb::DatabaseFlags = lmdb::DatabaseFlags::empty();
        let db_environment = lmdb::Environment::new().set_max_dbs(1).open(dir.as_ref()).unwrap();
        let database = db_environment.create_db(None, db_flags).unwrap();

        let mut counter: u64 = 0;

        for iteration in 0..1000 {
            let timespec = time::get_time();
            let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
            let random_number: u64 = rng.next_u64();
            let datastruct = Datastructure { ts: mills, n: random_number };

            let key = transform_u32_to_array_of_u8(iteration);
            let data = serde_json::to_string(&datastruct).unwrap();
            counter = counter + 4u64 + (data.len() as u64);
        }

        //println!("Counter={}", counter);
    })
}

#[bench]
fn benchmark_only_preparation_without_database(b: &mut Bencher) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    b.iter(|| {
        //let dir = tempdir::TempDir::new("temptestdir").unwrap();
        //let db_flags: lmdb::DatabaseFlags = lmdb::DatabaseFlags::empty();
        //let db_environment = lmdb::Environment::new().set_max_dbs(1).open(dir.as_ref()).unwrap();
        //let database = db_environment.create_db(None, db_flags).unwrap();

        let mut counter: u64 = 0;

        for iteration in 0..1000 {
            let timespec = time::get_time();
            let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
            let random_number: u64 = rng.next_u64();
            let datastruct = Datastructure { ts: mills, n: random_number };

            let key = transform_u32_to_array_of_u8(iteration);
            let data = serde_json::to_string(&datastruct).unwrap();
            counter = counter + 4u64 + (data.len() as u64);
        }

        //println!("Counter={}", counter);
    })
}

#[bench]
fn benchmark_only_preparation_with_dir(b: &mut Bencher) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    b.iter(|| {
        let dir = tempdir::TempDir::new("temptestdir").unwrap();
        //let db_flags: lmdb::DatabaseFlags = lmdb::DatabaseFlags::empty();
        //let db_environment = lmdb::Environment::new().set_max_dbs(1).open(dir.as_ref()).unwrap();
        //let database = db_environment.create_db(None, db_flags).unwrap();

        let mut counter: u64 = dir.as_ref().to_str().unwrap().len() as u64;

        for iteration in 0..1000 {
            let timespec = time::get_time();
            let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
            let random_number: u64 = rng.next_u64();
            let datastruct = Datastructure { ts: mills, n: random_number };

            let key = transform_u32_to_array_of_u8(iteration);
            let data = serde_json::to_string(&datastruct).unwrap();
            counter = counter + 4u64 + (data.len() as u64);
        }

        //println!("Counter={}", counter);
    })
}
#[bench]
fn benchmark_only_preparation_with_dir_and_env(b: &mut Bencher) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    b.iter(|| {
        let dir = tempdir::TempDir::new("temptestdir").unwrap();
        let db_flags: lmdb::DatabaseFlags = lmdb::DatabaseFlags::empty();
        let db_environment = lmdb::Environment::new().set_max_dbs(1).open(dir.as_ref()).unwrap();
        //let database = db_environment.create_db(None, db_flags).unwrap();

        let mut counter: u64 = dir.as_ref().to_str().unwrap().len() as u64;

        for iteration in 0..1000 {
            let timespec = time::get_time();
            let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
            let random_number: u64 = rng.next_u64();
            let datastruct = Datastructure { ts: mills, n: random_number };

            let key = transform_u32_to_array_of_u8(iteration);
            let data = serde_json::to_string(&datastruct).unwrap();
            counter = counter + 4u64 + (data.len() as u64);
        }

        //println!("Counter={}", counter);
    })
}


#[bench]
fn benchmark_add_time(b: &mut Bencher) {
    b.iter(||{
        let mut counter = 0i64;
        for _ in 0..1000 {

            let timespec = time::get_time();
            let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;

            counter += mills;

        }
        counter
    })
}



#[bench]
fn benchmark_random_writes(b: &mut Bencher) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    b.iter(|| {
        let dir = tempdir::TempDir::new("temptestdir").unwrap();
        let db_flags: lmdb::DatabaseFlags = lmdb::DatabaseFlags::empty();
        let db_environment = lmdb::Environment::new().set_max_dbs(1).open(dir.as_ref()).unwrap();
        let database = db_environment.create_db(None, db_flags).unwrap();

        for iteration in 0..100 {
            let timespec = time::get_time();
            let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
            let random_number: u64 = rng.next_u64();
            let datastruct = Datastructure { ts: mills, n: random_number };

            {
                let mut rw_transaction: RwTransaction = RwTransaction::new(&db_environment).unwrap();
                let tx_flags: WriteFlags = WriteFlags::empty();
                let key = transform_u32_to_array_of_u8(iteration);
                let data = serde_json::to_string(&datastruct).unwrap();
                let _ = rw_transaction.put(database, &key, &data, tx_flags);
                rw_transaction.commit().unwrap();
            }
        }
    })
}



#[bench]
fn benchmark_random_writes_setup_only(b: &mut Bencher) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    b.iter(|| {
        let dir = tempdir::TempDir::new("temptestdir").unwrap();
        let db_flags: lmdb::DatabaseFlags = lmdb::DatabaseFlags::empty();
        let db_environment = lmdb::Environment::new().set_max_dbs(1).open(dir.as_ref()).unwrap();
        let database = db_environment.create_db(None, db_flags).unwrap();

        for iteration in 0..1 {
            let timespec = time::get_time();
            let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
            let random_number: u64 = rng.next_u64();
            let datastruct = Datastructure { ts: mills, n: random_number };

            {
                let mut rw_transaction: RwTransaction = RwTransaction::new(&db_environment).unwrap();
                let tx_flags: WriteFlags = WriteFlags::empty();
                let key = transform_u32_to_array_of_u8(iteration);
                let data = serde_json::to_string(&datastruct).unwrap();
                let _ = rw_transaction.put(database, &key, &data, tx_flags);
                rw_transaction.commit().unwrap();
            }
        }
    })
}

