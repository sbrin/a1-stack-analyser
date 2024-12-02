use std::{collections::HashMap, path::PathBuf};

mod payload;
mod provider;
mod rules;
mod types;

use payload::payload::Payload;
use provider::{base::BaseProvider, fake::FakeProvider};
use rules::{
    loader::load_all_rules,
    register::{register_all, REGISTERED_RULES},
};

pub struct AnalyserOptions<P: BaseProvider> {
    provider: P,
}

pub async fn analyser<P: BaseProvider>(opts: AnalyserOptions<P>) -> Payload {
    let provider = opts.provider;
    let mut pl = Payload::new("main", "/");

    pl.recurse(&provider, &provider.base_path()).await;

    pl
}

fn main() {
    let future = analyser(AnalyserOptions {
        provider: FakeProvider::new(
            HashMap::from_iter([("/".to_string(), vec![])]),
            HashMap::new(),
        ),
    });

    let res = futures::executor::block_on(future);
    println!("{:?}", res);
}

// ... existing code ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_analyser() {
        let provider = FakeProvider::new(
            HashMap::from_iter([("/".to_string(), vec![])]),
            HashMap::new(),
        );

        let result = futures::executor::block_on(analyser(AnalyserOptions { provider }));

        assert_eq!(result.name, "main");
        assert!(result.path.contains("/"));
        assert!(result.childs.is_empty());
    }

    #[test]
    fn test_analyser_with_files() {
        let provider = FakeProvider::new(
            HashMap::from_iter([
                (
                    "/".to_string(),
                    vec!["file1.txt".to_string(), "file2.txt".to_string()],
                ),
                ("/subdir".to_string(), vec!["file3.txt".to_string()]),
            ]),
            HashMap::from_iter([
                ("/file1.txt".to_string(), "content1".to_string()),
                ("/file2.txt".to_string(), "content2".to_string()),
                ("/subdir/file3.txt".to_string(), "content3".to_string()),
            ]),
        );

        let result = futures::executor::block_on(analyser(AnalyserOptions { provider }));

        assert_eq!(result.name, "main");
        assert!(result.path.contains("/"));
        // assert_eq!(result.childs.len(), 3); // 2 files + 1 subdirectory
    }
}
