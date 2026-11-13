use std::thread::{self, ThreadId};

#[derive(Debug)]
struct Report {
    vowels: usize,
    worker: ThreadId,
}

fn analyze(text: &str) -> Report {
    Report {
        vowels: text.chars().filter(|c| "aeiouAEIOU".contains(*c)).count(),
        worker: thread::current().id(),
    }
}

fn analyze_all(texts: Vec<String>) -> Vec<Report> {
    // TODO: spawn one thread per text that calls `analyze`, then join all
    //       threads and return their reports in input order
    todo!()
}

fn main() {
    let texts = vec![
        String::from("fearless concurrency"),
        String::from("Rust"),
        String::from("threads share the heap"),
    ];

    for report in analyze_all(texts) {
        println!("{} vowels, counted by {:?}", report.vowels, report.worker);
    }
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn texts(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| word.to_string()).collect()
    }

    #[test]
    fn test_results_are_in_input_order() {
        let reports = analyze_all(texts(&["aaa", "b", "ee", "Rust is fun"]));
        let vowels: Vec<usize> = reports.iter().map(|report| report.vowels).collect();
        assert_eq!(vowels, vec![3, 0, 2, 3]);
    }

    #[test]
    fn test_empty_input() {
        assert!(analyze_all(Vec::new()).is_empty());
    }

    #[test]
    fn test_one_thread_per_text() {
        let reports = analyze_all(texts(&["a", "b", "c", "d"]));
        let workers: HashSet<ThreadId> = reports.iter().map(|report| report.worker).collect();
        assert_eq!(workers.len(), 4);
    }

    #[test]
    fn test_work_is_not_done_by_the_caller() {
        let reports = analyze_all(texts(&["a", "b"]));
        let caller = thread::current().id();
        assert!(reports.iter().all(|report| report.worker != caller));
    }
}
