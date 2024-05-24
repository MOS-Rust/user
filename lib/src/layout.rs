use crate::{env::Env, page::{PageRc, Pte}};
use bitflags::bitflags;

pub const VPT: *const Pte = UVPT as *const Pte;
pub const VPD: *const Pte = (UVPT + (VA(UVPT).pdx() << PGSHIFT)) as *const Pte;
pub const ENVS: *const Env = UENVS as *const Env;
pub const PAGES: *const PageRc = UPAGES as *const PageRc;

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VA(pub usize);

impl VA {
    /// Page Directory Index
    pub const fn pdx(self) -> usize {
        (self.0 >> PDSHIFT) & 0x3ff
    }

    /// Page Table Index
    pub const fn ptx(self) -> usize {
        (self.0 >> PGSHIFT) & 0x3ff
    }

    /// Get the page table entry address from the virtual address
    pub const fn pte_addr(self) -> Self {
        Self(self.0 & !0xFFF)
    }
}



/// Maximum number of Address Space Identifiers(ASIDs)
pub const NASID: usize = 256;
/// Bytes per page
pub const PAGE_SIZE: usize = 4096;
/// Bytes mapped by a page table entry, 4 KiB
pub const PTMAP: usize = PAGE_SIZE;
/// Bytes mapped by a page directory entry, 4 MiB
pub const PDMAP: usize = 0x0040_0000;
/// Page shift value
pub const PGSHIFT: usize = 12;
/// Page directory shift value
pub const PDSHIFT: usize = 22;
/// PTE flag shift
pub const PTE_HARDFLAG_SHIFT: usize = 6;

pub const ULIM: usize = 0x8000_0000;

pub const UVPT: usize = ULIM - PDMAP;
pub const UPAGES: usize = UVPT - PDMAP;
pub const UENVS: usize = UPAGES - PDMAP;

pub const UTOP: usize = UENVS;
pub const UXSTACKTOP: usize = UTOP;

pub const USTACKTOP: usize = UTOP - 2 * PTMAP;
pub const UTEXT: usize = PDMAP;
pub const UCOW: usize = UTEXT - PTMAP;
pub const UTEMP: usize = UCOW - PTMAP;

bitflags! {
    pub struct PteFlags: usize {
        /// the 6 bits below are those stored in cp0.entry_lo
        const G = 1 << 0 << PTE_HARDFLAG_SHIFT;
        const V = 1 << 1 << PTE_HARDFLAG_SHIFT;
        const D = 1 << 2 << PTE_HARDFLAG_SHIFT;

        // Only used internally
        const C0 = 1 << 3 << PTE_HARDFLAG_SHIFT;
        const C1 = 1 << 4 << PTE_HARDFLAG_SHIFT;
        const C2 = 1 << 5 << PTE_HARDFLAG_SHIFT;

        const CACHEABLE = PteFlags::C0.bits() | PteFlags::C1.bits();
        const UNCACHED = PteFlags::C0.bits() & !PteFlags::C1.bits();

        /// the bits below are controlled by software
        const COW = 0x1;
        const SHARED = 0x2;
    }
}


impl Pte {
    pub const fn flags(self) -> PteFlags {
        PteFlags::from_bits_truncate(self.0 & 0xFFF)
    }
}