use crate::algorithm::Coder;
use std::boxed::Box;
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::error::Error;
use std::rc::Rc;

const DECODE_ERROR: &str = "input file is not properly encoded";

#[derive(Clone, Debug)]
pub struct Huffman {}

impl Coder<u8, u8> for Huffman {
    type Error = Box<dyn Error + Send + Sync + 'static>;
    fn encode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, Self::Error> {
        let input = input.as_ref();
        let mut output = Vec::new();
        let mut freqs = [0u32; 256];
        input.iter().fold(&mut freqs, |acc, &byte| {
            (*acc)[byte as usize] = (*acc)[byte as usize].saturating_add(1);
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
        Ok(output)
    }

    fn decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, Self::Error> {
        let input = input.as_ref();
        if input.len() < std::mem::size_of::<u32>() * 256 {
            return Err(DECODE_ERROR.into());
        }
        let mut output = Vec::new();
        let mut count: u64 = 0;
        let freqs = input
            .iter()
            .take(std::mem::size_of::<u32>() * 256)
            .scan(([0u8; 4], 0usize), |s, b| {
                s.0[s.1] = *b;
                s.1 += 1;
                if s.1 == 4 {
                    s.1 = 0;
                    let num = u32::from_le_bytes(s.0);
                    count += num as u64;
                    Some(Some(num))
                } else {
                    Some(None)
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        let input = &input[std::mem::size_of::<u32>() * 256..];
        let freqs: [u32; 256] = freqs
            .try_into()
            .map_err(|_| Self::Error::from(DECODE_ERROR))?;
        let (_, root) = create_tree(&freqs);

        let mut current: *const Node = root.as_ptr() as *const _;
        for &v in input {
            let mut v = v;
            for _ in 0..8 {
                // unsafe dereference is okay, because root of Huffman tree lives
                // for the entirety of the loop, so no Nodes can be dropped, since the strong
                // count is at least one.
                // There are no aliasing &mut references to any Node, there are only read
                // accesses.
                let current_ = unsafe { &*current };
                if v & 0x80 == 0 {
                    let left = current_
                        .left
                        .as_ref()
                        .ok_or_else(|| Self::Error::from(DECODE_ERROR))?
                        .as_ptr() as *const _;
                    current = left;
                } else {
                    let right = current_
                        .right
                        .as_ref()
                        .ok_or_else(|| Self::Error::from(DECODE_ERROR))?
                        .as_ptr() as *const _;
                    current = right;
                }
                v <<= 1;

                let current_ = unsafe { &*current };
                if current_.leaf {
                    output.push(current_.input);
                    count -= 1;
                    if count == 0 {
                        return Ok(output);
                    }
                    current = root.as_ptr();
                }
            }
        }
        // check that tree outlives loop.
        let _ = root;
        Ok(output)
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

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq.eq(&other.freq)
    }
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
    let mut nodes = BinaryHeap::new();
    for (i, n_) in leafs.iter_mut().enumerate() {
        let mut n = RefCell::borrow_mut(n_);
        n.leaf = true;
        n.input = i as u8;
        n.freq = freqs[i as usize];
        drop(n);
        nodes.push(Reverse(n_.clone()));
    }

    while nodes.len() > 1 {
        let n1 = nodes.pop().unwrap().0;
        let n2 = nodes.pop().unwrap().0;
        let parent = Node {
            leaf: false,
            input: 0,
            freq: RefCell::borrow(&n1)
                .freq
                .saturating_add(RefCell::borrow(&n2).freq),
            mask: 0,
            len: 0,
            left: Some(n1.clone()),
            right: Some(n2.clone()),
        };
        nodes.push(Reverse(Rc::new(RefCell::new(parent))))
    }

    let root = nodes.pop().unwrap().0;

    let mut queue = Vec::with_capacity(256);
    queue.push(root.clone());

    while !queue.is_empty() {
        let n = if let Some(x) = queue.pop() {
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
            queue.push(right.clone());
            queue.push(left.clone());
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
    use crate::algorithm::huffman::create_tree;
    use crate::Coder;
    use crate::Huffman;

    #[test]
    fn create_codes() {
        let mut freqs: [u32; 256] = [0; 256];
        freqs[3] = 3000;
        freqs[40] = 20;

        create_tree(&freqs);
    }

    #[test]
    fn roundtrip() {
        let input = [1, 2, 3, 3, 3, 3, 4, 8, 19];
        let output = Huffman::decode(Huffman::encode(input).unwrap()).unwrap();
        assert_eq!(Vec::from(input), output);
    }
}
