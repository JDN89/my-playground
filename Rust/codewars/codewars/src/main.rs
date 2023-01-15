fn hor_mirror(s: String) -> String {
    s.split('\n').rev().collect::<Vec<&str>>().join("\n")
}
fn vert_mirror(s: String) -> String {
    s.split('\n')
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}
fn oper(oper: fn(String) -> String, s: String) -> String {
    oper(s)
}
fn main() {
    oper(hor_mirror, "yeCt\nCSbg\nJVhv\nlVHt".to_string());

    oper(vert_mirror, "yeCt\nCSbg\nJVhv\nlVHt".to_string());
}

fn testing1(s: &str, exp: &str) -> () {
    assert_eq!(oper(hor_mirror, s.to_string()), exp)
}
fn testing2(s: &str, exp: &str) -> () {
    assert_eq!(oper(vert_mirror, s.to_string()), exp)
}

#[test]
fn basics_oper() {
    testing1("lVHt\nJVhv\nCSbg\nyeCt", "yeCt\nCSbg\nJVhv\nlVHt");
    testing1("njMK\ndbrZ\nLPKo\ncEYz", "cEYz\nLPKo\ndbrZ\nnjMK");
    testing1("QMxo\ntmFe\nWLUG\nowoq", "owoq\nWLUG\ntmFe\nQMxo");
}

#[test]
fn basics_oper2() {
    testing2(
        "hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu",
        "QHdgSh\noaMDnH\nXxNNlC\nHxxvRi\nAvVTqb\nuRySvw",
    );
    testing2(
        "IzOTWE\nkkbeCM\nWuzZxM\nvDddJw\njiJyHF\nPVHfSx",
        "EWTOzI\nMCebkk\nMxZzuW\nwJddDv\nFHyJij\nxSfHVP",
    );
    testing2("cuQW\nxOuD\nfZwp\neqFx", "WQuc\nDuOx\npwZf\nxFqe");
}
