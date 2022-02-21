import pytest

from hamming_distance import compute


@pytest.mark.parametrize('dna_string1,dna_string2,expected', [
    ('GAGCCTACTAACGGGAT', 'GAGCCTACTAACGGGAT', 0),
    ('GAGCCTACTAACGGGAT', 'CAGCCTACTAACGGGAT', 1),
    ('GAGCCTACTAACGGGAT', 'GACCCAACTAACGGGAT', 2),
    ('GAGCCTACTAACGGGAT', 'GAGCCATGTAACGGGAT', 3),
    ('GAGCCTACTAACGGGAT', 'GAGCCTACTTTCGCCAT', 4),
    ('GAGCCTACTAACGGGAT', 'GAGCCTACTTTCGCCAA', 5),
    ('GAGCCTACTAACGGGAT', 'GATCGTAATGACGGCCT', 6),
    ('GAGCCTACTAACGGGAT', 'CATCGTAATGACGGCCT', 7),
])
def test_hamming_distance_computation(dna_string1: str, dna_string2: str, expected: int) -> None:
    assert expected == compute(dna_string1, dna_string2)
