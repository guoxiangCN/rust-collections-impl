use std::cell::RefCell;
use std::rc::Rc;

type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

#[derive(Debug)]
pub(crate) struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}

impl<T: PartialOrd> SkipNode<T> {
    pub(crate) fn new(t: T) -> Self {
        SkipNode {
            right: None,
            down: None,
            data: rcc(t),
        }
    }

    pub(crate) fn insert(&mut self, dt: T) -> Option<Rcc<SkipNode<T>>> {
        if let Some(ref rt) = self.right {
            if dt > *(rt.borrow().data.borrow()) {
                return rt.borrow_mut().insert(dt);
            }
        }

        if let Some(ref dw) = self.down {
            return match dw.borrow_mut().insert(dt) {
                Some(child) => match rand::random::<bool>() {
                    true => {},
                    false => None,
                }
                None=>None,
            }
        }

        // should be before right, at bottom node
        let mut nn = SkipNode::new(dt);
        nn.right = self.right.take();
        let res = rcc(nn);
        self.right = Some(res.clone());
        Some(res)
    }
}

pub struct SkipList<T:PartialOrd>(Vec<SkipNode<T>>);

impl<T:PartialOrd> SkipList<T> {
    pub fn new() -> Self {
        SkipList(vec![])
    }
}

#[cfg(test)]
mod tests {}
