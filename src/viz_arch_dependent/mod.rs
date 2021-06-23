#[cfg_attr(feature="pc", path = "default_pc.rs")]
pub mod arch_dependent;
/*
Note: Must implement:
pub fn load_program(&mut program_space:[u16; 0x10000])
pub fn io_in(port:u16) -> u16
pub fn io_out(data:u16, port:u16)
pub fn entropy() -> u16
pub fn pow16(base:u16, exp:u16) -> u16
*/