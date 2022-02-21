# https://rosalind.info/problems/gc/

def count_gc(current_id: str, dna_string: str) -> (str, float):
    total_bases = 0
    gc_bases = 0
    for base in dna_string:
        total_bases += 1
        if base is 'C' or base is 'G':
            gc_bases += 1
    return current_id, "{:.6f}".format((gc_bases / total_bases) * 100)


def compute_gc_content(dna_fasta_format: str) -> (str, float):
    results = []
    current_id = None
    dna_string = ''
    for line in dna_fasta_format.splitlines():
        if line.startswith('>'):
            if current_id is not None:
                results.append(count_gc(current_id, dna_string))
            current_id = line[1:]
            dna_string = ''
        else:
            dna_string += line
    results.append(count_gc(current_id, dna_string))
    results.sort(key=lambda x: x[1])
    return results.pop()
