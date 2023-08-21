fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
fn bubble_sort2<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut int_array = [32, 3, 34, 3, 4, 3, 2];
    bubble_sort(&mut int_array);
    println!("Sorted int array: {:?}", int_array);

    let mut float_array = [2.0, 22.5, 23.2, 11.9, 31.7, 12.3, 24.6];
    bubble_sort2(&mut float_array);
    println!("Sorted float array: {:?}", float_array);

    let mut char_array = ['d', 'e', 'h', 'j', 'b', 'r'];
    bubble_sort2(&mut char_array);
    println!("Sorted char array: {:?}", char_array);
}

