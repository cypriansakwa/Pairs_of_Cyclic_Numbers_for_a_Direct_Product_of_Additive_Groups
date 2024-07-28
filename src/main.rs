fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn are_coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}

fn generate_cyclic_pairs(limit: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();
    for n in 1..=limit {
        for m in 1..=limit {
            if are_coprime(n, m) {
                pairs.push((n, m));
            }
        }
    }
    pairs
}

fn main() {
    let limit = 100; // Adjust the limit as needed
    let pairs = generate_cyclic_pairs(limit);
    for (n, m) in pairs {
        println!("({}, {})", n, m);
    }
}
