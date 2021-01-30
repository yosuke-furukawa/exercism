use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

fn modpow(a: u64, b: u64, p: u64) -> u64 {
    let mut res = 1;
    let mut a = a;
    let mut n = b;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    res
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(b_pub, a, p)
}
