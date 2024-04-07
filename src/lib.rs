const MATCH_SCORE: i32 = 10;
const MISMATCH_SCORE: i32 = 3;
const GAP_PENALTY: i32 = -1;

pub fn align_sequences(sequence1: &str, sequence2: &str) -> (String, String) {
    let len1 = sequence1.len();
    let len2 = sequence2.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        dp[i][0] = (i as i32) * GAP_PENALTY;
    }

    for j in 0..=len2 {
        dp[0][j] = (j as i32) * GAP_PENALTY;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let match_score =
                dp[i - 1][j - 1] + score(sequence1.as_bytes()[i - 1], sequence2.as_bytes()[j - 1]);
            let delete_score = dp[i - 1][j] + GAP_PENALTY;
            let insert_score = dp[i][j - 1] + GAP_PENALTY;
            dp[i][j] = max(match_score, delete_score, insert_score);
        }
    }

    let mut aligned_seq1 = String::new();
    let mut aligned_seq2 = String::new();
    let mut i = len1;
    let mut j = len2;

    while i > 0 || j > 0 {
        if i > 0
            && j > 0
            && dp[i][j]
                == dp[i - 1][j - 1]
                    + score(sequence1.as_bytes()[i - 1], sequence2.as_bytes()[j - 1])
        {
            aligned_seq1.insert(0, sequence1.chars().nth(i - 1).unwrap());
            aligned_seq2.insert(0, sequence2.chars().nth(j - 1).unwrap());
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i][j] == dp[i - 1][j] + GAP_PENALTY {
            aligned_seq1.insert(0, sequence1.chars().nth(i - 1).unwrap());
            aligned_seq2.insert(0, '-');
            i -= 1;
        } else {
            aligned_seq1.insert(0, '-');
            aligned_seq2.insert(0, sequence2.chars().nth(j - 1).unwrap());
            j -= 1;
        }
    }

    (aligned_seq1, aligned_seq2)
}

fn max(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}

fn score(a: u8, b: u8) -> i32 {
    if a == b {
        MATCH_SCORE
    } else {
        MISMATCH_SCORE
    }
}

#[cfg(test)]
mod tests {
    use super::align_sequences;

    #[test]
    fn test_alignment() {
        let seq1 = "AGTACGCA";
        let seq2 = "TATGC";
        let (aligned_seq1, aligned_seq2) = align_sequences(seq1, seq2);
        assert_eq!(aligned_seq1, "AGTACGCA");
        assert_eq!(aligned_seq2, "--TATGC-");
    }
}
