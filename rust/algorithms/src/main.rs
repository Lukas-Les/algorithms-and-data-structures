use algorithms::bubble_sort;
//use algorithms::selection_sort;
//use algorithms::custom_insertion_sort;
//use algorithms::lab;

fn main() {
    let mut numbers = vec![9, 3, 6, 9, 4, 2, 0, 1, 2, 9, 8, 7, 5];
    //    bubble_sort::bubble_sort(&mut numbers);
    //    selection_sort::selection_sort(&mut numbers);
    bubble_sort::optimised_bubble_sort(&mut numbers);
}
