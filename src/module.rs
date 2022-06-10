pub fn check_luhn(purported_cc: &str) -> bool {
  let n_digits: usize = purported_cc.len();
  let mut sum: u32 = purported_cc[n_digits - 1..].parse::<u32>().unwrap();
  let parity = (n_digits - 2) % 2;

  for i in 0..=n_digits - 2 {
    let mut digit = purported_cc[i..i + 1].parse::<u32>().unwrap();
    digit = if i % 2 == parity { digit * 2 } else { digit };
    digit = if digit > 9 { digit - 9 } else { digit };
    sum += digit;
  }
  sum % 10 == 0
}

pub fn check_cpf(purported_cpf: &str) -> bool {
  let purported_first_digit = purported_cpf[9..10].parse::<i32>().unwrap();
  let purported_second_digit = purported_cpf[10..11].parse::<i32>().unwrap();

  let mut sum_first_digit: i32 = 0;
  let mut sum_second_digit: i32 = 0;
  let mut mult_first_digit: i32 = 10;
  let mut mult_second_digit: i32 = 11;

  for i in 0..9 {
    let digit = purported_cpf[i..i + 1].parse::<i32>().unwrap();
    sum_first_digit = sum_first_digit + digit * mult_first_digit;
    sum_second_digit = sum_second_digit + digit * mult_second_digit;
    mult_first_digit -= 1;
    mult_second_digit -= 1;
  }

  let first_digit = if sum_first_digit % 11 < 2 {
    0
  } else {
    11 - (sum_first_digit % 11)
  };
  sum_second_digit += first_digit * 2;

  let second_digit = if sum_second_digit % 11 < 2 {
    0
  } else {
    11 - (sum_second_digit % 11)
  };
  purported_first_digit == first_digit && purported_second_digit == second_digit
}
