use std::time::Instant;

const CAPACITY: usize = 500;

fn main() {
    let now = Instant::now();
    let array = vec![0; CAPACITY].into_boxed_slice();

    println!("The array sum is {num}", num = array_sum(array));
    println!(" time:{:?}", now.elapsed());
}

fn array_sum(mut array: std::boxed::Box<[usize]>) -> usize {
    array[1] = array[0] + 1;

    for i in 0..CAPACITY {
        array[i] = i;
        println!("value is {}", array[i]);
    }

    // get sum of  all values inside array
    let total_sum = array.iter().sum();
    return total_sum;
}

#[test]
fn test_sum_array() {
    let array = vec![0; CAPACITY].into_boxed_slice();
    assert_eq!(array_sum(array), 124750);
}
