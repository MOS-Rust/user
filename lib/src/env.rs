use crate::{layout::{IpcStatus, VA}, syscall_yield, ENVS};

#[repr(C)]
#[derive(Debug)]
pub struct Env {
    pub tf: Trapframe,

    __placeholder_1: [usize; 2], // env_link

    pub id: usize,
    pub asid: usize,
    pub parent_id: usize,
    pub status: EnvStatus,
    pub pgdir: VA,

    __placeholder_2: [usize; 2], // env_sched_link

    pub priority: u32,

    // IPC
    pub ipc_info: IpcInfo,

    pub user_tlb_mod_entry: usize,

    pub runs: u32,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnvStatus {
    Free = 0,
    Runnable = 1,
    NotRunnable = 2,
}

impl EnvStatus {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Trapframe {
    pub regs: [u32; 32],
    pub cp0_status: u32,
    pub hi: u32,
    pub lo: u32,
    pub cp0_badvaddr: u32,
    pub cp0_cause: u32,
    pub cp0_epc: u32,
}

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


pub fn envx(envid: usize) -> usize {
    envid & ((1 << 10) - 1)
}

pub fn wait(envid: usize) {
    let e = unsafe { &*ENVS.add(envx(envid)) };
    #[allow(clippy::while_immutable_condition)]
    while e.id == envid && e.status != EnvStatus::Free {
        syscall_yield();
    }
}