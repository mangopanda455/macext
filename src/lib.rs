use core::panic;
use mach2::{
    kern_return::KERN_SUCCESS,
    message::mach_msg_type_number_t,
    port::mach_port_t,
    traps::{mach_task_self, task_for_pid},
    vm::mach_vm_region,
    vm_prot::{VM_PROT_EXECUTE, VM_PROT_READ, VM_PROT_WRITE},
    vm_region::{vm_region_basic_info, VM_REGION_BASIC_INFO_64},
    vm_types::{vm_address_t, vm_size_t},
};
use process_memory::{DataMember, Memory, Pid, TryIntoProcessHandle};
use std::{ffi::CStr, usize};
use sysinfo::System;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub base_address: usize,
    pub size: usize,
}

pub fn get_base_address(pid: i32) -> vm_address_t {
    unsafe {
        let mut task: mach_port_t = 0;
        if task_for_pid(mach_task_self(), pid, &mut task) != KERN_SUCCESS {
            panic!("Error getting task!");
        }

        let mut address: vm_address_t = 1;
        let mut size: vm_size_t = 0;
        let mut info: vm_region_basic_info = std::mem::zeroed();
        let mut info_count = std::mem::size_of_val(&info) as mach_msg_type_number_t;
        let mut object_name: mach_port_t = 0;

        while mach_vm_region(
            task,
            &mut address as *mut _ as *mut u64,
            &mut size as *mut _ as *mut u64,
            VM_REGION_BASIC_INFO_64,
            &mut info as *mut _ as *mut i32,
            &mut info_count,
            &mut object_name,
        ) == KERN_SUCCESS
        {
            if info.protection & VM_PROT_EXECUTE != 0 {
                return address;
            }
            address += size;
        }
    }

    panic!("Base not found!")
}

pub fn patch(offsets: &Vec<u64>, base_address: usize, pid: i32, val: u64) {
    let handle = (pid as Pid).try_into_process_handle().unwrap();
    let mut current_address = base_address;
    let mut member: DataMember<u64> = DataMember::new(handle);
    for index in 0..offsets.len() {
        member = DataMember::new_offset(handle, vec![current_address + offsets[index] as usize]);
        unsafe {
            match member.read() {
                Ok(value) => current_address = value as usize,
                Err(e) => panic!("{}", e),
            }
        }
    }

    member.write(&val).unwrap();
}

pub fn read(offsets: &Vec<u64>, base_address: usize, pid: i32) -> u64 {
    let handle = (pid as Pid).try_into_process_handle().unwrap();
    let mut current_address = base_address;
    let mut member: DataMember<u64> = DataMember::new(handle);
    for index in 0..offsets.len() {
        member = DataMember::new_offset(handle, vec![current_address + offsets[index] as usize]);
        unsafe {
            match member.read() {
                Ok(value) => current_address = value as usize,
                Err(e) => panic!("{}", e),
            }
        }
    }
    unsafe {
        return member.read().unwrap();
    }
}

pub fn get_pid(process_name: &str) -> i32 {
    let mut system = System::new_all();
    system.refresh_all();

    let mut pid: i32 = -1;
    for process in system.processes_by_exact_name(&process_name) {
        pid = process.pid().as_u32() as i32;
    }
    if pid == -1 {
        panic!("Pid not found! Use sudo or make sure the target program is running.")
    }
    pid
}

pub fn fullprep(process_name: &str) -> (i32, usize) {
    let pid = get_pid(process_name);
    let base_address = get_base_address(pid);
    (pid, base_address)
}

pub fn get_modules(pid: i32) -> Vec<Module> {
    unsafe {
        let mut task: mach_port_t = 0;
        if task_for_pid(mach_task_self(), pid, &mut task) != KERN_SUCCESS {
            panic!("Error getting task!");
        }

        let mut address: vm_address_t = 1;
        let mut size: vm_size_t = 0;
        let mut info: vm_region_basic_info = std::mem::zeroed();
        let mut info_count = std::mem::size_of_val(&info) as mach_msg_type_number_t;
        let mut object_name: mach_port_t = 0;

        let mut modules = Vec::new();

        while mach_vm_region(
            task,
            &mut address as *mut _ as *mut u64,
            &mut size as *mut _ as *mut u64,
            VM_REGION_BASIC_INFO_64,
            &mut info as *mut _ as *mut i32,
            &mut info_count,
            &mut object_name,
        ) == KERN_SUCCESS
        {
            if info.protection & VM_PROT_EXECUTE != 0 {
                // Assuming module name is at the beginning of the module
                let module_name = String::from("Unimplemented");
                let module = Module {
                    name: module_name,
                    base_address: address as usize,
                    size: size as usize,
                };
                modules.push(module);
            }
            address += size;
        }
        modules
    }
}

// fn read_module_name(task: mach_port_t, address: vm_address_t) -> String {
//     let mut buffer: [u8; 256] = [0; 256];
//     let mut bytes_read: mach_msg_type_number_t = 0;

//     unsafe {
//         let result = mach_vm_read_overwrite(
//             task,
//             address,
//             256,
//             buffer.as_mut_ptr() as vm_address_t,
//             &mut bytes_read,
//         );

//         if result != KERN_SUCCESS {
//             return "Unknown".to_string();
//         }
//     }

//     let cstr = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
//     cstr.to_string_lossy().into_owned()
// }

#[link(name = "System")]
extern "C" {
    fn mach_vm_read_overwrite(
        task: mach_port_t,
        address: vm_address_t,
        size: vm_size_t,
        data: vm_address_t,
        data_size: *mut mach_msg_type_number_t,
    ) -> i32;
}

pub fn list_regions(pid: i32) {
    unsafe {
        let mut task: mach_port_t = 0;
        if task_for_pid(mach_task_self(), pid, &mut task) != KERN_SUCCESS {
            panic!("Error getting task for PID {}!", pid);
        }

        let mut address: vm_address_t = 1;
        let mut size: vm_size_t = 0;
        let mut info: vm_region_basic_info = std::mem::zeroed();
        let mut info_count = std::mem::size_of_val(&info) as mach_msg_type_number_t;
        let mut object_name: mach_port_t = 0;

        println!("Regions for PID {}:", pid);
        while mach_vm_region(
            task,
            &mut address as *mut _ as *mut u64,
            &mut size as *mut _ as *mut u64,
            VM_REGION_BASIC_INFO_64,
            &mut info as *mut _ as *mut i32,
            &mut info_count,
            &mut object_name,
        ) == KERN_SUCCESS
        {
            println!(
                "Address: 0x{:016x}, Size: 0x{:x}, Protection: {}",
                address,
                size,
                format_protection(info.protection)
            );
            address += size;
        }
    }
}

fn format_protection(prot: i32) -> String {
    let mut prot_flags = String::new();
    if prot & VM_PROT_READ != 0 {
        prot_flags.push('R');
    }
    if prot & VM_PROT_WRITE != 0 {
        prot_flags.push('W');
    }
    if prot & VM_PROT_EXECUTE != 0 {
        prot_flags.push('X');
    }
    prot_flags
}

pub fn read_string(offsets: &[u64], base_address: usize, pid: i32, max_length: usize) -> String {
    let handle = (pid as Pid).try_into_process_handle().unwrap();
    let mut current_address = base_address;
    let mut member: DataMember<u64> = DataMember::new(handle);
    // Normal offset method is completely and utterly fucked for strings :)
    // for index in 0..offsets.len() {
    //     member = DataMember::new_offset(handle, vec![current_address + offsets[index] as usize]);
    //     unsafe {
    //         match member.read() {
    //             Ok(value) => current_address = value as usize,
    //             Err(e) => panic!("{}", e),
    //         }
    //     }
    // }

    // LMAOOOOOO this is not real offsets at all just adding hex
    for i in 0..offsets.len() {
        current_address += offsets[i] as usize;
    }

    // Read the string from the final address
    let mut buffer = vec![0u8; max_length];
    let mut bytes_read: usize = 0;
    for i in 0..max_length {
        let byte_member = DataMember::<u8>::new_offset(handle, vec![current_address + i]);
        unsafe {
            match byte_member.read() {
                Ok(byte) => {
                    if byte == 0 {
                        break; // Stop reading if we encounter a null terminator
                    }
                    buffer[bytes_read] = byte;
                    bytes_read += 1;
                }
                Err(e) => panic!("{}", e),
            }
        }
    }
    buffer.truncate(bytes_read); // Truncate the buffer to the actual string length
    String::from_utf8_lossy(&buffer).into_owned()
}
