pub fn raindrops(n: usize) -> String {

    if (n % 3 != 0) && (n % 5 != 0) && (n % 7 != 0) {
        return n.to_string()
    }

    let mut s = String::with_capacity(15);

    if n % 3 == 0 {
        s += "Pling";
    }
    if n % 5 == 0 {
        s += "Plang";
    }
    if n % 7 ==0 {
        s += "Plong";
    }

    s
}
