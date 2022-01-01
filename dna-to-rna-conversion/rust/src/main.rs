fn convert(dna: &str) -> Result<String, String> {
    Ok(str::replace(dna, "T", "U"))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_convert() -> Result<(), String> {
        assert_eq!(convert("CGAT")?, "CGAU");
        Ok(())
    }
}


fn main() {
    println!("Hello, world!");
}
