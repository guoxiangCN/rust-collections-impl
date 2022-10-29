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

    #[inline]
    pub fn length(&self) -> usize {
        self.length
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
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

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head {
            None => {
                assert_eq!(self.length, 0);
                return None;
            }
            Some(ref mut h) => {
                let next = h.next.take();
                let save_head = self.head.take();
                self.head = next;
                self.length -= 1;
                return save_head.map(|x| (*x).data);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.head {
            None => {
                assert_eq!(self.length, 0);
                return None;
            }
            Some(ref mut x) => {
                todo!()
            }
        }
        None
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        match self.head {
            None => {
                assert_eq!(0, self.length);
                return None;
            }
            Some(_) => {
                let mut idx = index;
                let mut cur: &Option<Box<ListNode<T>>> = &self.head;
                loop {
                    match cur {
                        None => {
                            return None;
                        }
                        Some(ref x) => {
                            cur = &x.next;
                            if idx == 0 {
                                return Some(&x.data);
                            }
                            idx -= 1;
                        }
                    }
                }
            }
        }
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

        //1 2 3 100 200 300
        assert_eq!(Some(&1), list.get(0));
        assert_eq!(Some(&2), list.get(1));
        assert_eq!(Some(&3), list.get(2));
        assert_eq!(Some(&100), list.get(3));
        assert_eq!(Some(&200), list.get(4));
        assert_eq!(Some(&300), list.get(5));
        assert_eq!(None, list.get(6));

        assert_eq!(Some(1), list.pop_front());
        assert_eq!(Some(2), list.pop_front());
        assert_eq!(Some(3), list.pop_front());
        assert_eq!(Some(100), list.pop_front());
        assert_eq!(Some(200), list.pop_front());
        assert_eq!(Some(300), list.pop_front());
        assert_eq!(0, list.length());
        assert!(list.is_empty());
        println!("{:?}", list);
    }

    #[test]
    fn test_box() {
        struct A {
            data: String,
            data2: i32,
        }

        impl A {
            fn test(&self) {}
        }

        let mut a = Box::new(A {
            data: "data".to_owned(),
            data2: 123,
        });
        let b = (*a).data; // 为什么这里可以move，下面一行不可以? 编译器不也是调用deref/deref_mut的么
                           // let b = a.deref_mut().data;  //
        a.data2;
    }
}
