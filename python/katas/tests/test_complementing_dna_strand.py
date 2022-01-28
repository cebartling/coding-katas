from complementing_dna_strand import reverse_complement


def test_reverse_complement_dna() -> None:
    result = reverse_complement('AAAACCCGGT')
    assert result == 'ACCGGGTTTT'
