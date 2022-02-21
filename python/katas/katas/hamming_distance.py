def compute(dna_string1: str, dna_string2: str) -> int:
    hamming_distance = 0
    for index in range(0, len(dna_string1)):
        if dna_string1[index] is not dna_string2[index]:
            hamming_distance += 1
    return hamming_distance
