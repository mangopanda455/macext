use std::usize;

use mach2::{vm_types::{vm_address_t, vm_size_t}, port::mach_port_t, traps::{task_for_pid, mach_task_self}, kern_return::KERN_SUCCESS, vm_region::{vm_region_basic_info, VM_REGION_BASIC_INFO_64}, message::mach_msg_type_number_t, vm::mach_vm_region, vm_prot::VM_PROT_EXECUTE};
use process_memory::{Pid, TryIntoProcessHandle, DataMember, Memory};
use sysinfo::System;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

pub fn get_base_address(pid: i32) -> Option<vm_address_t> {
    unsafe {
        let mut task: mach_port_t = 0;
        if task_for_pid(mach_task_self(), pid, &mut task) != KERN_SUCCESS {
            return None;
        }

        let mut address: vm_address_t = 1;
        let mut size: vm_size_t = 0;
        let mut info: vm_region_basic_info = std::mem::zeroed();
        let mut info_count = std::mem::size_of_val(&info) as mach_msg_type_number_t;
        let mut object_name: mach_port_t = 0;

        while mach_vm_region(task, &mut address as *mut _ as *mut u64, &mut size as *mut _ as *mut u64, VM_REGION_BASIC_INFO_64, &mut info as *mut _ as *mut i32, &mut info_count, &mut object_name) == KERN_SUCCESS {
            if info.protection & VM_PROT_EXECUTE != 0 {
                return Some(address);
            }
            address += size;
        }
    }
    
    None
}

pub fn patch(offsets: Vec<u64>, base_address: usize, pid: i32, val: u64) {
    let mut handle = (pid as Pid).try_into_process_handle().unwrap();
    let mut current_address = base_address;
    let mut member: DataMember<u64> = DataMember::new(handle);
    for index in 0..offsets.len() {
        member = DataMember::new_offset(handle, vec![current_address + offsets[index] as usize]);
        unsafe {
            match member.read() {
                Ok(value) => current_address = value as usize,
                Err(e) => println!("Error {}", e),
            }
        }
    }
    
    member.write(&val).unwrap();
}

pub fn read(offsets: Vec<u64>, base_address: usize, pid: i32) -> u64 {
    let mut handle = (pid as Pid).try_into_process_handle().unwrap();
    let mut current_address = base_address;
    let mut member: DataMember<u64> = DataMember::new(handle);
    for index in 0..offsets.len() {
        member = DataMember::new_offset(handle, vec![current_address + offsets[index] as usize]);
        unsafe {
            match member.read() {
                Ok(value) => current_address = value as usize,
                Err(e) => println!("Error {}", e),
            }
        }
    }
    unsafe {
        return member.read().unwrap();
    }
}

pub fn get_pid() -> i32 {
    let mut system = System::new_all();
    system.refresh_all();

    let mut pid: i32 = 0;
    for process in system.processes_by_exact_name("assaultcube") {
        pid = process.pid().as_u32() as i32;
    }
    println!("Target PID: {}", pid); 
    return pid;
}

pub fn get_pid_ba(pid: i32) -> usize {
    let mut base_address = 0;

    match get_base_address(pid) {
        Some(value) => base_address = value,
        None => println!("Base address not found"),
    }
    println!("Found base: {}", base_address);
    return base_address;
}
