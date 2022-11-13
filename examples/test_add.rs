use std::time::{Duration, Instant};

use array_list::{IArray, single_array::SingleArray, vector_array::VectorArray, Wrapper, array::Array, matrix_array::MatrixArray};

fn main() {
    run_test("Standard Vec", Wrapper::new);
    run_test("Single array",SingleArray::new);
    run_test("Vector array (10)", || VectorArray::new(10));
    run_test("Vector array (100)", || VectorArray::new(10));
    run_test("Factor array (aka Array)", Array::new);
    run_test("Matrix array(10)", || MatrixArray::new(10));
    run_test("Matrix array (100)", || MatrixArray::new(100));
}

fn run_test<F, Array>(title: &str, create: F) 
where F: Fn() -> Array,
Array: IArray<i64> {
    for i in 1..9 {
        let mut a = create();
        let n = 10_usize.pow(i);
        println!("{title}: n = {n}, complete in {:?}", test_add_integers(&mut a, n));
    }
}

fn test_add_integers(array: &mut dyn IArray<i64>, n: usize) -> Duration {
    let start = Instant::now();
    for i in 0..n {
        array.push(i as i64);
    }
    Instant::now().duration_since(start)
}