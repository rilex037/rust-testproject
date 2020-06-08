/*
* TEST PROJECT
*/

mod array;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let array = vec![0; array::CAPACITY].into_boxed_slice();

    println!("The array sum is {num}", num = array::sum(array));
    println!(" time:{:?}", now.elapsed());
}
