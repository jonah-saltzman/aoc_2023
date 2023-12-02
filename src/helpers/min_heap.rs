pub struct MinHeapIterator<T> {
    heap: MinHeap<T>,
}

impl<T: Ord + Copy> Iterator for MinHeapIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

#[derive(Debug)]
pub struct MinHeap<T> {
    buf: Vec<T>,
}

impl<T> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { buf: vec![] }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.buf.is_empty() {
            None
        } else {
            Some(&self.buf[0])
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

impl<T: Ord + Copy> IntoIterator for MinHeap<T> {
    type Item = T;
    type IntoIter = MinHeapIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        MinHeapIterator { heap: self }
    }
}

macro_rules! parent {
    ($i:expr) => {
        ($i - 1) / 2
    };
}

macro_rules! left_child {
    ($i:expr) => {
        2 * $i + 1
    };
}

macro_rules! right_child {
    ($i:expr) => {
        2 * $i + 2
    };
}

impl<T: Ord + Copy> MinHeap<T> {
    pub fn push(&mut self, val: T) {
        self.buf.push(val);
        let mut i = self.buf.len() - 1;
        if i == 0 {
            return;
        }
        while i != 0 && self.buf[parent!(i)] > self.buf[i] {
            self.buf.swap(i, parent!(i));
            i = parent!(i);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.buf.len() {
            0 => None,
            1 => Some(self.buf.pop().unwrap()),
            _ => {
                let root = self.buf[0];
                self.buf[0] = self.buf.pop().unwrap();

                self.heapify(0);

                Some(root)
            }
        }
    }

    fn heapify(&mut self, i: usize) {
        let l = left_child!(i);
        let r = right_child!(i);
        let mut min_idx = i;
        if l < self.buf.len() && self.buf[l] < self.buf[min_idx] {
            min_idx = l;
        }
        if r < self.buf.len() && self.buf[r] < self.buf[min_idx] {
            min_idx = r;
        }
        if min_idx != i {
            self.buf.swap(i, min_idx);
            self.heapify(min_idx);
        }
    }
}
