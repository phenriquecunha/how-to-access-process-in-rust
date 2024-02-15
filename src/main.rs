use std::ptr;
use std::ffi::CString;
use std::thread::sleep;
use std::time::Duration;
use winapi::um::winuser::{FindWindowA, GetWindowThreadProcessId};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
fn main() {

    unsafe {

        let window_name = CString::new("programabruxo").expect("Failed to create window name CString");
        let window = FindWindowA(ptr::null(), window_name.as_ptr());
        if window == ptr::null_mut() {
            println!("Couldn't find window");
            return;
        }

        let mut proc_id = 0;
        GetWindowThreadProcessId(window, &mut proc_id);
        if proc_id == 0 {
            println!("Couldn't get process ID");
            return;
        }

        let h_proc = OpenProcess(PROCESS_ALL_ACCESS, 0, proc_id);
        if h_proc == ptr::null_mut() {
            println!("Couldn't open a handle to the process");
            return;
        }


        println!("Window: {:?}", window);
        println!("Process ID: {:?}", proc_id);
        println!("Process Handle: {:?}", h_proc);

        let address: usize = 0x404004;
        let mut value = 0;
        let mut buffer = 0;
        loop {
            ReadProcessMemory(h_proc, address as *const _, &mut buffer as *mut _ as *mut _, std::mem::size_of::<i32>(), ptr::null_mut());
            println!("Buffer: {}", buffer);
            value += buffer;
            WriteProcessMemory(h_proc, address as *mut _, &value as *const _ as *const _, std::mem::size_of::<i32>(), ptr::null_mut());
            sleep(Duration::from_secs(3));
        }
    }
}
