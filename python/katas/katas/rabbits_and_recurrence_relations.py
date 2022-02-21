def recurrence_relation(months: int, pairs_per_generation: int) -> int:
    total_pairs = [1, 1]
    if months > 2:
        for n in range(2, months):
            additional_pairs = total_pairs[n - 2] * pairs_per_generation
            total_pairs.append(additional_pairs + total_pairs[n - 1])
    return total_pairs.pop()
