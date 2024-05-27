use crate::error::{Result, Error};
use crate::utils::{read_u32, inc_bit};
use crate::node::Node;
use std::collections::HashMap;

const ASCII_MAX: usize = u8::MAX as usize;
const NODE_BYTES_LEN: usize = 5;

#[derive(PartialEq)]
pub enum TravRes<'n> {
    Node(&'n Box<Node>),
    Char(char)
}

pub struct Huffman {
    data: String,
    bytes: Vec<u8>, 
    frequencies: Vec<(char, u32)>,
    nodes: Vec<Box<Node>>,
    size: usize,
    tree: Option<Box<Node>>,
    tree_height: usize,
    lookup: HashMap<char, String>,
    offset_bit: Option<u32>
}

impl Huffman {
    pub fn from_str(data: &str) -> Self {
        let mut map: HashMap<char, Node> = HashMap::new();
        for ch in data.chars() {
            map.entry(ch)
                .and_modify(|n| (*n).freq += 1)
                .or_insert(Node::new_node(ch, 1));
        }
        let mut nodes = map.into_values().map(|n| Box::new(n)).collect::<Vec<Box<Node>>>();
        nodes.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let frequencies = nodes.iter().map(|n| (n.ch, n.freq)).collect::<Vec<(char, u32)>>();
        let size = nodes.len();
        Self {
            data: data.to_string(),
            bytes: Vec::new(),
            frequencies,
            nodes,
            tree: None,
            tree_height: 0,
            size,
            lookup: HashMap::new(),
            offset_bit: None,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut code = Self {
            data: String::new(),
            bytes: bytes.to_vec(),
            frequencies: Vec::new(),
            nodes: Vec::new(),
            tree: None,
            tree_height: 0,
            size: 0,
            lookup: HashMap::new(),
            offset_bit: None
        };
        let _ = code.deserialize();
        code.nodes.sort_by(|a, b| b.partial_cmp(a).unwrap());
        code.frequencies = code.nodes.iter().map(|n| (n.ch, n.freq)).collect::<Vec<(char, u32)>>();
        code.size = code.nodes.len();
        code
    }


    fn build(&mut self) {
        let n = self.size - 1;
        let i = (n - 1) / 2;
        for idx in (0..i+1).rev() {
            self.heapify(idx);
        }
    }   



    fn heapify(&mut self, idx: usize) {
        let mut smallest = idx;
        let left = 2 * idx + 1;
        let right = 2 *idx + 2;

        if left < self.size && self.nodes[left].freq < self.nodes[smallest].freq {
            smallest = left;
        }
        if right < self.size && self.nodes[right].freq < self.nodes[smallest].freq {
            smallest = right;
        }
        if smallest != idx {
            self.nodes.swap(smallest, idx);
            self.heapify(smallest);
        }
    }
        
    fn get_min(&mut self) -> Box<Node> {
        let temp = self.nodes.remove(0);
        self.size -= 1;
        self.heapify(0);
        temp
    }

    fn insert(&mut self, node: Box<Node>) {
        self.size += 1;
        let mut i = self.size - 1;
        while i > 0 && node.freq < self.nodes[(i-1)/2].freq {
            if i == self.nodes.len() {
                self.nodes.push(self.nodes[(i-1)/2].clone());
            } else {
                self.nodes[i] = self.nodes[(i-1)/2].clone();
            }
            i = (i-1)/2;
        }
        if i == self.nodes.len() {
            self.nodes.push(node)
        } else {
            self.nodes[i] = node;
        }
    }

    pub fn codes(&mut self) {
        if let Some(tree) = self.tree.as_ref() {
            let mut path = ['\0'; ASCII_MAX];
            self.prepare_codes(tree.clone(), &mut path, 0);
        }
    }


    fn prepare_codes(&mut self, root: Box<Node>, path: &mut [char; ASCII_MAX], top: usize) {
        if root.left.as_ref().is_some() {
            path[top] = '0';
            let r = root.left.as_ref().unwrap().clone();
            self.prepare_codes(r, path, top + 1);
        }
        if root.right.as_ref().is_some() {
            path[top] = '1';
            let r = root.right.as_ref().unwrap().clone();
            self.prepare_codes(r, path, top + 1);
        }
        if root.is_leave() {
            let mut buf = String::new();
            for i in 0..top {
                buf.push(path[i]);
            }
            self.lookup.insert(root.ch, buf);
            if top+1 > self.tree_height {
                self.tree_height = top+1;
            }
        }
    }

    pub fn create_tree(&mut self) {
        self.build();
        let mut left;
        let mut right;
        let mut top;
        while self.size > 1 {
            left = self.get_min();
            right = self.get_min();
            top = Node::new_node('\0', left.freq + right.freq);
            top.left = Some(left);
            top.right = Some(right);
            self.insert(Box::new(top));
        }
        self.tree = Some(self.get_min())
    }

    fn get_path(&self, ch: char) -> Option<String> {
        self.lookup.get(&ch).cloned()
    }

    pub fn encode(&mut self) -> Result<()> {
        self.serialize();
        let start_idx = self.bytes.len();
        self.bytes.extend_from_slice(&[0,0,0,0]);
        let mut chs = self.data.chars();
        let mut byte = 0u8;
        let mut bit = 0;
        loop {
            if let Some(ch) = chs.next() {
                if let Some(path) = self.get_path(ch) {
                    for dir in path.chars() {
                        if dir == '1' {
                            byte |= 1 << bit;
                        }
                        if inc_bit(&mut bit) {
                            self.bytes.push(byte);
                            byte = 0;
                        }
                    }
                } else {
                    Error::Encoding(format!("Could not get path for {}", ch), String::new());
                }
            } else {
                break;
            }
        }

        //TODO not sure if the offset bit works if 
        //it is a zero??
        if byte != 0 {
            self.bytes.push(byte);
            self.offset_bit = Some(bit as u32);
        }
        if let Some(offset_bit) = self.offset_bit {
            let ob = offset_bit.to_be_bytes();
            for i in 0..4 {
                self.bytes[start_idx + i] = ob[i];
            }
        }
        Ok(())
    }

    pub fn decode(&mut self) -> Result<()>{
        self.create_tree();
        if let Some(root) = self.tree.as_ref() {
            let mut bit = 0;
            let mut node = root;
            let len = self.bytes.len();
            for (i, byte) in self.bytes.iter().enumerate() {
                while bit < 8 {
                    let mut dir = 0;
                    if (byte & (1 << bit)) != 0 {
                        dir = 1;
                    }
                    match node.next(dir)? {
                        TravRes::Char(ch) => {
                            node = root;
                            self.data.push(ch);
                        },
                        TravRes::Node(nd) => {node = &nd; bit += 1;},
                    }
                    if  i == len - 1 {
                        if let Some(offset_bit) = self.offset_bit {
                            if bit == offset_bit as usize {
                                break;
                            }
                        }
                    }
                }
                bit = 0;
            }
           if node.is_leave() {
                self.data.push(node.ch);
           }
        }
        Ok(())
    }
    

    /// serializes the huffman codes into an array of subsets of 1 byte character and 4 bytes
    /// frequencies in order to preserve the priority queue.
    /// The data is preceded by 4 bytes of tree height, and by 4 bytes of huffmann codec data len
    fn serialize(&mut self) {
        let mut bytes = vec![0u8;8];
        for i in 1..=self.tree_height {
            self.current_level(&self.tree, i, &mut bytes);
        }
        let tree_len = bytes.len() - 8;
        let mut idx = 0;
        for b in (self.tree_height as u32).to_be_bytes() {
            bytes[idx] = b;
            idx+=1;
        }
        for b in (tree_len as u32).to_be_bytes() {
            bytes[idx] = b;
            idx += 1;
        }
        self.bytes = bytes;
    }

    fn current_level(&self, root: &Option<Box<Node>>, level: usize, buffer: &mut Vec<u8>) {
        if let Some(root) = root {
            if level == 1 {
                if let Some(flat) = root.flat() {
                    buffer.extend_from_slice(&flat);
                }
            }
            if level > 1 {
                self.current_level(&root.left, level - 1, buffer);
                self.current_level(&root.right, level - 1, buffer);
            }
        }  
    }
    ///deserializes the frequencies from bytes to Vec<Box<Node>>
    fn deserialize(&mut self) -> Result<()> {
        let mut input = &self.bytes[..];
        //skip tree height
        input = &input[4..];
        // get length of frequencies bytes
        let num_freq_bytes = read_u32(&input[..4])? as usize;
        input = &input[4..];

        let mut b_processed: usize = 0;
        //parse nodes
        while b_processed < num_freq_bytes {
            let node = Node::from_flat(&input)?;
            self.nodes.push(Box::new(node));
            input = &input[NODE_BYTES_LEN..];
            b_processed += NODE_BYTES_LEN;
        }
        self.offset_bit = Some(read_u32(&input[..4])?);
        //input = &input[4..];
        self.bytes = (&self.bytes[b_processed+8+4..]).to_vec();
        
        
        Ok(())
    }

   // pub fn bytes(&self) -> Vec<u8> {
   //     self.bytes.to_vec()
   // }
    pub fn read_bytes_into(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(self.bytes.as_slice());
    }

   // pub fn data(&self) -> String {
   //     self.data.clone()
   // }

    pub fn data_to_bytes(&self, buffer: &mut Vec<u8>) {
        for ch in self.data.bytes() {
            buffer.push(ch as u8)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode() {
     let text: &str = "abbcccddddeeeeeffffff";
     let exp = vec![0, 0, 0, 5, 0, 0, 0, 30, 100, 0, 0, 0, 4, 101, 0, 0, 0, 5, 102, 0, 0, 0, 6, 99, 0, 0, 0, 3, 97, 0, 0, 0, 1, 98, 0, 0, 0, 2, 0, 0, 0, 3, 247, 191, 13, 64, 213, 170, 2];
     let mut huf = Huffman::from_str(text);
     huf.create_tree();
     huf.codes();
     let _ = huf.encode();
     assert_eq!(huf.bytes(), exp);
    }

    #[test]
    fn decode() {
     let data = vec![0, 0, 0, 5, 0, 0, 0, 30, 100, 0, 0, 0, 4, 101, 0, 0, 0, 5, 102, 0, 0, 0, 6, 99, 0, 0, 0, 3, 97, 0, 0, 0, 1, 98, 0, 0, 0, 2, 0, 0, 0, 3, 247, 191, 13, 64, 213, 170, 2];
     let expected = String::from("abbcccddddeeeeeffffff");
     let mut huf = Huffman::from_bytes(&data);
     huf.decode();
     assert_eq!(huf.data(), expected);

    }
}




