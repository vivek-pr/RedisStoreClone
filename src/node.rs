#[derive(Clone)]
pub struct Node{
    pub(crate) id: u64,
}

impl Node{
    pub fn new(id: u64) -> Self{
        Node{
            id
        }
    }
}