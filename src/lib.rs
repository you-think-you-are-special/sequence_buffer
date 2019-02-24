use std::clone::Clone;


pub struct SequenceBuffer<D: Default + Clone>
{
    buff: Vec<D>,
    numbers_buf: Vec<usize>,
    size: usize,
}

impl<D: Default + Clone> SequenceBuffer<D> {
    pub fn new(size: usize) -> Self {
        let mut buff = Vec::with_capacity(size);
        let mut numbers_buf = Vec::with_capacity(size);

        buff.resize(size, D::default());
        numbers_buf.resize(size + 1, size + 1);

        return Self {
            buff,
            numbers_buf,
            size,
        };
    }

    pub fn insert(&mut self, data: D, number: usize) -> &mut D {
        let index = self.gen_index(number);
        self.buff[index] = data;
        self.numbers_buf[index] = number;
        &mut self.buff[index]
    }

    pub fn get_mut(&mut self, number: usize) -> Option<&mut D> {
        let index = self.gen_index(number);
        if self.exists(number) {
            return Some(&mut self.buff[index]);
        }

        return None;
    }

    pub fn remove(&mut self, number: usize) {
        let index = self.gen_index(number);
        self.buff[index.clone()] = D::default();
        self.numbers_buf[index] = self.size + 1;
    }

    pub fn exists(&self, number: usize) -> bool {
        let index = self.gen_index(number);
        self.numbers_buf[index] == number
    }

    fn gen_index(&self, number: usize) -> usize {
        number % self.size
    }
}


#[cfg(test)]
mod tests {
    use super::SequenceBuffer;

    #[derive(Clone, Default)]
    struct DataStub;

    #[test]
    fn insert_test() {
        let mut buf = SequenceBuffer::new(1);
        buf.insert(DataStub, 543535);
        assert!(buf.exists(543535));
    }

    #[test]
    fn remove_test() {
        let mut buf = SequenceBuffer::new(1);
        buf.insert(DataStub, 2535436);
        buf.remove(2535436);
        assert!(!buf.exists(2535436));
    }
}
