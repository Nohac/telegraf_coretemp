extern crate winapi;

use std::ffi::{CStr, CString};
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{MapViewOfFile, FILE_MAP_READ};
use winapi::um::winbase::OpenFileMappingA;

#[derive(Debug)]
#[repr(C)]
pub struct CoreTempSharedDataEx {
    pub ui_load: [u32; 256],
    pub ui_tj_max: [u32; 128],
    pub ui_core_cnt: u32,
    pub ui_cpu_cnt: u32,
    pub f_temp: [f32; 256],
    pub f_vid: f32,
    pub f_cpu_speed: f32,
    pub f_fsb_speed: f32,
    pub f_multiplier: f32,
    pub s_cpu_name: [i8; 100], // `char` in C is typically `i8` in Rust.
    pub uc_fahrenheit: u8,
    pub uc_delta_to_tj_max: u8,
    pub uc_tdp_supported: u8,
    pub uc_power_supported: u8,
    pub ui_struct_version: u32,
    pub ui_tdp: [u32; 128],
    pub f_power: [f32; 128],
    pub f_multipliers: [f32; 256],
}

fn main() {
    // The name of the shared memory file used by Core Temp
    let shared_memory_name = CString::new("CoreTempMappingObjectEx").unwrap();

    let core_temp = unsafe {
        let h_map_file = OpenFileMappingA(FILE_MAP_READ, 0, shared_memory_name.as_ptr());
        if h_map_file.is_null() {
            eprintln!("Could not open file mapping");
            return;
        }

        let p_buf = MapViewOfFile(h_map_file, FILE_MAP_READ, 0, 0, 0);
        if p_buf.is_null() {
            eprintln!("Could not map view of file");
            CloseHandle(h_map_file);
            return;
        }

        let data = &*(p_buf as *const CoreTempSharedDataEx);
        CloseHandle(h_map_file);

        data
    };

    let c_str: &CStr = unsafe { CStr::from_ptr(core_temp.s_cpu_name.as_ptr()) };
    let cpu_name = c_str.to_str().unwrap();

    let cpu_name = cpu_name.trim().replace(" ", "_");

    let core_count = core_temp.ui_core_cnt;

    let cores = core_temp.f_temp[..(core_temp.ui_core_cnt as usize)]
        .iter()
        .enumerate()
        .map(|(i, temp)| format!("core{i}_temp={temp:.1}"))
        .collect::<Vec<String>>()
        .join(",");

    println!("coretemp,cpu={cpu_name},core_count={core_count} {cores}");
}
