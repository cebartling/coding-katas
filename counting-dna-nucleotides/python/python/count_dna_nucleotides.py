def count(dna: str) -> dict:
    a_count = 0
    c_count = 0
    g_count = 0
    t_count = 0
    for nucleotide in dna:
        match nucleotide:
            case 'A':
                a_count += 1
            case 'C':
                c_count += 1
            case 'G':
                g_count += 1
            case 'T':
                t_count += 1
    return {'A': a_count, 'C': c_count, 'G': g_count, 'T': t_count}
