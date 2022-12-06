use std::{collections::HashSet, hash::Hash};

pub fn halves(input: &str) -> (&str, &str) {
    input.split_at(input.bytes().len() / 2)
}

pub fn intersecting_unique_chars(one: &str, two: &str) -> Vec<char> {
    let mut intersection: Vec<char> = one
        .chars()
        .into_iter()
        .filter(|c| two.contains(*c))
        .collect();
    dedup(&mut intersection); // make chars unique
    return intersection;
}

fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) {
    // note the Copy constraint
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_halves() {
        assert_eq!(halves("abcddcba"), ("abcd", "dcba"));
        assert_eq!(halves("abcdedcba"), ("abcd", "edcba")); //uneaven size
    }

    #[test]
    fn test_intersecting_chars() {
        assert_eq!(
            intersecting_unique_chars("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            vec!['p']
        );
        assert_eq!(
            intersecting_unique_chars("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            vec!['L']
        );
    }
}
