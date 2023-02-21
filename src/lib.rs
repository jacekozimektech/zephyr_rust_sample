#![no_std]
#![no_main]

#[cfg(target_arch = "arm")]
extern crate panic_halt;

use heapless::Deque;

static mut MY_FIFO: Deque<i32, 10> = Deque::new();

#[no_mangle]
pub extern "C" fn push_to_fifo(value: i32) -> bool
{
    let my_fifo = unsafe { &mut MY_FIFO };
    let ret = my_fifo.push_front(value);
    ret.is_ok()
}

#[no_mangle]
pub extern "C" fn get_fifo_len() -> i32
{
    let my_fifo = unsafe { &mut MY_FIFO };
    let ret = my_fifo.len();
    ret as i32
}

#[no_mangle]
pub extern "C" fn get_fifo_mean() -> f32
{
    let my_fifo = unsafe { &mut MY_FIFO };
    let mut sum: f32 = 0.0;
    for x in my_fifo {
        sum = sum + (*x as f32);
    }
    let mean: f32 = sum / (get_fifo_len() as f32);

    mean
}