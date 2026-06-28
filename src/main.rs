mod lib;
use lib::array_strings::{
    chosen_one_dcode, death_note_dcode, master_magic_dcode, paint_wall_dcode, pascal_tri,
    rnd_file_dcode,
};
use lib::recursive_backtracking::{combine, generate_parenthesis};
use std::collections::BinaryHeap;

fn main() {
    for i in 0..=10 {
        println!("{:?}", pascal_tri(i as u32, vec![1]));
    }
}
