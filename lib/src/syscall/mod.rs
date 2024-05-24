use core::arch::global_asm;

use crate::{env::{EnvStatus, Trapframe}, VA};

global_asm!(include_str!("syscall.S"));

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
enum Syscall {
    Putchar        = 0,
    PrintConsole   = 1,
    GetEnvId       = 2,
    Yield          = 3,
    EnvDestroy     = 4,
    SetTlbModEntry = 5,
    MemAlloc       = 6,
    MemMap         = 7,
    MemUnmap       = 8,
    Exofork        = 9,
    SetEnvStatus   = 10,
    SetTrapframe   = 11,
    Panic          = 12,
    IpcTrySend     = 13,
    IpcRecv        = 14,
    Getchar        = 15,
    WriteDev       = 16,
    ReadDev        = 17,
    MempoolOp      = 18,
    Unhandled      = 19,
}

impl Syscall {
    fn to_u32(self) -> u32 {
        self as u32
    }
}

extern "C" {
    fn msyscall(syscall: u32, ...) -> i32;
}

fn _syscall_0(syscall: Syscall) -> i32 {
    unsafe {
        msyscall(syscall.to_u32())
    }
}

fn _syscall_1(syscall: Syscall, arg1: u32) -> i32 {
    unsafe {
        msyscall(syscall.to_u32(), arg1)
    }
}

fn _syscall_2(syscall: Syscall, arg1: u32, arg2: u32) -> i32 {
    unsafe {
        msyscall(syscall.to_u32(), arg1, arg2)
    }
}

fn _syscall_3(syscall: Syscall, arg1: u32, arg2: u32, arg3: u32) -> i32 {
    unsafe {
        msyscall(syscall.to_u32(), arg1, arg2, arg3)
    }
}

fn _syscall_4(syscall: Syscall, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> i32 {
    unsafe {
        msyscall(syscall.to_u32(), arg1, arg2, arg3, arg4)
    }
}

fn _syscall_5(syscall: Syscall, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) -> i32 {
    unsafe {
        msyscall(syscall.to_u32(), arg1, arg2, arg3, arg4, arg5)
    }
}

pub fn syscall_putchar(ch: i32) {
    _syscall_1(Syscall::Putchar, ch as u32);
}

pub fn syscall_print_console(s: &str) -> i32 {
    _syscall_2(Syscall::PrintConsole, s.as_ptr() as u32, s.len() as u32)
}

pub fn syscall_getenvid() -> usize {
    _syscall_1(Syscall::GetEnvId, 0) as usize
}

pub fn syscall_yield() {
    _syscall_0(Syscall::Yield);
}

pub fn syscall_env_destroy(envid: usize) -> i32 {
    _syscall_1(Syscall::EnvDestroy, envid as u32)
}

pub fn syscall_set_tlb_mod_entry(envid: usize, entry: impl Fn(&Trapframe)) -> i32 {
    _syscall_2(Syscall::SetTlbModEntry, envid as u32, &entry as *const _ as u32)
}

pub fn syscall_mem_alloc(envid: usize, va: VA, perm: u32) -> i32 {
    _syscall_3(Syscall::MemAlloc, envid as u32, va.0 as u32, perm)
}

pub fn syscall_mem_map(srcid: usize, srcva: VA, dstid: usize, dstva: VA, perm: u32) -> i32 {
    _syscall_5(Syscall::MemMap, srcid as u32, srcva.0 as u32, dstid as u32, dstva.0 as u32, perm)
}

pub fn syscall_mem_unmap(envid: usize, va: VA) -> i32 {
    _syscall_2(Syscall::MemUnmap, envid as u32, va.0 as u32)
}

#[inline(always)]
pub fn syscall_exofork() -> i32 {
    _syscall_0(Syscall::Exofork)
}

pub fn syscall_set_env_status(envid: usize, status: &EnvStatus) -> i32 {
    _syscall_2(Syscall::SetEnvStatus, envid as u32, status.to_u32())
}

pub fn syscall_set_trapframe(envid: usize, tf: &Trapframe) -> i32 {
    _syscall_2(Syscall::SetTrapframe, envid as u32, tf as *const _ as u32)
}

pub fn syscall_panic() -> ! {
    _syscall_0(Syscall::Panic);
    loop {}
}

pub fn syscall_ipc_try_send(envid: usize, value: u32, srcva: VA, perm: u32) -> i32 {
    _syscall_4(Syscall::IpcTrySend, envid as u32, value, srcva.0 as u32, perm)
}

pub fn syscall_ipc_recv(dstva: VA) -> i32 {
    _syscall_2(Syscall::IpcRecv, dstva.0 as u32, 0)
}

pub fn syscall_getchar() -> i32 {
    _syscall_1(Syscall::Getchar, 0)
}

pub fn syscall_write_dev(va: VA, dev: u32, size: usize) -> i32 {
    _syscall_3(Syscall::WriteDev, va.0 as u32, dev, size as u32)
}

pub fn syscall_read_dev(va: VA, dev: u32, size: usize) -> i32 {
    _syscall_3(Syscall::ReadDev, va.0 as u32, dev, size as u32)
}

pub fn syscall_mempool_op(op: u32, poolid: u32, va: u32, page_count: u32) -> i32 {
    _syscall_4(Syscall::MempoolOp, op, poolid, va, page_count)
}


