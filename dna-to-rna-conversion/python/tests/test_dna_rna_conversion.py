from python.convert_dna_to_rna import convert_dna_to_rna


def test_convert_tyrosine_to_uracil() -> None:
    result = convert_dna_to_rna('GCAT')
    assert result == 'GCAU'


def test_convert_tyrosine_to_uracil_multiple_instances() -> None:
    result = convert_dna_to_rna('GCATTA')
    assert result == 'GCAUUA'


def test_convert_emtpy_string() -> None:
    result = convert_dna_to_rna('')
    assert result == ''
