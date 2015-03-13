// We use in place quick sort
// For details see http://en.wikipedia.org/wiki/Quicksort#In-place_version
fn quick_sort<T: Ord>(v: &mut[T]) {
    let len = v.len();
    if len < 2 {
        return;
    }
 
    let pivot_index = partition(v);
 
    // Sort the left side
    quick_sort(v.mut_slice(0, pivot_index));
 
    // Sort the right side
    quick_sort(v.mut_slice(pivot_index + 1, len));
}
 
// Reorders the slice with values lower than the pivot at the left side,
// and values bigger than it at the right side.
// Also returns the store index.
fn partition<T: Ord>(v: &mut [T]) -> uint {
    let len = v.len();
    let pivot_index = len / 2;
 
    v.swap(pivot_index, len - 1);
 
    let mut store_index = 0;
    for i in range(0, len - 1) {
        if v[i] <= v[len - 1] {
            v.swap(i, store_index);
            store_index += 1;
        }
    }
 
    v.swap(store_index, len - 1);
    store_index
}
 
fn main() {
    // Sort numbers
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {}", numbers.as_slice());
 
    quick_sort(numbers);
    println!("After: {}", numbers.as_slice());
 
    // Sort strings
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {}", strings.as_slice());
 
    quick_sort(strings);
    println!("After: {}", strings.as_slice());
}
