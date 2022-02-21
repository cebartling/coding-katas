import pytest
from rabbits_and_recurrence_relations import recurrence_relation


@pytest.mark.parametrize('month,pairs_per_generation,expected', [
    # each element of this list will provide values for the
    # topics "value_A" and "value_B" of the test and will
    # generate a stand-alone test case.
    (1, 3, 1),
    (2, 3, 1),
    (3, 3, 4),
    (4, 3, 7),
    (5, 3, 19),
    (6, 3, 40),
    (7, 3, 97),
    (12, 3, 6160),
])
def test_recurrence_relation(month, pairs_per_generation, expected):
    assert expected == recurrence_relation(month, pairs_per_generation)
