pub fn main(arr: &mut[i32]) {
    let len = arr.len();
    let mut i_loop = 0;
    let mut j_loop = 0;
    for i in 0..len {
        let mut swaped = false;
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swaped = true;
            }
            j_loop += 1;
        }
        if !swaped {
            break;
        }
        i_loop += 1;
    }
    println!("i_loop: {i_loop}");
    println!("j_loop: {j_loop}");
    println!("{:?}", arr);
}
