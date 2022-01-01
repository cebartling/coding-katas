fn rna(dna: &str) -> Result<String, String> {
    Ok(dna.replace( "T", "U"))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_rna_single_replacement() -> Result<(), String> {
        assert_eq!(rna("CGAT")?, "CGAU");
        Ok(())
    }

    #[test]
    fn test_rna_multiple_replacement() -> Result<(), String> {
        assert_eq!(rna("CGATCGATCGATCGATCGAT")?, "CGAUCGAUCGAUCGAUCGAU");
        Ok(())
    }
}


fn main() {
    println!("Hello, world!");
}
