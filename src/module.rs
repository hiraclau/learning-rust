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
    sum_first_digit += digit * mult_first_digit;
    sum_second_digit += digit * mult_second_digit;
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

pub fn match_cpf_state(cpf: &str) -> &str {
  let ninth_digit = &cpf[8..9];
  let state = match ninth_digit {
    "0" => "Rio Grande do Sul",
    "1" => "Distrito Federal, Goiás, Mato Grosso, Mato Grosso do Sul e Tocantins",
    "2" => "Amazonas, Pará, Roraima, Amapá, Acre e Rondônia",
    "3" => "Ceará, Maranhão e Piauí",
    "4" => "Paraíba, Pernambuco, Alagoas e Rio Grande do Norte",
    "5" => "Bahia e Sergipe",
    "6" => "Minas Gerais",
    "7" => "Rio de Janeiro e Espírito Santo",
    "8" => "São Paulo",
    "9" => "Paraná e Santa Catarina",
    _ => "Não identificado",
  };
  state
}

pub fn prime_factors(natural_number: i32) -> Vec<i32> {
  let mut prime_factors: Vec<i32> = Vec::new();
  let mut prime_factor: i32 = 2;
  let mut natural_number: i32 = natural_number;
  loop {
    if natural_number % prime_factor == 0 {
      natural_number = natural_number / prime_factor;
      prime_factors.push(prime_factor);
    } else {
      prime_factor += 1;
    }
    if natural_number == 1 {
      break;
    }
  }
  prime_factors
}
