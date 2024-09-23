mod lib;
use lib::array_strings::{
    chosen_one_dcode, death_note_dcode, master_magic_dcode, paint_wall_dcode, rnd_file_dcode,
};
use lib::recursive_backtracking::{combine, generate_parenthesis};
use std::collections::BinaryHeap;

fn main() {
    println!("{:?}", generate_parenthesis(8));
}
