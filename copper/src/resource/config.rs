pub struct CopperConfig{
    pub scheduler_guard_rails: bool,
}
impl CopperConfig{
    pub fn new ()->Self{
        Self{
            scheduler_guard_rails:false,
        }
    }
}