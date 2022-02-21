import pytest

from hamming_distance import compute


@pytest.mark.parametrize('dna_string1,dna_string2,expected', [
    ('GAGCCTACTAACGGGAT', 'CATCGTAATGACGGCCT', 7),
    ('GAGCCTACTAACGGGAT', 'GATCGTAATGACGGCCT', 6),
])
def test_hamming_distance_computation(dna_string1: str, dna_string2: str, expected: int) -> None:
    assert expected == compute(dna_string1, dna_string2)
