#![allow(dead_code)]
#![allow(unused_variables)]

use cpu::{Cpu};

macro_rules! mask_out_above_8 {
    ($e:expr) => ($e & 0xff)
}
macro_rules! mask_out_below_8 {
    ($e:expr) => ($e & !0xff)
}
macro_rules! mask_out_above_16 {
    ($e:expr) => ($e & 0xffff)
}
macro_rules! mask_out_below_16 {
    ($e:expr) => ($e & !0xffff)
}
macro_rules! mask_out_above_32 {
    ($e:expr) => ($e & 0xffffffff)
}
macro_rules! low_nibble {
    ($e:expr) => ($e & 0x0f);
}
macro_rules! high_nibble {
    ($e:expr) => ($e & 0xf0);
}

pub fn add_16(cpu: &mut Cpu, dst: u32, src: u32) -> u32 {
    let res = mask_out_above_16!(dst) + mask_out_above_16!(src);

    // set flags
    cpu.set_flags_16(res, src as u16, dst as u16);

    mask_out_above_16!(res)
}