/*
 * Compute integer partitions
 *
 * Useage:
 * for the number of k size partitions of n: partitions n k
 * for the total number of partitions of n: partitions n
 *
 * skittlemittle 2013
 */

use std::collections::HashMap;
use std::env;
use std::hash::{BuildHasherDefault, Hasher};
use std::process;

static BIG_ENOUGH_TO_BE_WORTH_MEMOING: i32 = 50; // memo if n is bigger than this

/** Recursively compute integer partitions of n
 * @param n: integer to compute partitions of
 * @param k: size of partitions you want to be counting
 * @param acc: accumulator, start it at 0
 */
fn partition(n: i32, k: i32, mut acc: i64, memo: &mut MemoHash) -> i64 {
    if k > n {
        return 0;
    }

    if n == k || k == 1 {
        acc += 1;
        return acc;
    }

    // cheeky optimisation
    // shit you not this makes it run so much faster
    if k == 2 {
        acc += (n / k) as i64;
        return acc;
    }

    let mut last_acc = acc;
    for i in (1..k + 1).rev() {
        let m = n - k;

        if i > m {
            continue;
        }
        if not_trivial(m, i) {
            // half of Szudzik's function, we dont memoise when m <= i
            let tag = m * m + m + i;

            let stored = memo.get(&tag);
            if stored != None {
                acc = stored.unwrap() + last_acc;
            } else {
                acc = partition(m, i, acc, memo);
            }

            // memoise owo
            let _ = &memo.entry(tag).or_insert(acc - last_acc);
        } else {
            acc = partition(m, i, acc, memo);
        }
        last_acc = acc;
    }
    acc
}

/** Count all integer partitions of n
 * @param n: integer to count partitions of
 * @param memo: memoisation store
 */
fn partitions(n: i32, memo: &mut MemoHash) -> i64 {
    if n == 0 {
        return 0;
    }

    let mut p: i64 = 0;

    for i in 1..n + 1 {
        p += partition(n, i, 0, memo);
    }

    p
}

// ======= Helpers =======
/** trivial cases dont need to be memoised */
fn not_trivial(n: i32, k: i32) -> bool {
    if k == 2 || k == 1 || k == n || n < BIG_ENOUGH_TO_BE_WORTH_MEMOING {
        return false;
    }
    true
}

#[derive(Default)]
struct Tag(i32);

// SPEED, just copy the key passed as the hash, not secure coz why would it even need to be
impl Hasher for Tag {
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!()
    }

    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write_i32(&mut self, key: i32) {
        self.0 = key;
    }
}

type MemoHash = HashMap<i32, i64, BuildHasherDefault<Tag>>;

fn main() {
    let mut args: env::Args = env::args();
    args.next();

    let n = match args.next() {
        Some(a) => a,
        None => "".to_string(),
    };

    let k = match args.next() {
        Some(a) => a,
        None => "".to_string(),
    };

    if n == "" {
        eprintln!("Missing argument");
        process::exit(1);
    }

    let mut memo: MemoHash = HashMap::<i32, i64, BuildHasherDefault<Tag>>::default();

    if k != "" {
        println!(
            "{}",
            partition(
                n.parse::<i32>().unwrap(),
                k.parse::<i32>().unwrap(),
                0,
                &mut memo
            )
        );
    } else {
        println!("{}", partitions(n.parse::<i32>().unwrap(), &mut memo));
    }
}
