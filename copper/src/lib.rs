pub mod engine;

pub trait Component {
    fn name(&self) -> &str;
}