from python.convert_dna_to_rna import convert_dna_to_rna


def test_convert_tyrosine_to_uracil() -> None:
    result = convert_dna_to_rna('GCAT')
    assert result == 'GCAU'
