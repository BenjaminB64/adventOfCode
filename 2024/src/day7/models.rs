#[derive(Debug)]
pub struct Equation {
    pub(crate) result: u64,
    pub(crate) inputs: Vec<u64>,
}

impl Equation {
    pub fn new(result: u64, inputs: Vec<u64>) -> Self {
        Self { result, inputs }
    }
}