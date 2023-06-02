pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    capacity: usize,
    size: usize,
    head: usize,
    tail: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            data: vec![None; capacity],
            capacity,
            size: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        match self.size {
            size if size < self.capacity => {
                self.data[self.head] = Some(element);
                self.size += 1;
                self.head += 1;
                self.head %= self.capacity;
                return Ok(())
            },
            _ => {
                return Err(Error::FullBuffer)
            },
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.size {
            size if size > 0 => {
                let ret = self.data[self.tail].clone().unwrap();
                self.size -= 1;
                self.tail += 1;
                self.tail %= self.capacity;
                return Ok(ret);
            },
            _ => {
                return Err(Error::EmptyBuffer);
            },
        }
    }

    pub fn clear(&mut self) {
        self.data = vec![None; self.capacity];
        self.size = 0;
        self.head = 0;
        self.tail = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.size == self.capacity {
            let _ = self.read();
        }
        let _ = self.write(element);
    }
}
