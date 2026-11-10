use spin::Mutex;

pub const POOL_SIZE: usize = 64 * 1024;

// MEMORY POOL

struct MemoryPool {
    buffer: [u8; POOL_SIZE],
    used: usize,
}

impl MemoryPool {
    const fn new() -> Self {
        Self {
            buffer: [0; POOL_SIZE],
            used: 0,
        }
    }
}

// GLOBAL MEMORY POOL

static MEMORY: Mutex<MemoryPool> =
    Mutex::new(MemoryPool::new());

// MEMORY ALLOCATION

pub fn allocate(size: usize) -> Option<usize> {
    let mut memory = MEMORY.lock();

    let end = memory.used.checked_add(size)?;

    if end > POOL_SIZE {
        return None;
    }

    let start = memory.used;

    memory.buffer[start..end].fill(0);

    memory.used = end;

    Some(start)
}

// MEMORY INFORMATION

pub fn get_stats() -> (usize, usize) {
    let memory = MEMORY.lock();

    (memory.used, POOL_SIZE - memory.used)
}
