use std::collections::HashMap;

#[derive(Debug)]
pub struct AllocatedLookupMap<T> {
    pub map: HashMap<usize, T>,
    next_id: usize,
}

impl<T> Default for AllocatedLookupMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AllocatedLookupMap<T> {
    pub fn new() -> AllocatedLookupMap<T> {
        Self {
            map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn alloc(&mut self, item: T) -> usize {
        let id = self.next_id;
        self.map.insert(id, item);
        self.next_id += 1;
        id
    }

    pub fn get(&self, id: &usize) -> Option<&T> {
        self.map.get(id)
    }

    pub fn get_mut(&mut self, id: &usize) -> Option<&mut T> {
        self.map.get_mut(id)
    }
}
