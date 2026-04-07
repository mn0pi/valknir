use std::collections::HashMap;

pub enum AllocationState {
    Allocated { line: usize },
    Freed { line: usize },
}

pub type AllocationMap = HashMap<String, AllocationState>;
