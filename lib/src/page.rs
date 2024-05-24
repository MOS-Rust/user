// Structs defined in the kernel
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pte(pub usize);

#[repr(C)]
#[derive(Debug)]
pub struct PageRc {
    __placeholder: [usize; 2],
    ref_count: u16,
}