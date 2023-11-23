pub fn custom_insertion_sort(arr: &mut[i32]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
    println!("{:?}", arr);
}
