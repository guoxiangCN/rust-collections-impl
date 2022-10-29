#[derive(Debug)]
struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn empty(&self) -> bool {
        self.length == 0
    }

    pub fn push_front(&mut self, t: T) {
        let y = self.head.take();
        self.head.replace(Box::new(ListNode { data: t, next: y }));
        self.length += 1;
    }

    pub fn push_back(&mut self, t: T) {
        let newnode = Box::new(ListNode {
            data: t,
            next: None,
        });
        match self.head {
            None => {
                self.head.replace(newnode);
            }
            Some(ref mut x) => {
                let mut cur = x;
                loop {
                    match cur.next {
                        None => {
                            cur.next.replace(newnode);
                            break;
                        }
                        Some(ref mut y) => {
                            cur = y;
                        }
                    }
                }
            }
        }
        self.length += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::new();
        list.push_back(100);
        list.push_back(200);
        list.push_back(300);
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        println!("{:?}", list);
    }
}
