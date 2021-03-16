use crate::service::DriverService;
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use std::ffi::CString;
use winapi::_core::ptr::null_mut;
use winapi::um::winnt::{GENERIC_READ, GENERIC_WRITE, HANDLE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::ioapiset::DeviceIoControl;
use std::mem;

const IOCTL_READ_MEM: u32 = 0x9C406104;
const IOCTL_WRITE_MEM: u32 = 0x9C40A108;
const IOCTL_READ_MSR: u32 = 0x9C402084;
const IOCTL_WRITE_MSR: u32 = 0x9C402088;
const IOCTL_IN_BYTE: u32 = 0x9C4060CC;
const IOCTL_IN_WORD: u32 = 0x9C4060D0;
const IOCTL_IN_DWORD: u32 = 0x9C4060D4;
const IOCTL_OUT_BYTE: u32 = 0x9C40A0D8;
const IOCTL_OUT_WORD: u32 = 0x9C40A0DC;
const IOCTL_OUT_DWORD: u32 = 0x9C40A0E0;

#[repr(C)]
struct MemRequest<const S: usize> {
    address: u64,
    element_size: u32,
    element_count: u32,
    array: [u8; S],
}

#[repr(C)]
struct MsrRequest {
    address: u32,
    value: u64,
}

#[repr(C)]
struct OutRequest<T: Sized> {
    address: u16,
    value: T,
}

pub struct WinRing0 {
    service: DriverService,
    device_handle: HANDLE,
}

impl WinRing0 {
    pub fn new() -> Self {
        let service = DriverService::new(
            "WinRing0_1_2_0",
            std::fs::canonicalize(".\\WinRing0x64.sys").unwrap().to_str().unwrap()
        );

        unsafe {
            let device_name = CString::new(r"\\.\WinRing0_1_2_0").unwrap();
            let device_handle = CreateFileA(
                device_name.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                null_mut(),
                OPEN_EXISTING,
                0,
                null_mut()
            );

            Self {
                service,
                device_handle
            }
        }
    }

    pub fn read(&self, address: u64, count: u32) -> Vec<u8> {
        let request = MemRequest {
            address,
            element_size: 1,
            element_count: count,
            array: []
        };
        let mut out: Vec<u8> = Vec::with_capacity(count as usize);
        let bytes_read = 0u32;
        unsafe {
            DeviceIoControl(
                self.device_handle,
                IOCTL_READ_MEM,
                &request as *const _ as *mut _, mem::size_of::<MemRequest<0>>() as u32,
                out.as_mut_ptr() as *mut _, count,
                &bytes_read as *const _ as *mut _, null_mut()
            );
            out.set_len(bytes_read as usize);
        }
        return out;
    }

    pub fn write<const S: usize>(&self, address: u64, data: [u8; S]) {
        let request = MemRequest {
            address,
            element_size: 1,
            element_count: S as u32,
            array: data
        };
        let bytes_read = 0u32;
        unsafe {
            DeviceIoControl(
                self.device_handle,
                IOCTL_WRITE_MEM,
                &request as *const _ as *mut _, mem::size_of::<MemRequest<S>>() as u32,
                null_mut(), 0,
                &bytes_read as *const _ as *mut _, null_mut()
            );
        }
    }

    pub fn read_msr(&self, address: u32) -> u64 {
        let request = MsrRequest {
            address,
            value: 0
        };
        let out = 0u64;
        let bytes_read = 0u32;
        unsafe {
            DeviceIoControl(
                self.device_handle,
                IOCTL_READ_MSR,
                &request as *const _ as *mut _, mem::size_of::<MsrRequest>() as u32,
                &out as *const _ as *mut _, 8,
                &bytes_read as *const _ as *mut _, null_mut()
            );
        }

        out
    }

    pub fn write_msr(&self, address: u32, value: u64) {
        let request = MsrRequest {
            address,
            value,
        };
        let bytes_read = 0u32;
        unsafe {
            DeviceIoControl(
                self.device_handle,
                IOCTL_WRITE_MSR,
                &request as *const _ as *mut _, mem::size_of::<MsrRequest>() as u32,
                null_mut(), 8,
                &bytes_read as *const _ as *mut _, null_mut()
            );
        }
    }

    unsafe fn generic_in<T: Sized>(&self, address: u16, ioctl: u32) -> T {
        let address = address as u32;
        let out = mem::zeroed();
        let bytes_read = 0u32;
        DeviceIoControl(
            self.device_handle,
            ioctl,
            &address as *const _ as *mut _, mem::size_of::<u32>() as u32,
            &out as *const _ as *mut _, mem::size_of::<T>() as u32,
            &bytes_read as *const _ as *mut _, null_mut()
        );

        out
    }

    pub fn in8(&self, address: u16) -> u8 {
        unsafe { self.generic_in(address, IOCTL_IN_BYTE) }
    }

    pub fn in16(&self, address: u16) -> u16 {
        unsafe { self.generic_in(address, IOCTL_IN_WORD) }
    }

    pub fn in32(&self, address: u16) -> u32 {
        unsafe { self.generic_in(address, IOCTL_IN_DWORD) }
    }

    unsafe fn generic_out<T: Sized>(&self, address: u16, value: T, ioctl: u32) {
        let request = OutRequest {
            address,
            value,
        };
        let bytes_read = 0u32;
        DeviceIoControl(
            self.device_handle,
            ioctl,
            &request as *const _ as *mut _, mem::size_of::<OutRequest<T>>() as u32,
            null_mut(), 0,
            &bytes_read as *const _ as *mut _, null_mut()
        );
    }

    pub fn out8(&self, address: u16, value: u8) {
        unsafe { self.generic_out(address, value, IOCTL_OUT_BYTE) }
    }

    pub fn out16(&self, address: u16, value: u16) {
        unsafe { self.generic_out(address, value, IOCTL_OUT_WORD) }
    }

    pub fn out32(&self, address: u16, value: u32) {
        unsafe { self.generic_out(address, value, IOCTL_OUT_DWORD) }
    }
}

impl Drop for WinRing0 {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.device_handle);
        }
    }
}