// Some advanced stuff is happening in this file. A goal was to create a simple linked list, that would just pass the tests.
// We achieve this by using single struct called SimpleLinkedList without defining additional Node struct, that is for the DRY purpose.

use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    // SimpleLinkedList will contain a single data field of generic type and a link to next list node, wrapped in Box container to be placed on the heap,
    // and wrapped in Option enum to be able to contain None value.
    data: Option<T>,
    head: Option<Box<SimpleLinkedList<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        // New list is initialized with no data and no link to the next node.
        Self {
            data: None,
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        // Here we check for the data field, if there is Some(data) we check for the next node to aquire length
        match &self.data {
            Some(_) => match &self.head {
                Some(head) => head.len() + 1,
                None => 1,
            },
            None => 0,
        }
    }

    pub fn push(&mut self, _element: T) {
        // To perform a push operation we have to create a next node of the same type and push .data and .head fields into it.
        // This allows us to keep &mut reference to the object. I did not find a way of doing it without copying data field
        // due to "cannot move out of self" compiler error.
        if self.data.is_some() {
            self.head = Some(Box::new(SimpleLinkedList {
                data: self.data.take(),
                head: self.head.take(),
            }));
        }
        self.data = Some(_element);
    }

    pub fn pop(&mut self) -> Option<T> {
        // First we take out data from the Option enum, leaving None there. Then if there is next node we just switch the reference to point to the next node.
        let data = self.data.take();
        if let Some(head) = self.head.take() {
            // "*" in *self allows us to point self to the previous link in the list, while destroing the first one.
            // "*" in *head dereferences Box container.
            *self = *head;
        }
        data
    }

    pub fn peek(&self) -> Option<&T> {
        // This is easy just using Option.as_ref().
        self.data.as_ref()
    }

    pub fn rev(self) -> Self {
        // This is a function for a reversal of a linked list. For that we change head nodes to point backwards and then return a node that was previously the last.
        if self.head.is_none() {
            return self;
        }
        let mut opt_previous_node = None; // opt_previous node is a Box container wrapped in Option enum
        let mut current_node = Box::new(self); // current_node and next_node are in Box containers
        let mut next_node;
        while current_node.head.is_some() {
            next_node = current_node.head.unwrap();
            current_node.head = opt_previous_node;
            opt_previous_node = Some(current_node);
            current_node = next_node;
        }
        current_node.head = opt_previous_node;
        *current_node
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        // This could probably be done faster, but we already had spent enough rime on push() method
        let mut list = SimpleLinkedList::new();
        for i in _iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        // This is a funky function at the moment, but it passes the test
        let mut v = Vec::new();
        let mut current_node = self;
        let mut next_node;
        while current_node.head.is_some() {
            v.push(current_node.data.unwrap());
            next_node = current_node.head.unwrap();
            current_node = *next_node;
        }
        v.push(current_node.data.unwrap());
        v.reverse();
        v
    }
}
