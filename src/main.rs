use std::{thread, time::Instant};

fn main() {
    let n = 1000_000_000;
    let nthreads = 4;

    let timer = Instant::now();
    let x = sum_of_primes(n);
    let time = timer.elapsed();

    println!("single:\n{}", x);
    println!("{:?}", time);

    let timer = Instant::now();
    let x = sum_of_parallel(n, nthreads);
    let time = timer.elapsed();

    println!("\nthreaded:\n{}", x);
    println!("{:?}", time);
}

fn sum_of_parallel(n: usize, nthreads: usize) -> usize {
    let sieve_length = n / 2 - 1;
    let sieve_limit = (((n as f64).sqrt() as usize + 1) - 3) / 2 + 1;

    (0..=nthreads)
        .map(|i| {
            thread::spawn(move || {
                sieve_of_eratosthenes(
                    sieve_length,
                    (0..sieve_limit).skip(i).step_by(nthreads),
                )
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .fold(vec![true; sieve_length], |mut acc, n_sieve| {
            for (a, b) in acc.iter_mut().zip(n_sieve.into_iter()) {
                *a &= b
            }
            acc
        })
        .into_iter()
        .enumerate()
        .map(|(i, b)| if b { 2 * i + 3 } else { 0 })
        .sum::<usize>()
        + 2
}

fn sieve_of_eratosthenes<I: Iterator<Item = usize>>(
    sieve_length: usize,
    nums: I,
) -> Vec<bool> {
    let mut sieve = vec![true; sieve_length];

    for i in nums {
        if sieve[i] {
            let p = 2 * i + 3;
            for j in (i + p..sieve.len()).step_by(p) {
                sieve[j] = false;
            }
        }
    }

    sieve
}

fn sum_of_primes(n: usize) -> usize {
    let mut sieve = vec![true; n / 2 - 1];

    let mut sum = 2;

    let limit = (((n as f64).sqrt() as usize + 1) - 3) / 2;

    for i in 0..limit {
        if sieve[i] {
            let p = 2 * i + 3;
            sum += p;
            for j in (i + p..sieve.len()).step_by(p) {
                sieve[j] = false;
            }
        }
    }

    sum += (limit..sieve.len())
        .map(|i| if sieve[i] { 2 * i + 3 } else { 0 })
        .sum::<usize>();

    sum
}
