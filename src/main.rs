extern crate core;
extern crate sysinfo;

use std::alloc::alloc;
use core::alloc::Layout;
use std::alloc::dealloc;
use std::env;
use sysinfo::{System, SystemExt};

fn main() {
    print_stats();

    println!("Trying to push free RAM under 1GiB");
    push_until(&|sys| sys.get_free_memory() > 1024 * 1024);
    println!("Trying to push free swap under 1GiB");
    push_until(&|sys| sys.get_free_swap() > 1024 * 1024);

}

fn push_until( condition: &Fn(&sysinfo::System) -> bool) {
    let mut sys = System::new();
    let mut storage = vec![];
    let value: [i8; 16 * 1024] = [-1; 16 * 1024];
    let mut counter: i64 = 0;
    while condition(&sys) {
        storage.push(value);
        counter = counter + 1;
        if counter % (1024 * 4) == 0 {
            sys.refresh_system();
        }
    }
    print_stats();
}

fn print_stats() {
    let sys = System::new();
    println!("Total amount of RAM : {} kB", sys.get_total_memory());
    println!("Used RAM: {} kB", sys.get_used_memory());
    println!("Free RAM: {} kB", sys.get_free_memory());
    println!("Total amount of swap: {} kB", sys.get_total_swap());
    println!("Used swap: {} kB", sys.get_used_swap());
    println!("Free swap: {} kB", sys.get_free_swap());
}
