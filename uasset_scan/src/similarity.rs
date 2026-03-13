//! Shared string similarity helpers used by digest-backed matching.

/// Calculate Levenshtein distance between two strings.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for (i, row) in matrix.iter_mut().enumerate() {
        row[0] = i;
    }

    #[allow(clippy::needless_range_loop)]
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for (i, a_char) in a_chars.iter().enumerate() {
        for (j, b_char) in b_chars.iter().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[a_len][b_len]
}

/// Return the closest candidate within the given maximum edit distance.
pub fn find_closest_match<'a>(
    query: &str,
    candidates: impl IntoIterator<Item = &'a str>,
    max_distance: usize,
) -> Option<&'a str> {
    let query_lower = query.to_lowercase();

    candidates
        .into_iter()
        .map(|candidate| {
            (
                candidate,
                levenshtein(&candidate.to_lowercase(), &query_lower),
            )
        })
        .filter(|(_, distance)| *distance <= max_distance)
        .min_by_key(|(_, distance)| *distance)
        .map(|(candidate, _)| candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("", "test"), 4);
        assert_eq!(levenshtein("test", ""), 4);
        assert_eq!(levenshtein("same", "same"), 0);
    }

    #[test]
    fn test_find_closest_match() {
        let candidates = ["Extinguish", "Light", "AddFuel"];
        assert_eq!(
            find_closest_match("Extenguish", candidates.iter().copied(), 3),
            Some("Extinguish")
        );
        assert_eq!(
            find_closest_match("Unknown", candidates.iter().copied(), 3),
            None
        );
    }
}
