def reverse_complement(dna: str) -> str:
    result = ''
    for nucleotide in dna:
        match nucleotide:
            case 'A':
                result = result + 'T'
            case 'C':
                result = result + 'G'
            case 'G':
                result = result + 'C'
            case 'T':
                result = result + 'A'
    return result[::-1]
