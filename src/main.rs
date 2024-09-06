mod lib;
use lib::array_strings::{
    chosen_one_dcode, death_note_dcode, master_magic_dcode, paint_wall_dcode, rnd_file_dcode,
};

fn main() {
    let res = death_note_dcode(4, 2);
    println!("{:?}", res);
}
