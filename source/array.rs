pub const CAPACITY: usize = 5000000;

///
/// Get array sum of all values
///
pub fn sum(mut array: std::boxed::Box<[usize]>) -> usize {
    array[1] = array[0] + 1;
    for i in 0..CAPACITY {
        array[i] = i;
        // println!("value is {}", array[i]);
    }

    // get sum of  all values inside array
    let total_sum = array.iter().sum();
    return total_sum;
}

#[test]
fn test_sum_array() {
    let array = vec![0; CAPACITY].into_boxed_slice();
    assert_eq!(sum(array), 124750);
}
