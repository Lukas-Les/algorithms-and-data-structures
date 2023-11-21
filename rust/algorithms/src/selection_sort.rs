pub fn selection_sort(arr: &mut[i32]) {
    let len = arr.len();
    for i in 0..len - 1 {
        let mut min = i;
        for j in i + 1..len {
            if arr[min] > arr[j] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
    println!("{:?}", arr);
}
