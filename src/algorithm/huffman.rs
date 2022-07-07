use crate::algorithm::Coder;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Huffman {}

impl Coder<u8, u8> for Huffman {
    fn encode(input: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        let mut freqs = [0u32; 256];
        input.iter().fold(&mut freqs, |acc, &byte| {
            (*acc)[byte as usize] += 1;
            acc
        });
        freqs
            .into_iter()
            .flat_map(|int| int.to_le_bytes())
            .for_each(|b| output.push(b));

        let (leafs, _) = create_tree(&freqs);
        let mut next = 0u8;
        let mut filled = 0;
        for &v in input.iter() {
            let leaf = RefCell::borrow(&leafs[v as usize]);
            let len = leaf.len;
            let mut code = leaf.mask << (32 - len);

            for _ in 0..len {
                next <<= 1;
                if code & 0x80000000 > 0 {
                    next += 1;
                }
                filled += 1;
                code <<= 1;
                if filled == 8 {
                    output.push(next);
                    next = 0;
                    filled = 0;
                }
            }
        }
        if filled != 0 {
            next <<= 8 - filled;
            output.push(next);
        }
        output
    }

    fn decode(input: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        let mut count = 0;
        let freqs = input
            .iter()
            .take(std::mem::size_of::<u32>() * 256)
            .scan(([0u8; 4], 0usize), |s, b| {
                s.0[s.1] = *b;
                s.1 += 1;
                if s.1 == 4 {
                    s.1 = 0;
                    let num = u32::from_le_bytes(s.0);
                    count += num;
                    Some(Some(num))
                } else {
                    Some(None)
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        println!("{:?}", freqs.len());
        let input = &input[std::mem::size_of::<u32>() * 256..];
        let freqs: [u32; 256] = freqs.try_into().unwrap();
        let (_, root) = create_tree(&freqs);

        let mut current = root.clone();
        for &v in input {
            let mut v = v;
            for _ in 0..8 {
                if v & 0x80 == 0 {
                    let left = RefCell::borrow(&current).left.clone().unwrap();
                    current = left;
                } else {
                    let right = RefCell::borrow(&current).right.clone().unwrap();
                    current = right;
                }
                v <<= 1;

                if RefCell::borrow(&current).leaf {
                    output.push(RefCell::borrow(&current).input);
                    count -= 1;
                    if count == 0 {
                        return output;
                    }
                    current = root.clone();
                }
            }
        }
        output
    }
}

struct Node {
    leaf: bool,
    input: u8,
    freq: u32,
    mask: u32,
    len: usize,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new() -> Self {
        Self {
            leaf: false,
            input: 0,
            freq: 0,
            mask: 0,
            len: 0,
            left: None,
            right: None,
        }
    }
}

fn create_tree(freqs: &[u32; 256]) -> ([Rc<RefCell<Node>>; 256], Rc<RefCell<Node>>) {
    let mut leafs: Vec<Rc<RefCell<Node>>> = (0..256)
        .map(|_| Rc::new(RefCell::new(Node::new())))
        .collect::<Vec<_>>();
    let mut nodes = Vec::new();
    for (i, n_) in leafs.iter_mut().enumerate() {
        let mut n = RefCell::borrow_mut(n_);
        n.leaf = true;
        n.input = i as u8;
        n.freq = freqs[i as usize];
        nodes.push(n_.clone());
    }

    while nodes.len() > 1 {
        let extract_min = |nodes: &Vec<Rc<RefCell<Node>>>| {
            nodes
                .iter()
                .enumerate()
                .min_by(|n1, n2| RefCell::borrow(n1.1).freq.cmp(&RefCell::borrow(n2.1).freq))
                .unwrap_or_else(|| panic!("Could not extract min"))
                .0
        };
        let min = extract_min(&nodes);
        let n1 = nodes.swap_remove(min);
        let min = extract_min(&nodes);
        let n2 = nodes.swap_remove(min);
        let parent = Node {
            leaf: false,
            input: 0,
            freq: RefCell::borrow(&n1).freq + RefCell::borrow(&n2).freq,
            mask: 0,
            len: 0,
            left: Some(n1.clone()),
            right: Some(n2.clone()),
        };
        nodes.push(Rc::new(RefCell::new(parent)))
    }

    let root = nodes[0].clone();

    while !nodes.is_empty() {
        let n = if let Some(x) = nodes.pop() {
            x
        } else {
            panic!()
        };
        let n = RefCell::borrow_mut(&n);
        if !n.leaf {
            let left = n.left.as_ref().unwrap_or_else(|| panic!());
            let right = n.right.as_ref().unwrap_or_else(|| panic!());
            {
                let mut left_r = RefCell::borrow_mut(left);
                let mut right_r = RefCell::borrow_mut(right);
                left_r.mask = n.mask << 1;
                left_r.len = n.len + 1;
                right_r.mask = (n.mask << 1) + 1;
                right_r.len = n.len + 1;
            }
            nodes.push(right.clone());
            nodes.push(left.clone());
        }
    }
    #[cfg(feature = "verbose")]
    for i in 0..256 {
        if freqs[i] > 0 {
            let leaf = RefCell::borrow(&leafs[i]);
            println!(
                "[{}] ({}x): {:0width$b}",
                char::from(i as u8),
                freqs[i],
                leaf.mask,
                width = leaf.len
            )
        }
    }
    (leafs.try_into().unwrap_or_else(|_| panic!()), root)
}
#[cfg(test)]
mod tests {
    use crate::algorithm::huffman::{create_tree, Huffman};
    use crate::algorithm::Coder;
    use std::path::Path;

    #[test]
    fn create_codes() {
        let mut freqs: [u32; 256] = [0; 256];
        freqs[3] = 3000;
        freqs[40] = 20;

        create_tree(&freqs);
    }
}
