use crate::utils::read_u32;
use crate::huffman::TravRes;
use crate::error::{Result, Error};
use std::cmp::Ordering;


#[derive(Clone, Ord, Eq)]
pub struct Node {
    pub ch: char,
    pub freq: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}



impl Node {
    pub fn new_node(ch: char, freq: u32) -> Self {
        Self {
            ch,
            freq,
            left: None,
            right: None,
        }
    }
    pub fn from_flat(data: &[u8]) -> Result<Self> {
        let ch = data[0] as char;
        let freq = read_u32(&data[1..5])?;
        Ok(Self::new_node(ch, freq))
    }

    pub fn is_leave(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn flat(&self) -> Option<Vec<u8>> {
        if self.is_leave() {
        let mut buf = Vec::with_capacity(5);
        buf.push(self.ch as u8);
        buf.extend_from_slice(&self.freq.to_be_bytes());
        Some(buf)
        } else {
            None
        }
    }

    pub fn next(&self, dir: u8) -> Result<TravRes> {
        if self.is_leave() {
            return Ok(TravRes::Char(self.ch))
        }
        if dir == 0 {
            if let Some(left) = self.left.as_ref() {
                    return Ok(TravRes::Node(left))
            } 
        }
        if dir == 1 {
            if let Some(right) = self.right.as_ref() {
                return Ok(TravRes::Node(right))
            } 
        }
        Err(Error::Heapify("Unreachable, node is no leave or 'dir' parameter is not '0' or '1'".to_string(), String::new()))
    }

}



impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other {
            Some(Ordering::Less)
        } else if self > other {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
    fn lt(&self, other: &Self) -> bool {
        self.freq < other.freq
    }
    fn gt(&self, other: &Self) -> bool {
        self.freq > other.freq
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ordering() {
        let node1 = Node::new_node('a', 1);
        let node2 = Node::new_node('b', 2);
        let node3 = Node::new_node('d', 3);
        let node4 = Node::new_node('d', 3);
        assert!(node1 != node2);
        assert!(node3 == node4);
        assert!(node1 < node2);
        assert!(node2 > node1);
    }


    #[test]
    fn flatten() {
        let node = Node::new_node('a', 1);
        let flattened = vec![97, 0, 0, 0, 1];
        assert_eq!(node.flat(), Some(flattened.clone()));
        let from = Node::from_flat(&flattened).unwrap();
        assert_eq!(from.ch, 'a');
        assert_eq!(from.freq, 1);

    }


    #[test]
    fn is_a_leave() {
        let node = Node::new_node('a', 1);
        assert!(node.is_leave());
        let mut node_non_leave = Node::new_node('b', 2);
        node_non_leave.left = Some(Box::new(node));
        assert!(!node_non_leave.is_leave());
    }

    #[test]
    fn traverse() {
        let mut node1 = Node::new_node('a', 1);
        let mut node3 = Node::new_node('d', 3);
        let mut node_a = Node::new_node('\0', 4);
        node_a.left = Some(Box::new(node1.clone()));
        node_a.right = Some(Box::new(node3.clone()));

        assert!(node_a.next(0).unwrap() == TravRes::Node(&Box::new(node1)));
        assert!(node_a.next(1).unwrap() == TravRes::Node(&Box::new(node3.clone())));
        assert!(node3.next(0).unwrap()  == TravRes::Char('d'));
        assert!(node_a.next(2).is_err());

    }
}
