use crate::Sort;

pub struct BubbleSort
{
    i: usize,
    j: usize,
}

impl BubbleSort
{
    pub fn new() -> Self
    {
        Self {
            i: usize::MAX,
            j: 0,
        }
    }
}

impl Sort for BubbleSort
{
    fn step(&mut self, array: &mut [u32]) -> bool
    {
        if self.i >= array.len()
        {
            self.i = array.len()-1;
        }

        if self.i != 0
        {
            if array[self.j] > array[self.j+1]
            {
                let temp = array[self.j];
                array[self.j] = array[self.j+1];
                array[self.j+1] = temp;
            }

            self.j += 1;
            if self.j >= self.i
            {
                self.i -= 1;
                if self.i == 0
                {
                    return true;
                }

                self.j = 0;
            }
        }

        return self.i == 0;
    }

    fn reset(&mut self)
    {
        self.i = usize::MAX;
        self.j = 0;
    }

    fn current_index(&self) -> u32
    {
        self.j as u32
    }
}
