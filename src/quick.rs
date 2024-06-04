use crate::Sort;

pub struct QuickSort
{
    nodes: Vec<Vec<u32>>,
    i: usize,
    j: usize,
    k: usize,
}

impl QuickSort
{
    pub fn new() -> Self
    {
        Self {
            nodes: Vec::new(),
            i: 0,
            j: 0,
            k: 0
        }
    }
}

impl Sort for QuickSort
{
    fn reset(&mut self)
    {
        self.nodes.clear();
        self.i = 0;
        self.j = 0;
        self.k = 0;
    }
}
