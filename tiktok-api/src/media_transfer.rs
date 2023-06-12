//
pub const CHUNK_SIZE_MIN: usize = 1024 * 1024 * 5;
pub const CHUNK_SIZE_MAX: usize = 1024 * 1024 * 64;
pub const CHUNK_COUNT_MIN: usize = 1;
pub const CHUNK_COUNT_MAX: usize = 1000;

//
pub fn get_chunk_size_and_total_chunk_count(file_size: u64) -> (u64, u64) {
    if file_size <= CHUNK_SIZE_MAX as u64 {
        (file_size, 1)
    } else {
        (
            CHUNK_SIZE_MAX as u64,
            (file_size as f64 / CHUNK_SIZE_MAX as f64).ceil() as u64,
        )
    }
}
