import re


def compute(sample: str, substr: str) -> list[int]:
    """
    Given two strings s and t, t is a substring of s if t is contained as a contiguous collection of symbols in s (as a result, t must be no longer than s). The position of a symbol in a string is the total number of symbols found to its left, including itself (e.g., the positions of all occurrences of 'U' in "AUGCUUCAGAAAGGUCUUACG" are 2, 5, 6, 15, 17, and 18). The symbol at position i of s is denoted by s[i].

    :param sample: A string.
    :param substr: A string that is equal to or shorter than sample string.
    :return: A list of integers representing the start position of the occurrence of substr in string sample.
    """
    return [m.start() + 1 for m in re.finditer(f'(?={substr})', sample)]
