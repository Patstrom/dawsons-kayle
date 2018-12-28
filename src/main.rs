use std::env;

fn mex(s: Vec<u16>) -> u16 {
    let mut i = 0;
    while s.contains(&i) {
        i += 1;
    }
    i
}

fn g(n: usize, sgv: &Vec<u16>) -> u16  {
    match n {
        0|1 => 0,
        2|3 => 1,
        _ => {
            mex((0..( (n-1) - (n-1) / 2)).map(|i| sgv[i] ^ sgv[n-i-2]).collect())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();

    let mut sgv = vec![0; n+1];

    for i in 0..sgv.len() {
        sgv[i] = g(i, &sgv);
    }

    let mut total = 0;
    for _sgv in sgv {
        if _sgv != 0 {
            total += 1
        }
    }
    println!("{:?}", total);
}


