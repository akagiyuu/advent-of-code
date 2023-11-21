use std::fmt::Write;

pub fn calculate_signal_stregth(input: &str) -> i32 {
    let mut signal_stregth = 0;
    let mut register = 1;
    let mut pivot_cycle = 20;
    let mut current_cycle = 1;
    for instruction in input.lines() {
        match instruction.as_bytes()[0] {
            b'n' => {
                current_cycle += 1;
                if current_cycle == pivot_cycle {
                    signal_stregth += register * pivot_cycle;
                    pivot_cycle += 40;
                }
            }
            b'a' => {
                current_cycle += 1;
                if current_cycle == pivot_cycle {
                    signal_stregth += register * pivot_cycle;
                    pivot_cycle += 40;
                }

                register += instruction[5..].parse::<i32>().unwrap();

                current_cycle += 1;
                if current_cycle == pivot_cycle {
                    signal_stregth += register * pivot_cycle;
                    pivot_cycle += 40;
                }
            }
            _ => {}
        }
    }
    signal_stregth
}

type CRT = [char; 240];

fn render_crt(crt: &CRT) -> String {
    let mut result = String::with_capacity(crt.len() + crt.len() / 40);
    crt.iter().array_chunks::<40>().for_each(|line| {
        for pixel in line {
            write!(result, "{}", pixel).unwrap();
        }
        writeln!(result).unwrap();
    });
    result
}

fn get_pixel(pixel_index: i32, sprite_middle: i32) -> char {
    let pixel_index = pixel_index % 40;
    if sprite_middle - 1 <= pixel_index && pixel_index <= sprite_middle + 1 {
        '#'
    } else {
        '.'
    }
}

pub fn render(input: &str) -> String {
    let mut register = 1;
    let mut current_cycle = 0i32;
    let mut current_sprite_middle = 1i32;
    let mut crt: CRT = ['.'; 240];

    for instruction in input.lines() {
        match instruction.as_bytes()[0] {
            b'n' => {
                crt[current_cycle as usize] = get_pixel(current_cycle, current_sprite_middle);
                current_cycle += 1;
            }
            b'a' => {
                crt[current_cycle as usize] = get_pixel(current_cycle, current_sprite_middle);
                current_cycle += 1;

                crt[current_cycle as usize] = get_pixel(current_cycle, current_sprite_middle);
                current_cycle += 1;

                register += instruction[5..].parse::<i32>().unwrap();
                current_sprite_middle = register;
            }
            _ => {}
        }
    }

    render_crt(&crt)
}
