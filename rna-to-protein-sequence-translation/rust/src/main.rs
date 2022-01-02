use std::collections::HashMap;

fn protein(rna: &str) -> Result<String, String> {
    let stop_codon = "Stop";
    let mut codon_hashmap = HashMap::new();
    codon_hashmap.insert("UUC", "F");
    codon_hashmap.insert("UUU", "F");
    codon_hashmap.insert("UUA", "L");
    codon_hashmap.insert("UUG", "L");
    codon_hashmap.insert("CUU", "L");
    codon_hashmap.insert("CUC", "L");
    codon_hashmap.insert("CUA", "L");
    codon_hashmap.insert("CUG", "L");
    codon_hashmap.insert("AUU", "I");
    codon_hashmap.insert("AUC", "I");
    codon_hashmap.insert("AUA", "I");
    codon_hashmap.insert("AUG", "M");
    codon_hashmap.insert("GUU", "V");
    codon_hashmap.insert("GUC", "V");
    codon_hashmap.insert("GUA", "V");
    codon_hashmap.insert("GUG", "V");
    codon_hashmap.insert("UCU", "S");
    codon_hashmap.insert("UCC", "S");
    codon_hashmap.insert("UCA", "S");
    codon_hashmap.insert("UCG", "S");
    codon_hashmap.insert("AGU", "S");
    codon_hashmap.insert("AGC", "S");
    codon_hashmap.insert("CCU", "P");
    codon_hashmap.insert("CCC", "P");
    codon_hashmap.insert("CCA", "P");
    codon_hashmap.insert("CCG", "P");
    codon_hashmap.insert("ACU", "T");
    codon_hashmap.insert("ACC", "T");
    codon_hashmap.insert("ACA", "T");
    codon_hashmap.insert("ACG", "T");
    codon_hashmap.insert("GCU", "A");
    codon_hashmap.insert("GCC", "A");
    codon_hashmap.insert("GCA", "A");
    codon_hashmap.insert("GCG", "A");
    codon_hashmap.insert("UAU", "Y");
    codon_hashmap.insert("UAC", "Y");
    codon_hashmap.insert("CAU", "H");
    codon_hashmap.insert("CAC", "H");
    codon_hashmap.insert("CAA", "Q");
    codon_hashmap.insert("CAG", "Q");
    codon_hashmap.insert("AAU", "N");
    codon_hashmap.insert("AAC", "N");
    codon_hashmap.insert("AAA", "K");
    codon_hashmap.insert("AAG", "K");
    codon_hashmap.insert("GAU", "D");
    codon_hashmap.insert("GAC", "D");
    codon_hashmap.insert("GAA", "E");
    codon_hashmap.insert("GAG", "E");
    codon_hashmap.insert("UGU", "C");
    codon_hashmap.insert("UGC", "C");
    codon_hashmap.insert("UGG", "W");
    codon_hashmap.insert("CGU", "R");
    codon_hashmap.insert("CGC", "R");
    codon_hashmap.insert("CGA", "R");
    codon_hashmap.insert("CGG", "R");
    codon_hashmap.insert("AGA", "R");
    codon_hashmap.insert("AGG", "R");
    codon_hashmap.insert("GGU", "G");
    codon_hashmap.insert("GGC", "G");
    codon_hashmap.insert("GGA", "G");
    codon_hashmap.insert("GGG", "G");
    codon_hashmap.insert("UAA", stop_codon);
    codon_hashmap.insert("UGA", stop_codon);
    codon_hashmap.insert("UAG", stop_codon);

    let mut protein_string: String = "".to_owned();
    for x in (0..rna.chars().count()).step_by(3) {
        let codon = &rna[x..x + 3];
        if codon_hashmap.contains_key(codon) {
            let amino_acid = codon_hashmap.get(codon).unwrap();
            if amino_acid.to_string() == stop_codon {
                break;
            }
            protein_string.push_str(amino_acid);
        }
    }
    Ok(protein_string.to_string())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_protein() -> Result<(), String> {
        assert_eq!(protein("AUG")?,  "M");
        assert_eq!(protein("AUGUGA")?,  "M");
        assert_eq!(protein("AUGGUUAGUUGA")?,  "MVS");
        assert_eq!(protein("UGCGAUGAAUGGGCUCGCUCC")?,  "CDEWARS");
        assert_eq!(protein("AUGUCCUUCCAUCAAGGAAACCAUGCGCGUUCAGCUUUCUGA")?,  "MSFHQGNHARSAF");
        assert_eq!(protein("AUGCUUCAAGUGCACUGGAAAAGGAGAGGGAAAACCAGUUGA")?,  "MLQVHWKRRGKTS");
        assert_eq!(protein("AUGGCGUUCAGCUUUCUAUGGAGGGUAGUGUACCCAUGCUGA")?,  "MAFSFLWRVVYPC");
        assert_eq!(protein("AUGCAGCUUUCUAUGGAGGGUAGUGUUAACUACCACGCCUGA")?,  "MQLSMEGSVNYHA");
        assert_eq!(protein("AUGCUAUGGAGGGUAGUGUUAACUACCACGCCCAGUACUUGA")?,  "MLWRVVLTTTPST");
        assert_eq!(protein("AUGUAUCCUUCCAUCAAGGAAACCAUGCGCGUUCAGCUUUCUAUGGAGGGUAGUGUUAACUACCACGCCUUCAAGUGCACUGGAAAAGGAGAGGGAAAACCAUACGAAGGCACCCAAAGCCUGAAUAUUACAAUAACUGAAGGAGGUCCUCUGCCAUUUGCUUUUGACAUUCUGUCACACGCCUUUCAGUAUGGCAUCAAGGUCUUCGCCAAGUACCCCAAAGAAAUUCCUGACUUCUUUAAGCAGUCUCUACCUGGUGGUUUUUCUUGGGAAAGAGUAAGCACCUAUGAAGAUGGAGGAGUGCUUUCAGCUACCCAAGAAACAAGUUUGCAGGGUGAUUGCAUCAUCUGCAAAGUUAAAGUCCUUGGCACCAAUUUUCCCGCAAACGGUCCAGUGAUGCAAAAGAAGACCUGUGGAUGGGAGCCAUCAACUGAAACAGUCAUCCCACGAGAUGGUGGACUUCUGCUUCGCGAUACCCCCGCACUUAUGCUGGCUGACGGAGGUCAUCUUUCUUGCUUCAUGGAAACAACUUACAAGUCGAAGAAAGAGGUAAAGCUUCCAGAACUUCACUUUCAUCAUUUGCGUAUGGAAAAGCUGAACAUAAGUGACGAUUGGAAGACCGUUGAGCAGCACGAGUCUGUGGUGGCUAGCUACUCCCAAGUGCCUUCGAAAUUAGGACAUAACUGA")?,  "MYPSIKETMRVQLSMEGSVNYHAFKCTGKGEGKPYEGTQSLNITITEGGPLPFAFDILSHAFQYGIKVFAKYPKEIPDFFKQSLPGGFSWERVSTYEDGGVLSATQETSLQGDCIICKVKVLGTNFPANGPVMQKKTCGWEPSTETVIPRDGGLLLRDTPALMLADGGHLSCFMETTYKSKKEVKLPELHFHHLRMEKLNISDDWKTVEQHESVVASYSQVPSKLGHN");
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
