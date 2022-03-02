import pytest

from mendel_first_law import calculate_probability


@pytest.mark.parametrize('k,m,n,expected', [
    (2, 2, 2, 0.7833333333333333),
    (4, 4, 4, 0.7651515151515151),
    (10, 10, 10, 0.7557471264367817),
])
def test_calculate_probability(k: int, m: int, n: int, expected: float):
    assert calculate_probability(k=k, m=m, n=n) == expected
