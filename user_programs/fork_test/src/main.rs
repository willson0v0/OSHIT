#![no_std]
#![no_main]

#[macro_use]
extern crate oshit_usrlib;

use oshit_usrlib::{sys_fork, wait, sys_exit};

const MAX_CHILD: usize = 40;

#[no_mangle]
pub fn main() -> i32 {
    for i in 0..MAX_CHILD {
        let pid = sys_fork();
        if pid == 0 {
            println!("I am child {}", i);
            sys_exit(0);
        } else {
            println!("forked child pid = {}", pid);
        }
        assert!(pid > 0);
    }
    let mut exit_code: i32 = 0;
    for _ in 0..MAX_CHILD {
        if wait(&mut exit_code) <= 0 {
            panic!("wait stopped early");
        }
    }
    if wait(&mut exit_code) > 0 {
        panic!("wait got too many");
    }
    println!("forktest pass.");
    0
}