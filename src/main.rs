mod module;

fn main() {
  let purported_cc = String::from("4532095035598175");
  println!("purported_cc: {}", purported_cc);
  let result = module::check_luhn(&purported_cc);
  println!("result: {}", result);
  let purported_cpf = String::from("84546258798");
  println!("purported_cpf: {}", purported_cpf);
  let result = module::check_cpf(&purported_cpf);
  println!("result: {}", result);

  let mut natural_number = 60;
  let mut prime_factor = 2;

  loop {
    if natural_number % prime_factor == 0 {
      println!("{}", prime_factor);
      natural_number = natural_number / prime_factor;
    } else {
      prime_factor += 1;
    }
    if natural_number == 1 {
      break;
    }
  }
}
