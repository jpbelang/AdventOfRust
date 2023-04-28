fn main() {
    println!("Hello, world!");
}

fn add(a: u32, b: u32) -> u32 {

    return a + b;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
