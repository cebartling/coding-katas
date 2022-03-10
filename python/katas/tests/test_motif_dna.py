import pytest

from motif_dna import compute


@pytest.mark.parametrize('sample,pattern,expected', [
    ('GATATATGCATATACTT', 'ATAT', [2, 4, 10]),
    ('GTTATATGCATATACTTGATACATGCATATACTTGATATATGCAGATACTT', 'ATAT', [4, 10, 27, 36, 38]),
])
def test_compute(sample: str, pattern: str, expected: list[int]) -> None:
    assert compute(sample, pattern) == expected
