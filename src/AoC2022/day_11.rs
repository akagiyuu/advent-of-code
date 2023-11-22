use std::str::FromStr;

use evalexpr::eval_int;

struct Test {
    divide_by: usize,
    true_monkey: usize,
    false_monkey: usize,
}
impl Test {
    fn run(&self, item: usize) -> usize {
        if item % self.divide_by == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

struct Operation {
    expression: String,
}
impl Operation {
    fn run(&self, item: usize) -> usize {
        eval_int(&self.expression.replace("old", &item.to_string())).unwrap() as usize
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    number_of_inspection: usize,
}
impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();

        let items = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let expression = lines
            .next()
            .unwrap()
            .split_once("= ")
            .unwrap()
            .1
            .to_string();

        let divide_by = lines
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let true_monkey = lines
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let false_monkey = lines
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        Ok(Self {
            items,
            operation: Operation { expression },
            test: Test {
                divide_by,
                true_monkey,
                false_monkey,
            },
            number_of_inspection: 0,
        })
    }
}

pub fn calculate_monkey_bussiness_level(input: &str, round: usize, divide_by: usize) -> usize {
    let mut monkeys = input
        .split("\n\n")
        .map(|monkey| Monkey::from_str(monkey).unwrap())
        .collect::<Vec<_>>();

    let mut modulus = 1;
    for monkey in &monkeys {
        modulus *= monkey.test.divide_by;
    }

    for _ in 0..round {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let new_worry_level =
                    monkeys[i].operation.run(monkeys[i].items[j]) / divide_by % modulus;
                let new_owner = monkeys[i].test.run(new_worry_level);
                monkeys[new_owner].items.push(new_worry_level);
            }

            monkeys[i].number_of_inspection += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }

    monkeys.sort_by(|a, b| b.number_of_inspection.cmp(&a.number_of_inspection));
    monkeys[0].number_of_inspection * monkeys[1].number_of_inspection
}
