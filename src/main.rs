#![feature(duration_as_u128)]

extern crate r2d2;
extern crate r2d2_odbc;
extern crate odbc;

use std::thread;
use std::time::{SystemTime};
use std::sync::{Arc, Mutex};
use std::env;
use std::collections::HashMap;

use r2d2_odbc::ODBCConnectionManager;

use odbc::*;

fn main() {
    let dsn = env::var("DSN").unwrap();
    let query = env::var("QUERY").unwrap();

    let manager = ODBCConnectionManager::new(dsn);

    let pool = r2d2::Pool::builder()
            .max_size(32)
            .build(manager)
            .unwrap();

    let counter = Arc::new(Mutex::new(0));

    let mut children = vec![];

    for _ in 0..16i32 {
        let pool = pool.clone();
        let query = query.clone();

        let counter = Arc::clone(&counter);

        children.push(thread::spawn(move || {
            let now = SystemTime::now();

            loop {
                let pool_conn = pool.get().unwrap();
                let conn = pool_conn.raw();
                let stmt = Statement::with_parent(conn).unwrap();

                let mut vec = Vec::new();

                match stmt.exec_direct(query.as_str()).unwrap() {
                    Data(mut stmt) => {
                        let cols = stmt.num_result_cols().unwrap();

                        let mut row = HashMap::new();

                        while let Some(mut cursor) = stmt.fetch().unwrap() {
                            for i in 1..(cols + 1) {
                                row.insert(i.to_string(), cursor.get_data::<String>(i as u16).unwrap());
                            }

                            vec.push(row.to_owned());
                        }
                    },
                    NoData(_) => println!("Query executed, no data returned"),
                }

                let mut num = counter.lock().unwrap();

                *num += 1;

                if *num % 100 == 0 {
                    let elapsed = now.elapsed().unwrap().as_millis();

                    let iterations_per_second = (*num as f64 / elapsed as f64) * 1000.0 as f64;

                    println!("{} {}", *num, iterations_per_second);
                }
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
