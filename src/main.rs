fn main() {
    println!("Hello, world!");
}

fn eat_primary(formula: &str) -> (i32, &str) {
    if formula.starts_with("(") {
        let (result, formula) = eat_expression(&formula[1..]);
        if formula.starts_with(")") {
            return (result, &formula[1..]);
        } else {
            panic!("end of bracket required")
        }
    }

    let first_letter = formula
        .chars()
        .nth(0)
        .expect("first letter expected")
        .to_digit(10)
        .expect("first letter expected number");

    (first_letter as i32, &formula[1..])
}

#[test]
fn test_eat_primary() {
    assert_eq!(eat_primary("3+5"), (3, "+5"));
    assert_eq!(eat_primary("8-5*4+3"), (8, "-5*4+3"));
}

fn eat_multiplicative(formula: &str) -> (i32, &str) {
    let (mut current_num, mut formula) = eat_primary(&formula);
    loop {
        if formula.starts_with("-")
            || formula.starts_with("+")
            || formula.starts_with("(")
            || formula.starts_with(")")
            || formula.is_empty()
        {
            return (current_num, formula);
        }

        if formula.starts_with("*") {
            let next_num;
            (next_num, formula) = eat_primary(&formula[1..]);
            current_num *= next_num
        }
        if formula.starts_with("/") {
            let next_num;
            (next_num, formula) = eat_primary(&formula[1..]);
            current_num /= next_num
        }
    }
}

#[test]
fn test_eat_multiplicative() {
    assert_eq!(eat_multiplicative("3+7"), (3, "+7"));
    assert_eq!(eat_multiplicative("3*5+7"), (15, "+7"));
    assert_eq!(eat_multiplicative("3*5*7+7"), (105, "+7"));
}

fn eat_expression(formula: &str) -> (i32, &str) {
    let (mut current_num, mut formula) = eat_multiplicative(&formula);
    loop {
        let next_num;
        if formula.starts_with("-") {
            (next_num, formula) = eat_multiplicative(&formula[1..]);
            current_num -= next_num
        } else if formula.starts_with("+") {
            (next_num, formula) = eat_multiplicative(&formula[1..]);
            current_num += next_num
        }
        if formula.is_empty() || formula.starts_with(")") || formula.starts_with("(") {
            return (current_num, &formula);
        }
    }
}

#[test]
fn test_eat_expression() {
    assert_eq!(eat_expression("3-5+7"), (5, ""));
    assert_eq!(eat_expression("3*5+7"), (22, ""));
    assert_eq!(eat_expression("3*5+7*4"), (43, ""));
    assert_eq!(eat_expression("3*5+7*4-2*3*5"), (13, ""));
    assert_eq!(
        eat_expression("3*(5+7-3)*(6-(3-(2+(4))+5))"),
        (3 * (5 + 7 - 3) * (6 - (3 - (2 + (4)) + 5)), "")
    );
}
