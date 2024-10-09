use std::collections::{HashMap, HashSet};

pub struct Nfa {
    initial: HashSet<State>,
    accepting: HashSet<State>,
    transitions: Vec<HashMap<char, HashSet<State>>>, // TODO: add members
}

type State = usize;

impl Nfa {
    pub fn new(n_states: usize) -> Self {
        Self {
            initial: HashSet::default(),
            accepting: HashSet::default(),
            transitions: vec![HashMap::default(); n_states],
        }
    }

    pub fn add_transition(&mut self, from: State, to: State, label: char) {
        self.transitions[from].entry(label).or_default().insert(to);
    }

    pub fn add_initial(&mut self, q: State) {
        self.initial.insert(q);
    }

    pub fn add_final(&mut self, q: State) {
        self.accepting.insert(q);
    }

    fn step(&self, states: HashSet<State>, a: char) -> HashSet<State> {
        let mut res = HashSet::new();
        for p in states {
            if let Some(tr) = self.transitions[p].get(&a) {
                for q in tr {
                    res.insert(*q);
                }
            }
        }

        res
    }

    pub fn accepts(&self, s: &str) -> bool {
        let mut states = self.initial.clone();
        for a in s.chars() {
            states = self.step(states, a);
        }

        !states.is_disjoint(&self.accepting)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parity() {
        let mut nfa = Nfa::new(2);
        nfa.add_transition(0, 1, 'a');
        nfa.add_transition(1, 0, 'a');
        nfa.add_transition(0, 0, 'b');
        nfa.add_transition(1, 1, 'b');
        nfa.add_initial(0);
        nfa.add_final(0);

        assert!(nfa.accepts(""));
        assert!(nfa.accepts("ababbaba"));
        assert!(nfa.accepts("aabbaa"));
        assert!(!nfa.accepts("abbaa"));
        assert!(!nfa.accepts("aaa"));
    }

    #[test]
    fn a_b_star() {
        let mut nfa = Nfa::new(2);
        nfa.add_transition(0, 1, 'a');
        nfa.add_transition(1, 0, 'b');
        nfa.add_initial(0);
        nfa.add_final(0);

        assert!(nfa.accepts(""));
        assert!(nfa.accepts("ababab"));
        assert!(nfa.accepts("abab"));
        assert!(!nfa.accepts("aba"));
        assert!(!nfa.accepts("aababa"));
        assert!(!nfa.accepts("abababba"));
    }
}
