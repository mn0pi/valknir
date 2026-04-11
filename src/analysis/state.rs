use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AllocationState {
    Allocated { alloc_line: usize },
    Freed { alloc_line: usize, free_line: usize },
}

pub type AllocationMap = HashMap<String, AllocationState>;
