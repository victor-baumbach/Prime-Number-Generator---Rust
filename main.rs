fn main() {
  println!("{:#?} ", sieve_of_eratosthenes(2, 100, 110));
  /* for number in find_prime_numbers(100000) {
    print!("{}, ", number);
  } */
}

fn find_prime_numbers(end_value: u64) -> Vec<u64> {
    let mut prime_numbers = vec![2u64];
    let mut n = 3u64;
    while n <= end_value as u64 {
        let mut is_prime = true;
        for number in prime_numbers.iter() {
          if n % number == 0 {
            is_prime = false;
            break;
          }
        }
        if is_prime {
            prime_numbers.push(n);
        }
        n += 2;
    }
    return prime_numbers;
}

fn find_amount_prime_numbers(amount: usize) -> Vec<u64> {
  let mut prime_numbers = vec![2u64];
  let mut n = 3u64;
  while prime_numbers.len() < amount {
    let mut is_prime = true;
    for number in prime_numbers.iter() {
      if n % number == 0 {
        is_prime = false;
        break;
      }
    }
    if is_prime {
      prime_numbers.push(n);
    }
    n += 2;
  }
  return prime_numbers;
}

/* fn is_prime (number: u64) -> bool {
  match number {
    2|3|5 -> return true,
    0|1|4|6 -> return false,
  }
  let mut i = 3;
  match number % 10 {
    0|2|4|5|6|8 -> return false,
    1|3|7|9 -> while i < number {
      if number % i == 0 { return false; }
      match i % 10 {
        3 -> i += 4,
        1|7|9 -> i += 2,
      }
    },
  }
  return true;
} */

fn is_prime(number: u64, prime_numbers: &Vec<u64>) -> bool {
  for n in 0..=(prime_numbers.len()/3) {
    if number % prime_numbers[n] == 0 { return false; }
  }
  return true;
}

fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
  let mut prime_numbers = vec![3u64];
  if m == 2 || m == 5 {
    let mut is_diff: bool = true;
    let diff = m + g as u64;
    for i in 2..diff {
      if diff % i == 0 {
        is_diff = false;
        break;
      }
    }
    if is_diff { return Some((m, diff)); }
  }

  let mut number = 7u64;
  while number < m {
    if is_prime(number, &prime_numbers) { prime_numbers.push(number); }
    match number % 10 {
      3 => number += 4,
      1|7|9 => number += 2,
      _ => break,
    }
  }
  while number < n {
    if is_prime(number, &prime_numbers) {
      prime_numbers.push(number);
      let other_number = number + g as u64;
      match other_number % 10 {
        1|3|7|9 => if is_prime(other_number, &prime_numbers) { return Some((number, other_number)); }
        _ => {},
      }
    }
    match number % 10 {
      3 => number += 4,
      1|7|9 => number += 2,
      _ => break,
    }
  }
  return None;
}

// Sieve of Eratosthenes 
fn sieve_of_eratosthenes (diff: i32, min: u64, max: u64) -> Option<(u64, u64)> {
  let mut is_prime = vec![true; 1+max as usize];
  let mut i = 2;
  while i*i <= max {
    if is_prime[i as usize] {
      let mut n = i*2;
      while n <= max {
        is_prime[n as usize] = false;

        n += i;
      }
    }

    i += 1;
  }
  for i in min..=(max - diff as u64) {
    if is_prime[i as usize] {
      let a = i + diff as u64;
      if is_prime[a as usize] {
        return Some((i, a));
      }
    }
  }

  return None;
}

/* TODO merge into m single loop, check your numbers as you go.*/
/* fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let prime_numbers = find_prime_numbers(n);
    let mut i = 0;
    while i < prime_numbers.len() {
        if prime_numbers[i] >= m {
            let mut x = i+1;
            while x < prime_numbers.len() {
                let gerence = prime_numbers[x] - prime_numbers[i];
                if gerence > g as u64 {
                    break;
                }
                else if gerence == g as u64 {
                    return Some((prime_numbers[i], prime_numbers[x]));
                }
                else {
                    x += 1;
                }
            }
        }
        i += 1;
    }
    return None;
}

fn find_prime_numbers(end_value: u64) -> Vec<u64> {
    let mut prime_numbers = vec![2u64];
    let mut n = 3u64;
    while n <= end_value as u64 {
        let mut is_prime = true;
        for number in prime_numbers.iter() {
          if n % number == 0 {
            is_prime = false;
            break;
          }
        }
        if is_prime {
            prime_numbers.push(n);
        }
        n += 2;
    }
    return prime_numbers;
} */

/* WARNING: The following code is too slow.
fn is_prime (number: u64) -> bool {
  if number % 2 == 0 && number != 2 { return false; }
  let mut i = 3;
  while i < number {
    if number % i == 0 { return false; }
    i += 2;
  }
  return true;
}

fn step (g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
  let mut index = m;
  while index <= n - g {
    if is_prime(index) {
      if is_prime(index + g as u64) {
        return Some((index, index + g as u64));
      }
    }
    index += 1;
  }
  return None;
}*/