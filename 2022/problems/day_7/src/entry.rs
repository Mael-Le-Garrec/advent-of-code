use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug,PartialEq)]
pub enum EntryType {
    Directory,
    File,
}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub type_: EntryType,
    pub size: u32,
    pub parent: RefCell<Weak<Entry>>,      // the node does not own its parent
    pub children: RefCell<Vec<Rc<Entry>>>, // but does own its children
}

impl Entry {
    pub fn new(name: &str, type_: EntryType, size: u32) -> Rc<Entry>
    {
        Rc::new(
            Entry {
                name: name.to_string(),
                type_,
                size,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            }
        )
    }

    pub fn add_child(self: &Rc<Self>, content: Rc<Entry>)
    {
        self.children.borrow_mut().push(Rc::clone(&content));
        *self.children.borrow().last().unwrap().parent.borrow_mut() = Rc::downgrade(self);
    }

    pub fn find_child(self: &Rc<Self>, name: &str) -> Option<Rc<Entry>>
    {
        for child in self.children.borrow_mut().iter_mut(){
            if child.name == name {
                return Some(child.clone());
            }
        }
        return None;
    }

    pub fn contains_child(self: &Rc<Self>, name: &str) -> bool
    {
        match self.find_child(name)
        {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_parent(self: &Rc<Self>) -> Option<Rc<Entry>>
    {
        self.parent.borrow().upgrade()
    }

    pub fn get_size(&self) -> u32
    {
        // If the entry is a file, return its size
        if self.type_ == EntryType::File {
            return self.size;
        }

        // Otherwise, iterate through the children
        let mut sum: u32 = 0;
        for child in self.children.borrow().iter(){
            sum += child.get_size();
        }
        return sum;
    }
}
