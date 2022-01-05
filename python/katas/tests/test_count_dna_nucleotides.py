from count_dna_nucleotides import count


def test_count():
    results = count('AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC')
    assert results['A'] == 20
    assert results['C'] == 12
    assert results['G'] == 17
    assert results['T'] == 21


def test_count2():
    results = count('AGCTTTTCATTCTGAAAAGAGTGTCTGATAGCAGC')
    assert results['A'] == 10
    assert results['C'] == 6
    assert results['G'] == 8
    assert results['T'] == 11
