use std::error::Error;
use crate::cpu::CpuInterface;

mod service;
mod win_ring;
mod cpu;

fn main() -> Result<(), Box<dyn Error>> {
    let cpu = CpuInterface::new();
    println!("{:04x}", cpu.read_pci_dword(0, 31, 0, 0xf0));
    Ok(())
}
