/*
Structures.rs
Crate file for the library
*/

#![feature(managed_boxes, phase)]
#[link(name = "structures", vers = "0.1", author = "pdn")]
extern crate quickcheck;
#[phase(plugin)]
extern crate quickcheck_macros;



pub mod trees
{
     mod bst;
//     mod llrb;
//     mod dbl_linked;
}

mod linear
{

   mod circular;
    mod list;
}
