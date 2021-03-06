use crate::Coder;

pub struct MoveToFront {}

const fn get_list() -> [u8; 256] {
    let mut list = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        list[i] = i as u8;
        i += 1;
    }
    list
}

impl Coder<u8, u8> for MoveToFront {
    fn encode(input: &[u8]) -> Vec<u8> {
        let list = get_list();
        input
            .iter()
            .scan(list, |state, &b| {
                let index = state
                    .iter()
                    .enumerate()
                    .find(|(_, &v)| v == b)
                    .unwrap_or_else(|| panic!("Could not find byte in list."))
                    .0;
                for i in (1..=index).rev() {
                    state.swap(i, i - 1);
                }
                Some(index as u8)
            })
            .collect()
    }

    fn decode(input: &[u8]) -> Vec<u8> {
        let list = get_list();
        input
            .iter()
            .scan(list, |state, &index| {
                let byte = state[index as usize];
                for i in (1..=index as usize).rev() {
                    state.swap(i, i - 1);
                }
                Some(byte)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::movetofront::MoveToFront;
    use crate::Coder;

    #[test]
    fn move_to_front() {
        let input = b"Hello, World!!!!!";
        let encoded = MoveToFront::encode(input);
        println!("{:?}", encoded);
        let output = MoveToFront::decode(&encoded);
        assert_eq!(input, &output[..]);
    }
}
