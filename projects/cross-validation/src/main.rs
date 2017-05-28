extern crate distance;
extern crate database;

use database::{AthlDatabase, MpgDatabase};

fn main() {
    AthlDatabase::test("../../data/athletesTrainingSet.txt", "../../data/athletesTestSet.txt");
    MpgDatabase::test("../../data/mpgTrainingSet.txt", "../../data/mpgTestSet.txt");

    // println!("Pred => {:?}", db.predict(70.0, 170.0));
    // println!("Pred => {:?}", db.predict(8.0, 360.0, 215.0, 4615.0, 14.0));
}
