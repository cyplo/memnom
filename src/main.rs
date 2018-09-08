extern crate core;
extern crate sysinfo;
extern crate circular_queue;

use std::alloc::alloc;
use core::alloc::Layout;
use std::alloc::dealloc;
use std::env;
use sysinfo::{System, SystemExt};
use circular_queue::CircularQueue;
use std::thread::sleep;
use std::time::Duration;

const CHUNK_SIZE : usize = 16 * 1024;

fn main() {
    print_stats();
    println!("Allocating memory until we start swapping");
    let mut storage = vec![];

    push_until_swapping(&mut storage);

    const wait_time: u64 = 16;
    println!("sustaining memory load for the next {} seconds", wait_time);
    sleep(Duration::from_secs(wait_time));
}

fn push_until_swapping(storage: &mut Vec<[i8; CHUNK_SIZE]>) {
    let mut sys = System::new();
    let chunk: [i8; CHUNK_SIZE] = [-1; CHUNK_SIZE];
    let check_every_bytes = 128*1024*1024;
    let check_every_chunks = check_every_bytes / CHUNK_SIZE;
    let mut counter: usize = 0;
    const swap_history_size : usize = 16;
    let mut free_swap_history = CircularQueue::with_capacity(swap_history_size);
    loop {
        storage.push(chunk);
        counter = counter + 1;
        if counter % check_every_chunks == 0 {
            sys.refresh_system();
            free_swap_history.push(sys.get_free_swap());
            if free_swap_history.len() < swap_history_size {
                continue;
            }
            let mut previous = std::u64::MAX;
            let mut decreasing = true;
            for free_swap in free_swap_history.iter() {
                if free_swap > &previous {
                    decreasing = false;
                    break;
                }
                previous = *free_swap;
            }
            if decreasing {
                break;
            }
        }
    }
    print_stats();
}

fn print_stats() {
    let sys = System::new();
    println!();
    println!("Total amount of RAM : {} kB", sys.get_total_memory());
    println!("Used RAM: {} kB", sys.get_used_memory());
    println!("Free RAM: {} kB", sys.get_free_memory());
    println!("Total amount of swap: {} kB", sys.get_total_swap());
    println!("Used swap: {} kB", sys.get_used_swap());
    println!("Free swap: {} kB", sys.get_free_swap());
}
