#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use memory::Mem;
use arithmetic;

// opcode function pointer
pub type OpFn = fn(cpu: &mut Cpu, dst: u32, src: u32) -> u32;

pub struct Cpu {
    pub d: [u32; 8], // data registers
    pub a: [u32; 8], // address registers
    pub pc: u32, // program counter

    pub x: bool, // extend
    pub n: bool, // negative
    pub z: bool, // zero
    pub v: bool, // overflow
    pub c: bool, // carry

    pub i0: bool, // interrupt mask 0
    pub i1: bool, // interrupt mask 1
    pub i2: bool, // interrupt mask 2
    pub m: bool, // master/interrupt state
    pub s: bool, // supervisor/user state
    pub t0: bool, // trace mode 0
    pub t1: bool, // trace mode 1

    pub ssp: u32, // supervisior stact pointer
    pub vbr: u32, // vector base register
    // TODO(matt) add cache registers
    pub ops: HashMap<u16, OpFn>, // opcodes
    pub mem: Mem, // memory
}

impl Cpu {
     pub fn new() -> Cpu {
         Cpu {
             d: [0;8], a:[0;8], 
             pc: 0, 
             x: false, n: false, z: true, v: false, c: false, 
             i0: false, i1: false, i2: false, m: true, s: true, t0: false, t1: false,
             ssp: 0,
             vbr: 0,
             mem: Mem { ram: [0; 0xffffff], },
             ops: HashMap::new(),
         }
     }

     pub fn build_op_table(&mut self) {
         //self.ops.insert("this will be a regex of all actual opcodes matching this method", "then here is the method");
         let mut inter_op_table = HashMap::new();
         inter_op_table.insert("1101[0-1]{3}1(00|01|10)(((010|011|100|101|110)[0-1]{3})|(111(000|001)))", arithmetic::add_16);
     }

     pub fn set_flags_16(&mut self, res: u32, src: u16, dst: u16) {
        self.z = res as u16 - 0xffff == 0;
        self.c = res >> 24 == 1;
        self.x = self.c;    // TODO(matt) - is this always true?
        self.n = res >> 15 == 1;
        self.v = ((src ^ res as u16) & (dst ^ res as u16)) >> 8 == 1;
    }
}