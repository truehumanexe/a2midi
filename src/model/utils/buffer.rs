pub struct MirrorBuffer<T> {
    index: usize,
    size: usize,
    data: Vec<T>,
}

impl<T: Copy> MirrorBuffer<T> {
    pub fn new(size: usize, default: T) -> Self {
        assert!(size <= usize::MAX / 2);
        Self {
            index: 0,
            size,
            data: vec![default; size * 2],
        }
    }

    pub fn push(&mut self, value: T) {
        self.data[self.index] = value;
        self.data[self.index + self.size] = value;

        self.index = if self.index + 1 < self.size {
            self.index + 1
        } else {
            0
        };
    }

    pub fn push_slice(&mut self, slice: &[T]) {
        for &value in slice {
            self.push(value);
        }
    }

    pub fn get(&self, index: usize) -> T {
        assert!(index < self.size, "index out of bounds");

        self.data[index + self.index]
    }

    pub fn get_slice(&self, start: usize, end: usize) -> &[T] {
        assert!(start <= end && end <= self.size, "invalid range bounds");
        
        let i = self.index + start;
        let e = self.index + end;
        &self.data[i..e]
    }

    pub fn get_copy(&self, start: usize, end: usize) -> Vec<T> {
        self.get_slice(start, end).to_vec()
    }

    pub fn len(&self) -> usize {
        self.size
    }
}