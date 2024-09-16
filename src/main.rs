mod lib;
use lib::array_strings::{
    chosen_one_dcode, death_note_dcode, master_magic_dcode, paint_wall_dcode, rnd_file_dcode,
};
use std::collections::BinaryHeap;

fn main() {
    let arr = vec![0, 3, 62, 1, 7, 9, 2, 3, 67];
    let heap = BinaryHeap::from_iter(arr.into_iter());
    println!("{:?}", heap)
}
