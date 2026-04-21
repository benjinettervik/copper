use std::any::TypeId;
use std::collections::HashSet;

pub struct SystemMeta {
    pub reads: HashSet<TypeId>,
    pub writes: HashSet<TypeId>,
    pub resource_reads: HashSet<TypeId>,
    pub resource_writes: HashSet<TypeId>,
}