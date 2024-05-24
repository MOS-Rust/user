use crate::{
    curenv, syscall_exofork, syscall_mem_alloc, syscall_mem_map, syscall_mem_unmap,
    syscall_set_env_status, syscall_set_tlb_mod_entry, syscall_set_trapframe, try_run, PteFlags,
    Trapframe, PAGE_SIZE, PDSHIFT, PGSHIFT, UCOW, USTACKTOP, VA, VPD, VPT,
};

macro_rules! round_down {
    ($x:expr, $n:expr) => {
        $x & !($n - 1)
    };
}

fn cow_entry(tf: *const Trapframe) -> ! {
    let va = VA((unsafe { &*tf }).cp0_badvaddr as usize);
    let mut perm = unsafe { &*VPT.add(va.0 >> PGSHIFT) }.flags();
    if !perm.contains(PteFlags::COW) {
        panic!("PTE_COW is not set");
    }

    perm &= !PteFlags::COW;
    perm |= PteFlags::D;

    syscall_mem_alloc(0, VA(UCOW), perm.bits() as u32);

    unsafe {
        core::ptr::copy(
            round_down!(va.0, PAGE_SIZE) as *const u8,
            UCOW as *mut u8,
            PAGE_SIZE,
        );
    }

    syscall_mem_map(0, VA(UCOW), 0, va, perm.bits() as u32);
    syscall_mem_unmap(0, VA(UCOW));

    let r = syscall_set_trapframe(0, tf);
    if r < 0 {
        panic!("cow_entry: syscall_set_trapframe: {}", r);
    }

    unreachable!()
}

fn duppage(envid: usize, vpn: usize) {
    let addr = VA(vpn << PGSHIFT);
    let mut perm = unsafe { &*VPT.add(vpn) }.flags();

    let mut flag = false;
    if (perm.contains(PteFlags::D) && !perm.contains(PteFlags::COW))
        && !perm.contains(PteFlags::SHARED)
    {
        perm |= PteFlags::COW;
        perm &= !PteFlags::D;
        flag = true;
    }

    syscall_mem_map(0, addr, envid, addr, perm.bits() as u32);

    if flag {
        syscall_mem_map(0, addr, 0, addr, perm.bits() as u32);
    }
}

pub fn fork() -> i32 {
    if curenv().user_tlb_mod_entry != cow_entry as usize {
        try_run!(syscall_set_tlb_mod_entry(0, cow_entry));
    }

    let child = syscall_exofork();
    if child == 0 {
        return 0;
    }

    for i in (0..USTACKTOP).step_by(PAGE_SIZE) {
        let vpn = i >> PGSHIFT;
        let pde = unsafe { &*VPD.add(i >> PDSHIFT) };
        let pte = unsafe { &*VPT.add(vpn) };
        if pde.flags().contains(PteFlags::V) && pte.flags().contains(PteFlags::V) {
            duppage(child as usize, vpn);
        }
    }

    try_run!(syscall_set_tlb_mod_entry(child as usize, cow_entry));
    try_run!(syscall_set_env_status(
        child as usize,
        crate::EnvStatus::Runnable
    ));

    child
}
