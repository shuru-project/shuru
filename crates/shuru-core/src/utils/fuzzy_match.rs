pub fn fuzzy_match(query: &str, candidates: Vec<String>) -> Vec<(String, f64)> {
    let query_lowercased = query.to_lowercase();

    let mut result: Vec<(String, f64)> = candidates
        .iter()
        .map(|candidate| {
            let candidate_lowercased = candidate.to_lowercase();
            let distance = levenshtein_distance(&query_lowercased, &candidate_lowercased);
            let max_len = query_lowercased.len().max(candidate_lowercased.len()) as f64;

            // Normalize the score: 1 - (distance / max_len)
            let normalized_score = 1.0 - (distance as f64 / max_len);
            (candidate.to_owned(), normalized_score)
        })
        .collect();

    // Sort by score (descending)
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    result
}

/// Filters matches based on a minimum normalized score threshold
pub fn filter_matches(query: &str, candidates: Vec<String>, min_score: f64) -> Vec<(String, f64)> {
    fuzzy_match(query, candidates)
        .into_iter()
        .filter(|&(_, score)| score >= min_score)
        .collect()
}

/// Computes the Levenshtein distance between two strings
fn levenshtein_distance(query: &str, candidate: &str) -> usize {
    let query_len = query.len();
    let candidate_len = candidate.len();

    let mut distance_matrix: Vec<Vec<usize>> = (0..=query_len)
        .map(|i| {
            (0..=candidate_len)
                .map(|j| {
                    if i == 0 {
                        j
                    } else if j == 0 {
                        i
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();

    let query_chars = query.chars().collect::<Vec<_>>();
    let candidate_chars = candidate.chars().collect::<Vec<_>>();

    for i in 1..=query_len {
        for j in 1..=candidate_len {
            let cost = if query_chars[i - 1] == candidate_chars[j - 1] {
                0
            } else {
                1
            };

            distance_matrix[i][j] = (distance_matrix[i - 1][j - 1] + cost)
                .min((distance_matrix[i - 1][j] + 1).min(distance_matrix[i][j - 1] + 1));
        }
    }

    distance_matrix[query_len][candidate_len]
}
