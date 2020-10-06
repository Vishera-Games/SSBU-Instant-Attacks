#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

//mod ganon;
//mod byleth;
//mod gigabowser;
mod donkey;

#[skyline::main(name = "instant_attacks")]
pub fn main() {
    //ganon::install();
    //byleth::install();
    //gigabowser::install();
    donkey::install();
}