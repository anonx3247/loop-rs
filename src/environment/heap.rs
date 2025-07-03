use crate::ast::value::Value;

#[derive(Clone, Debug)]
pub struct GcBox {
    value: Value,
}

#[derive(Clone, Debug, Default)]
pub struct Heap {
    boxes: Vec<Option<GcBox>>,
}

impl Heap {
    pub fn new() -> Self {
        Self { boxes: Vec::new() }
    }

    pub fn allocate(&mut self, value: Value) -> usize {
        // Try to reuse a free slot
        if let Some((i, slot)) = self.boxes.iter_mut().enumerate().find(|(_, slot)| slot.is_none()) {
            *slot = Some(GcBox { value });
            i
        } else {
            self.boxes.push(Some(GcBox { value }));
            self.boxes.len() - 1
        }
    }

    pub fn deallocate(&mut self, index: usize) {
        if let Some(slot) = self.boxes.get_mut(index) {
            *slot = None;
        }
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.boxes.get(index).and_then(|slot| slot.as_ref().map(|b| &b.value))
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.boxes.get_mut(index).and_then(|slot| slot.as_mut().map(|b| &mut b.value))
    }

    pub fn print(&self) {
        for (i, box_) in self.boxes.iter().enumerate() {
            if box_.is_some() {
                println!("{}: {:?}", i, box_.as_ref().unwrap().value);
            }
        }
    }
}