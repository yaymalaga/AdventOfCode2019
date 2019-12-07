fn main() {
    let range_top = 240_920;
    let range_bottom = 789_857;

    let mut simple_counter = 0;
    let mut counter = 0;
    for password in range_top..range_bottom {
        let mut number = password;
        let mut last_digit = 10;
        let mut simple_adjacent = false;
        let mut adjacent = false;
        let mut increase = true;
        let mut digit_times: Vec<usize> = vec![0;10];

        // We are iterating from right to left
        while number != 0 {
            let digit = number % 10;
            if digit == last_digit {
                digit_times[digit] += 1;
            }
            if digit > last_digit {
                increase = false;
            }
            number /= 10;
            last_digit = digit;
        }

        for item in digit_times.into_iter() {
            if item >= 1 {
                simple_adjacent = true;
            }
            if item == 1 {
                adjacent = true;
                break;
            }
        }
        
        if increase {
            if adjacent {
                counter += 1;
            }
            if simple_adjacent {
                simple_counter += 1;
            }
        }
    }

    println!("Part 1: {}", simple_counter);
    println!("Part 2: {}", counter);
}
