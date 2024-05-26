use crate::{curenv, syscall_ipc_recv, syscall_ipc_try_send, syscall_yield, VA};

#[repr(C)]
#[derive(Debug)]
pub struct IpcInfo {
    pub value: u32,
    pub from: usize,
    pub recving: IpcStatus,
    pub dstva: VA,
    pub perm: usize,
}

impl Default for IpcInfo {
    fn default() -> Self {
        Self {
            value: 0,
            from: 0,
            recving: IpcStatus::NotReceiving,
            dstva: VA(0),
            perm: 0,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpcStatus {
    NotReceiving = 0,
    Receiving = 1,
}

impl IpcStatus {
    pub fn to_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Self {
        match val {
            0 => IpcStatus::NotReceiving,
            1 => IpcStatus::Receiving,
            _ => panic!("Invalid IpcStatus value: {}", val),
        }
    }
}

pub fn ipc_send(to: usize, val: u32, srcva: VA, perm: u32) {
    loop {
        let r = syscall_ipc_try_send(to, val, srcva, perm);
        if r == 0 {
            break;
        }
        if IpcStatus::from_u32(r as u32) != IpcStatus::Receiving {
            panic!("ipc_send: unexpected error: {}", r);
        }
        syscall_yield();
    }
}


/// Returns (from, value, perm)
pub fn ipc_recv(dstva: VA) -> (usize, u32, u32) {
    match syscall_ipc_recv(
        dstva
    ) {
        0 => (),
        r => panic!("ipc_recv: unexpected error: {}", r),
    }
    let ipc_info = &curenv().ipc_info;
    (
        ipc_info.from,
        ipc_info.value,
        ipc_info.perm as u32,
    )
}