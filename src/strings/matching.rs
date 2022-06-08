use std::str::Chars;

pub fn matching<'b>(pattern: &str, candidates: Vec<&'b str>) -> Vec<&'b str> {
    let l = candidates.len();
    let mut pattern_iter = pattern.chars();
    let mut candidates_iters: Vec<Chars> = candidates
        .iter()
        .map(|candidate| candidate.chars())
        .collect();
    let mut active = vec![true; candidates.len()];
    loop {
        let p = match pattern_iter.next() {
            Some(p) => p,
            None => {
                // reached the end of the pattern --> return all candidates still standing
                let mut result = vec![];
                for i in 0..l {
                    if active[i] {
                        result.push(candidates[i]);
                    }
                }
                return result;
            }
        };
        for i in 0..l {
            let candidate = &mut candidates_iters[i];
            loop {
                match &candidate.next() {
                    Some(c) if c == &p => break,
                    Some(_) => continue,
                    None => {
                        active[i] = false;
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
