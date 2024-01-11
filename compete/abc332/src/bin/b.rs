use io::*;
use std::*;
use std::cmp::*;

fn solve(k: i64, g: i64, m: i64) {
    let mut cg = g;
    let mut cm = m;
    for _ in 0..k {
        if cg == g {
            cg = 0;
        } else if cm == 0 {
            cm = m;
        } else {
            let ng = cg + min(cm, g - cg);
            let nm = cm - min(cm, g - cg);
            cg = ng;
            cm = nm;
        }
    }
    println!("{} {}", cg, cm);
}

fn main() {
    let con = read_string();
    let mut scanner = Scanner::new(&con);
    let mut K: i64;
    K = scanner.next();
    let mut G: i64;
    G = scanner.next();
    let mut M: i64;
    M = scanner.next();
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(move || solve(K, G, M)).unwrap().join().unwrap();
}

pub mod io {
    use std;
    use std::str::FromStr;

    pub struct Scanner<'a> {
        iter: std::str::SplitWhitespace<'a>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(s: &'a str) -> Scanner<'a> {
            Scanner {
                iter: s.split_whitespace(),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            let s = self.iter.next().unwrap();
            if let Ok(v) = s.parse::<T>() {
                v
            } else {
                panic!("Parse error")
            }
        }

        pub fn next_vec_len<T: FromStr>(&mut self) -> Vec<T> {
            let n: usize = self.next();
            self.next_vec(n)
        }

        pub fn next_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.next()).collect()
        }
    }

    pub fn read_string() -> String {
        use std::io::Read;

        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }

    pub fn read_line() -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    }
}
