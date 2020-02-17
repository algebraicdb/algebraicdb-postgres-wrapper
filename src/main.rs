#![feature(str_strip)]
#![feature(never_type)]
#![feature(box_syntax)]
// TODO: remove this once we actually start using our code
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod ast;
mod global;
mod grammar;
mod pattern;
mod pre_typechecker;
mod table;
mod typechecker;
mod types;

fn main() {
    unimplemented!("main")
}
