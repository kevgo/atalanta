use std::str::Chars;

/// provides the elements from candidates that match the given pattern
pub fn matching<'a>(pattern: &str, candidates: Vec<&'a str>) -> Vec<&'a str> {
    let len = candidates.len();
    let mut pattern_iter = pattern.chars();
    let mut candidates_iters: Vec<Chars> = candidates
        .iter()
        .map(|candidate| candidate.chars())
        .collect();
    let mut tracker = Tracker::new(candidates);
    loop {
        let pattern_char = match pattern_iter.next() {
            Some(c) => c,
            None => {
                // reached the end of the pattern --> return all candidates still in the race
                return tracker.actives();
            }
        };
        for i in 0..len {
            loop {
                match candidates_iters[i].next() {
                    Some(candidate_char) if candidate_char == pattern_char => break, // same character --> done with this candidate
                    Some(_) => continue, // no match --> go to the next candidate character
                    None => {
                        // candidate ran out of characters while the pattern still has characters left --> candidate is no match
                        tracker.disable(i);
                        break;
                    }
                }
            }
        }
    }
}

/// tracks a boolean status of given elements
struct Tracker<'a> {
    elements: Vec<&'a str>,
    statuses: Vec<bool>,
}

impl<'a> Tracker<'a> {
    fn new(candidates: Vec<&str>) -> Tracker {
        let len = candidates.len();
        Tracker {
            elements: candidates,
            statuses: vec![true; len],
        }
    }

    /// disables the element with the given number
    fn disable(&mut self, index: usize) {
        self.statuses[index] = false;
    }

    /// provides all the active elements
    fn actives(&self) -> Vec<&'a str> {
        let mut result = vec![];
        for i in 0..self.len() {
            if self.is_active(i) {
                result.push(self.elements[i]);
            }
        }
        return result;
    }

    /// indicates whether the given element is active
    fn is_active(&self, number: usize) -> bool {
        self.statuses[number]
    }

    /// provides the number of elements
    fn len(&self) -> usize {
        self.elements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Tracker;

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

    #[test]
    fn tracker() {
        let mut t = Tracker::new(vec!["one", "two", "three"]);
        t.disable(1);
        assert!(t.is_active(0));
        assert!(!t.is_active(1));
        assert_eq!(t.actives(), vec!["one", "three"]);
    }
}
