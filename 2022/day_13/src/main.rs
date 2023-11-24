const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Number(usize),
}
impl TryFrom<&[u8]> for Packet {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut bytes = value.iter();
        let mut result = Self::List(vec![]);
        let mut temp = String::new();

        while let Some(c) = bytes.next() {
            match c {
                b'[' => {
                    temp.clear();
                    let mut depth = 1;

                    while depth > 0 {
                        let c = bytes.next().unwrap();
                        temp.push(*c as char);
                        match c {
                            b'[' => depth += 1,
                            b']' => depth -= 1,
                            _ => {}
                        }
                    }
                    if let Packet::List(data) = &mut result {
                        data.push(Self::try_from(temp[..temp.len() - 1].as_bytes())?);
                    }
                }
                c if c.is_ascii_digit() => {
                    temp.clear();
                    temp.push(*c as char);
                    for &c in bytes.by_ref() {
                        if c == b',' {
                            break;
                        }
                        temp.push(c as char);
                    }
                    if let Packet::List(data) = &mut result {
                        data.push(Self::Number(temp.parse()?))
                    }
                }
                _ => {}
            }
        }
        Ok(result)
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
            (Packet::List(vec_a), Packet::List(vec_b)) => {
                for (a, b) in vec_a.iter().zip(vec_b.iter()) {
                    let compare_result = a.cmp(b);
                    if compare_result != std::cmp::Ordering::Equal {
                        return compare_result;
                    }
                }
                vec_a.len().cmp(&vec_b.len())
            }
            (Packet::List(_), Packet::Number(b)) => {
                self.cmp(&Packet::List(vec![Packet::Number(*b)]))
            }
            (Packet::Number(a), Packet::List(_)) => {
                Packet::List(vec![Packet::Number(*a)]).cmp(other)
            }
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn main() {
    let index_sum = INPUT
        .trim()
        .split("\n\n")
        .map(|pair| {
            let (a, b) = pair.split_once('\n').unwrap();
            (
                Packet::try_from(a[1..a.len() - 1].as_bytes()).unwrap(),
                Packet::try_from(b[1..b.len() - 1].as_bytes()).unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .fold(0, |result, (i, _)| result + i + 1);
    println!("{}", index_sum);


    let divider_a = Packet::try_from("[2]".as_bytes()).unwrap();
    let divider_b = Packet::try_from("[6]".as_bytes()).unwrap();

    let mut packets: Vec<Packet> = vec![divider_a.clone(), divider_b.clone()];
    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }
        packets.push(Packet::try_from(line.as_bytes()).unwrap());
    }
    packets.sort();
    let decoder_key = packets
        .iter()
        .enumerate()
        .filter(|&(_, x)| x == &divider_a || x == &divider_b)
        .fold(1, |result, (i, _)| result * (i + 1));
    println!("{}", decoder_key);
}
