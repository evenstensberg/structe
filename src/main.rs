fn main() {
    println!("Structe Rust built && ran!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        assert_eq!(true, true);
    }
}