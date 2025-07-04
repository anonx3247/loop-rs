use crate::ast::value::Value;

#[derive(Clone, Debug)]
pub struct GcBox<T> {
    value: T,
}

pub trait Heap<T : std::fmt::Debug> {

    fn elements(&self) -> Vec<Option<GcBox<T>>>;

    fn allocate(&mut self, value: T) -> usize;

    fn deallocate(&mut self, index: usize);

    fn get(&self, index: usize) -> Option<&T>;

    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    fn print(&self) {
        for (i, box_) in self.elements().iter().enumerate() {
            if box_.is_some() {
                println!("{}: {:?}", i, box_.as_ref().unwrap().value);
            }
        }
    }
}


#[derive(Clone, Debug, Default)]
pub struct VariableHeap {
    boxes: Vec<Option<GcBox<Value>>>,
}

impl VariableHeap {
    pub fn new() -> Self {
        Self { boxes: Vec::new() }
    }    
}

impl Heap<Value> for VariableHeap {

    fn elements(&self) -> Vec<Option<GcBox<Value>>> {
        self.boxes.clone()
    }

    fn allocate(&mut self, value: Value) -> usize {
        // Try to reuse a free slot
        if let Some((i, slot)) = self.boxes.iter_mut().enumerate().find(|(_, slot)| slot.is_none()) {
            *slot = Some(GcBox { value });
            i
        } else {
            self.boxes.push(Some(GcBox { value }));
            self.boxes.len() - 1
        }
    }

    fn deallocate(&mut self, index: usize) {
        if let Some(slot) = self.boxes.get_mut(index) {
            *slot = None;
        }
    }

    fn get(&self, index: usize) -> Option<&Value> {
        self.boxes.get(index).and_then(|slot| slot.as_ref().map(|b| &b.value))
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.boxes.get_mut(index).and_then(|slot| slot.as_mut().map(|b| &mut b.value))
    }
}

/*
#[derive(Debug, Default)]
pub struct FunctionHeap {
    boxes: Vec<Option<GcBox<FnDeclaration>>>,
}

impl FunctionHeap {
    pub fn new() -> Self {
        Self { boxes: Vec::new() }
    }    
}

impl Heap<FnDeclaration> for FunctionHeap {

    fn elements(&self) -> Vec<Option<GcBox<FnDeclaration>>> {
        self.boxes.iter().map(|slot| match slot {
            None => None,
            Some(box_) => Some(GcBox { value: box_.value.clone_element() })
        }).collect()
    }

    fn allocate(&mut self, value: FnDeclaration) -> usize {
        // Try to reuse a free slot
        if let Some((i, slot)) = self.boxes.iter_mut().enumerate().find(|(_, slot)| slot.is_none()) {
            *slot = Some(GcBox { value });
            i
        } else {
            self.boxes.push(Some(GcBox { value }));
            self.boxes.len() - 1
        }
    }

    fn deallocate(&mut self, index: usize) {
        if let Some(slot) = self.boxes.get_mut(index) {
            *slot = None;
        }
    }

    fn get(&self, index: usize) -> Option<&FnDeclaration> {
        self.boxes.get(index).and_then(|slot| slot.as_ref().map(|b| &b.value))
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut FnDeclaration> {
        self.boxes.get_mut(index).and_then(|slot| slot.as_mut().map(|b| &mut b.value))
    }
}
*/