extern crate csv;
extern crate test;
extern crate rustc_serialize;

mod distance;

use std::fs::File;
use csv::Reader;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use distance::{manhattan_dist, pearson_coef, cosine_dist};
use test::Bencher;

#[derive(RustcDecodable)]
struct User {
    id: u32,
    location: String,
    age: u32,
}

#[derive(RustcDecodable)]
struct Book {
    isbn: String,
    title: String,
    author: String,
    pub_year: u32,
    publisher: String,
}

#[derive(RustcDecodable)]
struct Rating {
    user_id: u32,
    book_isbn: String,
    rating: u32,
}

fn distance(db: &mut HashMap<u32, HashMap<String, u32>>,
            a: u32,
            b: u32,
            func: fn(&Vec<f32>, &Vec<f32>) -> f32)
            -> f32 {
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();

    match db.get(&a) {
        Some(mut a_ratings) => {
            for (isbn_a, &rating_a) in a_ratings.iter() {
                // print!("{} ", rating_a);
                if let Some(b_ratings) = db.get(&b) {
                    if let Some(rating_b) = b_ratings.get(isbn_a) {
                        // print!("{} ", val_b);
                        vec_a.push(rating_a as f32);
                        vec_b.push(*rating_b as f32);
                    } else {
                        vec_a.push(rating_a as f32);
                        vec_b.push(0.0);
                    }
                } else {
                    panic!("b is not found!");
                }
            }
        }
        None => {}
    }

    match db.get(&b) {
        Some(mut b_ratings) => {
            for (isbn_b, &rating_b) in b_ratings.iter() {
                if let Some(a_ratings) = db.get(&a) {
                    if let Some(_) = a_ratings.get(isbn_b) {
                    } else {
                        vec_b.push(rating_b as f32);
                        vec_a.push(0.0);
                    }
                } else {
                    panic!("a is not found!");
                }
            }
        }
        None => {}
    }

    func(&vec_a, &vec_b)
}

fn init_db_reader(path: &str) -> Reader<File> {
    let rdr = csv::Reader::from_file(path).unwrap();
    rdr
}

fn load_book_ratings() {
    /*********************LOAD RATINGS*********************/
    let mut rdr = init_db_reader("./data/BX-Dump/BX-Book-Ratings.csv").has_headers(false);
    let mut ratings: HashMap<u32, HashMap<String, u32>> = HashMap::new();

    // let mut i = 0;
    for record in rdr.decode() {
        // if i > 54000 {
        //     break;
        // }
        // i += 1;
        let rating: Rating = record.unwrap();

        match ratings.entry(rating.user_id) {
            Vacant(entry) => {
                let mut user_ratings = HashMap::new();
                user_ratings.insert(rating.book_isbn, rating.rating);
                entry.insert(user_ratings);
            }
            Occupied(entry) => {
                entry.into_mut().insert(rating.book_isbn, rating.rating);
            }
        }
    }
    
    println!("{}", distance(&mut ratings, 11676, 278418, cosine_dist));
    println!("{}", distance(&mut ratings, 11676, 278418, manhattan_dist));
    println!("{}", distance(&mut ratings, 11676, 278418, pearson_coef));

    /*********************LOAD USERS*********************/
    /****************************************************/
}

fn main() {
    load_book_ratings();
    println!("Hello, world!");
}
