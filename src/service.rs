use winapi::um::winsvc::{OpenSCManagerA, SC_MANAGER_CREATE_SERVICE, CreateServiceA, SERVICE_START, SERVICE_STOP, OpenServiceA, CloseServiceHandle, SC_HANDLE, StartServiceA, ControlService, SERVICE_CONTROL_STOP, SERVICE_STATUS, DeleteService};
use winapi::_core::ptr::{null, null_mut};
use winapi::um::winnt::{DELETE, SERVICE_KERNEL_DRIVER, SERVICE_DEMAND_START, SERVICE_ERROR_IGNORE};
use std::ffi::CString;
use std::mem;

pub struct DriverService {
    sc_manager: SC_HANDLE,
    handle: SC_HANDLE,
}

impl DriverService {
    pub fn new(name: &str, driver_path: &str) -> Self {
        unsafe {
            let name = CString::new(name).unwrap();
            let name = name.as_ptr();
            let driver_path = CString::new(driver_path).unwrap();
            let driver_path = driver_path.as_ptr();
            let sc_manager = OpenSCManagerA(null(), null(), SC_MANAGER_CREATE_SERVICE);
            let mut handle = CreateServiceA(
                sc_manager,
                name,
                name,
                SERVICE_START | DELETE | SERVICE_STOP,
                SERVICE_KERNEL_DRIVER,
                SERVICE_DEMAND_START,
                SERVICE_ERROR_IGNORE,
                driver_path,
                null(), null_mut(), null(), null(), null()
            );

            if handle == null_mut() {
                handle = OpenServiceA(
                    sc_manager,
                    name,
                    SERVICE_START | DELETE | SERVICE_STOP,
                );
            }

            StartServiceA(handle, 0, null_mut());

            Self {
                sc_manager,
                handle
            }
        }
    }
}

impl Drop for DriverService {
    fn drop(&mut self) {
        unsafe {
            let ss: SERVICE_STATUS = mem::MaybeUninit::uninit().assume_init();
            ControlService(self.handle, SERVICE_CONTROL_STOP, &ss as *const _ as *mut _);
            DeleteService(self.handle);
            CloseServiceHandle(self.handle);
            CloseServiceHandle(self.sc_manager);
        }
    }
}