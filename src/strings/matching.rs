use std::str::Chars;

/// provides the elements from candidates that match the given pattern
pub fn matching<'a>(pattern: &str, candidates: Vec<&'a str>) -> Vec<&'a str> {
    let len = candidates.len();
    let mut pattern_iter = pattern.chars();
    let mut candidates_iters: Vec<Chars> = candidates
        .iter()
        .map(|candidate| candidate.chars())
        .collect();
    let mut candidate_in_race = vec![true; candidates.len()]; // true = still in the race, false = no match
    loop {
        let pattern_char = match pattern_iter.next() {
            Some(c) => c,
            None => {
                // reached the end of the pattern --> return all candidates still in the race
                let mut result = vec![];
                for i in 0..len {
                    if candidate_in_race[i] {
                        result.push(candidates[i]);
                    }
                }
                return result;
            }
        };
        for i in 0..len {
            let candidate = &mut candidates_iters[i];
            loop {
                match &candidate.next() {
                    Some(candidate_char) if candidate_char == &pattern_char => break, // same character --> done with this candidate
                    Some(_) => continue, // no match --> go to the next candidate character
                    None => {
                        // candidate ran out of characters while the pattern still has characters left --> candidate is no match
                        candidate_in_race[i] = false;
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    mod matching {

        #[test]
        fn no_match() {
            let names = vec!["task1", "task2", "task3"];
            let have = super::super::matching("t4", names);
            let want: Vec<&str> = vec![];
            assert_eq!(have, want);
        }

        #[test]
        fn one_match() {
            let names = vec!["task1", "task2", "task3"];
            let have = super::super::matching("t2", names);
            let want = vec!["task2"];
            assert_eq!(have, want);
        }

        #[test]
        fn multiple_matches() {
            let names = vec!["task1", "task2"];
            let have = super::super::matching("tk", names);
            let want = vec!["task1", "task2"];
            assert_eq!(have, want);
        }
    }
}
