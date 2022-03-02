from scipy.special import comb


def calculate_probability(k: int, m: int, n: int) -> float:
    """
    Calculates the probability that two randomly selected mating organisms will produce an individual
    possessing a dominant allele (and thus displaying the dominant phenotype). Assume that any two
    organisms can mate.

    :param k: number of homozygous dominant organisms
    :param m: number of homozygous recessive organisms
    :param n: number of heterozygous organisms
    :return: The probability that two randomly selected mating organisms will produce an individual
    possessing a dominant allele (and thus displaying the dominant phenotype). Assume that any two
    organisms can mate.
    """
    # Calculate total number of organisms in the population:
    total_population = k + m + n

    # Calculate the number of combos that could be made (valid or not):
    total_combinations = comb(total_population, 2)

    # Calculate the number of combinations that have a dominant allele therefore are valid.
    valid_combinations = (comb(k, 2)) + (k * m) + (k * n) + (.5 * m * n) + (.75 * comb(m, 2))

    # An event is simply a collection of outcomes. Because outcomes are distinct, the probability of
    # an event can be written as the sum of the probabilities of its constituent outcomes.
    probability = valid_combinations / total_combinations

    return probability
