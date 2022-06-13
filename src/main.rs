mod module;

fn main() {
  let purported_cc = String::from("4532095035598175");
  println!("purported_cc: {}", purported_cc);
  let result = module::check_luhn(&purported_cc);
  println!(
    "{}",
    match result {
      true => "👍",
      false => "👎",
    }
  );

  let purported_cpf = String::from("84546258798");
  println!("purported_cpf: {}", purported_cpf);
  let result = module::check_cpf(&purported_cpf);
  println!(
    "{}",
    match result {
      true => "👍",
      false => "👎",
    }
  );

  let natural_number = 63;
  println!("natural_number: {}", natural_number);
  let prime_factors = module::prime_factors(natural_number);  
  for prime_factor in prime_factors {
    print!("{} ", prime_factor);
  }
}
