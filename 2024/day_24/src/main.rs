#![feature(array_chunks)]
const INPUT: &str = include_str!("../input.txt");

use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    And,
    Or,
    Xor,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    input: [&'a str; 2],
    output: &'a str,
    operation: Operation,
}

fn process_gate<'a>(gate: &Gate<'a>, memory: &mut HashMap<&'a str, u8>) -> Option<()> {
    let a = *memory.get(gate.input[0])?;
    let b = *memory.get(gate.input[1])?;

    let c = match gate.operation {
        Operation::And => a & b,
        Operation::Or => a | b,
        Operation::Xor => a ^ b,
    };
    memory.insert(gate.output, c);

    Some(())
}

fn process_all_gates<'a>(mut gates: Vec<Gate<'a>>, memory: &mut HashMap<&'a str, u8>) -> u128 {
    while !gates.is_empty() {
        gates.retain(|gate| process_gate(gate, memory).is_none());
    }
    let mut bits = 0u128;
    for (name, value) in memory {
        if name.as_bytes()[0] != b'z' {
            continue;
        }
        let i = name[1..].parse::<usize>().unwrap();
        println!("{}, {}", i, value);
        if *value == 0 {
            bits &= !(1 << i);
        } else {
            bits |= 1 << i;
        }
    }

    bits
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Multiple,
    None,
    One(usize),
}

fn merge_deps([ax, ay]: [State; 2], [bx, by]: [State; 2]) -> [State; 2] {
    [[ax, bx], [ay, by]].map(|pair| match pair {
        [State::Multiple, _] | [_, State::Multiple] => State::Multiple,
        [State::One(a), State::One(b)] => {
            if a == b {
                State::One(a)
            } else {
                State::Multiple
            }
        }
        [State::One(a), _] => State::One(a),
        [_, State::One(b)] => State::One(b),
        _ => unreachable!(),
    })
}

fn test_gate<'a>(gate: &Gate<'a>, deps_map: &mut HashMap<&'a str, [State; 2]>) -> Option<()> {
    let a = *deps_map.get(gate.input[0])?;
    let b = *deps_map.get(gate.input[1])?;
    deps_map.insert(gate.output, merge_deps(a, b));

    Some(())
}

fn test_all_gates<'a>(
    mut gates: Vec<Gate<'a>>,
    mut deps_map: HashMap<&'a str, [State; 2]>,
) -> bool {
    while !gates.is_empty() {
        gates.retain(|gate| test_gate(gate, &mut deps_map).is_none());
    }

    for (name, value) in deps_map {
        if name.as_bytes()[0] != b'z' {
            continue;
        }
        let i = name[1..].parse::<usize>().unwrap();
        if value != [State::One(i), State::One(i)] {
            return false;
        }
    }

    true
}

fn swap(i: usize, j: usize, gates: &mut [Gate<'_>]) {
    (gates[i].output, gates[j].output) = (gates[j].output, gates[i].output);
}

fn find_valid_swap<'a>(
    mut gates: Vec<Gate<'a>>,
    deps_map: &HashMap<&'a str, [State; 2]>,
) -> [&'a str; 8] {
    let n = gates.len();

    for a in 0..n {
        for b in a + 1..n {
            swap(a, b, &mut gates);
            for c in b + 1..n {
                for d in c + 1..n {
                    swap(c, d, &mut gates);
                    for e in d + 1..n {
                        for f in e + 1..n {
                            swap(e, f, &mut gates);
                            for g in f + 1..n {
                                for h in g + 1..n {
                                    swap(g, h, &mut gates);
                                    let mut key = [a, b, c, d, e, f, g, h].map(|i| gates[i].output);
                                    key.sort();
                                    println!("{:?}", [a, b, c, d, e, f, g, h]);
                                    if test_all_gates(gates.clone(), deps_map.clone()) {
                                        return key;
                                    }
                                    swap(g, h, &mut gates);
                                }
                            }
                            swap(e, f, &mut gates);
                        }
                    }
                    swap(c, d, &mut gates);
                }
            }
            swap(a, b, &mut gates);
        }
    }

    panic!()
}

fn check_xyop(gate: &Gate<'_>, x: &str, y: &str, operation: Operation) -> bool {
    gate.input == [x, y] && gate.operation == operation
}

fn is_input(gate: &Gate<'_>, input: &str) -> bool {
    gate.input[0] == input || gate.input[1] == input
}

fn is_output(gate: &Gate<'_>, output: &str) -> bool {
    gate.output == output
}

fn check_carry_add_op(gate: &Gate<'_>, carry: &str, basic_add: &str, operation: Operation) -> bool {
    gate.operation == operation && (is_input(gate, carry) || is_input(gate, basic_add))
}

// TODO: how does this work???????
fn find_valid(bit_length: u8, gates: &[Gate<'_>], pairs: usize) -> String {
    let mut swapped = BTreeSet::new();
    let z00 = gates
        .iter()
        .find(|gate| check_xyop(gate, "x00", "y00", Operation::Xor))
        .unwrap();

    if z00.output != "z00" {
        swapped.insert(z00.output.to_string());
    }

    let mut carry: &str = gates
        .iter()
        .find_map(|gate| {
            if check_xyop(gate, "x00", "y00", Operation::And) {
                Some(gate.output)
            } else {
                None
            }
        })
        .unwrap();

    for bit in 1..bit_length {
        // find basic add x_bit XOR y_bit -> ??
        let x = format!("x{bit:02}");
        let y = format!("y{bit:02}");
        let z = format!("z{bit:02}");
        let basic_add = gates
            .iter()
            .find(|gate| check_xyop(gate, &x, &y, Operation::Xor))
            .unwrap()
            .output;
        // check Add (either previous carry, basic add or output can be wrong)

        let add = gates
            .iter()
            .find(|gate| check_carry_add_op(gate, carry, basic_add, Operation::Xor))
            .unwrap();

        if !is_output(add, &z) {
            swapped.insert(z);
            swapped.insert(add.output.to_string());
        }

        if !is_input(add, basic_add) {
            swapped.insert(basic_add.to_string());
        }

        if !is_input(add, carry) {
            swapped.insert(carry.to_string());
        }
        // check basic carry - only output can be wrong
        let basic_carry = gates
            .iter()
            .find(|gate| check_xyop(gate, &x, &y, Operation::And))
            .unwrap()
            .output;
        // check cascade carry (if either previous carry or basic add were wrong, ignore that)
        // if carry was wrong, basic_add could also be wrong... let's ignore that for now
        let cascade_carry = gates
            .iter()
            .find(|gate| check_carry_add_op(gate, carry, basic_add, Operation::And))
            .unwrap();

        if !is_input(cascade_carry, basic_add) {
            swapped.insert(basic_add.to_string());
        }

        if !is_input(cascade_carry, carry) {
            swapped.contains(carry);
        }
        // check carry (basic carry or cascade carry can be wrong)
        let carry_gate = gates
            .iter()
            .find(|gate| check_carry_add_op(gate, cascade_carry.output, basic_carry, Operation::Or))
            .unwrap();

        if !is_input(carry_gate, cascade_carry.output) {
            swapped.insert(cascade_carry.output.to_string());
        }

        if !is_input(carry_gate, basic_carry) {
            swapped.insert(basic_carry.to_string());
        }

        carry = carry_gate.output;
    }

    assert_eq!(pairs * 2, swapped.len());

    let swapped: Vec<_> = swapped.into_iter().collect();
    swapped.join(",")
}

fn main() {
    let (memory, gates) = INPUT.split_once("\n\n").unwrap();
    let bit_length = memory.lines().count() as u8 / 2;

    let mut memory: HashMap<&str, u8> = memory
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            let value = value.parse().unwrap();

            (name, value)
        })
        .collect();

    let gates: Vec<_> = gates
        .lines()
        .map(|line| {
            let tokens: [&str; 5] = line
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let mut input = [tokens[0], tokens[2]];
            input.sort();

            Gate {
                input,
                output: tokens[4],
                operation: Operation::from(tokens[1]),
            }
        })
        .collect();

    println!("{}", process_all_gates(gates.clone(), &mut memory));
    // println!("{}", find_valid_swap(gates, &deps_map).join(","))
    println!("{}", find_valid(bit_length, &gates, 4));
}
