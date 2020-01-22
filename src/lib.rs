#![feature(stmt_expr_attributes)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(rustc_private)]

#[macro_use]
extern crate bitflags;

#[macro_use(defer)] 
extern crate scopeguard;

mod core;
mod x86;