extern crate scrabble;
use std::time::Instant;
fn main(){
  let start = Instant::now();
    scrabble::load_dawg();
    let duration = start.elapsed();
    println!("Time elapsed in load_dawg() is: {:?}", duration);
}