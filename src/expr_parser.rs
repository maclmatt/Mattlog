use crate::Term;

const PRECEDENCE: &[(&str, u8)] = &[
    ("*", 3), ("/", 3),
    ("+", 2), ("-", 2),
    ("<", 1), (">", 1),
    ("=<", 1), (">=", 1),
    ("=", 1), ("\\=", 1),
];

fn precedence(op: &str) -> u8 {
    PRECEDENCE.iter()
        .find(|&&(sym, _)| sym == op)
        .map(|&(_, prec)| prec)
        .unwrap_or(0)
}

pub fn parse_expression_with_precedence(terms: &[Term]) -> Option<Term> {
    let mut it = terms.iter();
    parse_with_precedence(&mut it, 0)
}

fn parse_with_precedence(it: &mut std::slice::Iter<Term>, min_prec: u8) -> Option<Term> {
    let mut left = parse_primary(it)?;

    while let Some(op_term) = it.next() {
        if let Term::Atom(op) = op_term {
            let prec = precedence(op);
            if prec < min_prec {
                // Operator has lower precedence, put it back and stop.
                return Some(left);
            }

            // Binary operator — expect right operand
            let mut right = parse_with_precedence(it, prec + 1)?;

            // Combine into new compound term (forming the expression tree)
            left = Term::Compound(op.clone(), vec![left, right]);
        } else {
            // Not an operator — we’re done with this expression
            return Some(left);
        }
    }

    Some(left)
}

fn parse_primary(it: &mut std::slice::Iter<Term>) -> Option<Term> {
    it.next().cloned() // Term could be integer, variable, compound, etc.
}

pub fn preprocess_expression(term: &Term) -> Option<Term> {
    if let Term::Compound(_, args) = term {
        parse_expression_with_precedence(args)
    } else {
        Some(term.clone())
    }
}
