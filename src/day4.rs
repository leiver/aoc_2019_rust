pub fn part1() {
    println!("Running day4 part1!");
    let mut count = 0;
    for password in 147981..691423 {
        let mut mutable_password = password;
        let sixth_digit = mutable_password % 10;
        mutable_password /= 10;
        let fifth_digit = mutable_password % 10;
        mutable_password /= 10;
        let fourth_digit = mutable_password % 10;
        mutable_password /= 10;
        let third_digit = mutable_password % 10;
        mutable_password /= 10;
        let second_digit = mutable_password % 10;
        let first_digit = mutable_password / 10;

        if first_digit <= second_digit &&
            second_digit <= third_digit &&
            third_digit <= fourth_digit &&
            fourth_digit <= fifth_digit &&
            fifth_digit <= sixth_digit {
            if second_digit == third_digit ||
                second_digit == third_digit ||
                third_digit == fourth_digit ||
                fourth_digit == fifth_digit ||
                fifth_digit == sixth_digit {
                count += 1;
            }
        }
    }
    println!("\tcount: {}", count);
    println!("Completed day4 part1!\n");
}

pub fn part2() {
    println!("Running day4 part2!");
    let mut count = 0;
    for password in 147981..691423 {
        let mut mutable_password = password;
        let sixth_digit = mutable_password % 10;
        mutable_password /= 10;
        let fifth_digit = mutable_password % 10;
        mutable_password /= 10;
        let fourth_digit = mutable_password % 10;
        mutable_password /= 10;
        let third_digit = mutable_password % 10;
        mutable_password /= 10;
        let second_digit = mutable_password % 10;
        let first_digit = mutable_password / 10;

        if first_digit <= second_digit &&
            second_digit <= third_digit &&
            third_digit <= fourth_digit &&
            fourth_digit <= fifth_digit &&
            fifth_digit <= sixth_digit {
            if (first_digit == second_digit && second_digit != third_digit) ||
                (first_digit != second_digit && second_digit == third_digit && third_digit != fourth_digit) ||
                (second_digit != third_digit && third_digit == fourth_digit && fourth_digit != fifth_digit) ||
                (third_digit != fourth_digit && fourth_digit == fifth_digit && fifth_digit != sixth_digit) ||
                (fourth_digit != fifth_digit && fifth_digit == sixth_digit) {
                count += 1;
            }
        }
    }
    println!("\tcount: {}", count);
    println!("Completed day4 part2!\n");
}
