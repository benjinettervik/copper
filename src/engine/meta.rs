use std::any::TypeId;
use std::collections::HashSet;

pub struct SystemMeta {
    pub reads: HashSet<TypeId>,
    pub writes: HashSet<TypeId>,
    pub resource_reads: HashSet<TypeId>,
    pub resource_writes: HashSet<TypeId>,
}

impl SystemMeta {
    pub fn print_it(&self) {
        println!("Reads:");
        for item in &self.reads {
            println!("{:?}", item);
        }

        println!("\nWrites:");
        for item in &self.writes {
            println!("{:?}", item);
        }

        println!("\nResource Reads:");
        for item in &self.resource_reads {
            println!("{:?}", item);
        }

        println!("\nResource Writes:");
        for item in &self.resource_writes {
            println!("{:?}", item);
        }
    }
}