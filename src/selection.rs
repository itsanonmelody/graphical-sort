use crate::Sort;

pub struct SelectionSort
{
    i: usize,
    j: usize,
    min: usize,
}

impl SelectionSort
{
    pub fn new() -> Self
    {
        Self {
            i: 0,
            j: 1,
            min: 0,
        }
    }
}

impl Sort for SelectionSort
{
    fn step(&mut self, array: &mut [u32]) -> bool
    {
        if self.i < array.len()-1
        {
            if array[self.j] < array[self.min]
            {
                self.min = self.j;
            }
            
            self.j += 1;
            if self.j >= array.len()
            {
                let temp = array[self.i];
                array[self.i] = array[self.min];
                array[self.min] = temp;

                self.i += 1;
                if self.i >= array.len()+1
                {
                    return true;
                }

                self.min = self.i;
                self.j = self.i+1;
            }
        }

        return self.i >= array.len()-1;
    }

    fn reset(&mut self)
    {
        self.i = 0;
        self.j = 1;
        self.min = 0;
    }

    fn current_index(&self) -> u32
    {
        self.j as u32
    }
}
