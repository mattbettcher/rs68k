#![allow(dead_code)]
#![allow(unused_variables)]

// defines the memory interface

pub enum AddressSpace {
    UserData,
    UserProgram,
    SupervisorData,
    SupervisorProgram,
}

/// Interface for accessing the address bus.
pub trait AddressBus {
    fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32;
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32;
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32;
    fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32);
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32);
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32);
}

pub struct Mem {
    pub ram: [u32; 0xffffff]
}

impl AddressBus for Mem {
    fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32 {
        unimplemented!();
    }
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32{
        unimplemented!();
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32{
        unimplemented!();
    }
    fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32){
        unimplemented!();
    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32){
        unimplemented!();
    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32){
        unimplemented!();
    }
    
}