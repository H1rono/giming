use io::*;
use std::*;

fn solve(r: f64, n: i64) {
    //
}

fn main() {
    let con = read_string();
    let mut scanner = Scanner::new(&con);
    let mut r: f64;
    r = scanner.next();
    let mut N: i64;
    N = scanner.next();
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(move || solve(r, N)).unwrap().join().unwrap();
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
