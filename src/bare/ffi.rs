#[allow(improper_ctypes)]
extern "C" {
    pub fn hal_frame_alloc() -> Option<usize>;
    pub fn hal_frame_alloc_contiguous(frame_count: usize, align_log2: usize) -> Option<usize>;
    pub fn hal_frame_dealloc(paddr: usize);
}
