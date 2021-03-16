use crate::win_ring::WinRing0;

pub const IO_PORT_PCI_CONFIG_ADDRESS: u16 = 0xcf8;
pub const IO_PORT_PCI_CONFIG_DATA: u16 = 0xcfc;

pub struct CpuInterface {
    pub wr: WinRing0
}

impl CpuInterface {
    pub fn new() -> Self {
        let wr = WinRing0::new();

        Self {
            wr
        }
    }

    pub fn in8(&self, address: u16) -> u8 {
        self.wr.in8(address)
    }

    pub fn in16(&self, address: u16) -> u16 {
        self.wr.in16(address)
    }

    pub fn in32(&self, address: u16) -> u32 {
        self.wr.in32(address)
    }

    pub fn out8(&self, address: u16, value: u8) {
        self.wr.out8(address, value)
    }

    pub fn out16(&self, address: u16, value: u16) {
        self.wr.out16(address, value)
    }

    pub fn out32(&self, address: u16, value: u32) {
        self.wr.out32(address, value)
    }

    pub fn read_msr(&self, address: u32) -> u64{
        self.wr.read_msr(address)
    }

    pub fn write_msr(&self, address: u32, value: u64) {
        self.wr.write_msr(address, value)
    }

    pub fn read_pci_word(&self, bus: u8, device: u8, function: u8, offset: u8) -> u16{
        let bus = bus as u32;
        let device = device as u32;
        let function = function as u32;
        let offset = offset as u32;

        let address =
            0x80000000u32 |
            (bus << 16) | (device << 11) | (function << 8) | (offset * 0xfc);

        self.out32(IO_PORT_PCI_CONFIG_ADDRESS, address);

        ((self.in32(IO_PORT_PCI_CONFIG_DATA) >> ((offset & 2) * 8)) & 0xffff) as u16
    }
}