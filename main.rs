use std::io::{self, Read, Write};
use std::mem;

fn calc_zeros(num: u32) -> u32 {
  let size = (mem::size_of::<u32>() * 8) as u32;
  let mut count: u32 = 0;
  let mut mask = (1 << size - 1) as u32;

  for _ in 0..size {
    if num & mask != 0 {
      return count;
    }

    count += 1;
    mask >>= 1;
  }

  return count;
}

fn pause() {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();

  write!(stdout, "Press any key to continue...").unwrap();
  stdout.flush().unwrap();

  stdin.read(&mut [0u8]).unwrap();
}

fn main() {
  println!("Введите число");

  let stdin = io::stdin();
  let mut number_string = String::new();

  stdin
    .read_line(&mut number_string)
    .expect("Ошибка ввода числа");

  let number = number_string
    .trim()
    .parse::<u32>()
    .expect("Ошибка парсинга числа");

  let zeros_count = calc_zeros(number);

  println!("В числе {} нулей в начале: {}", number_string, zeros_count);

  pause();
}