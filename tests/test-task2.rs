#[cfg(test)]

mod tests {
    use std::collections::{BTreeSet, HashMap, HashSet};

    use graphs_at_ssu::*;

    #[test]
    fn test_sc_components1() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task21.gr".to_string())?;
        let components = tasks::task2::solve21(&gr)?;

        let expected = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["D".to_string(), "E".to_string(), "F".to_string()],
            vec!["G".to_string(), "H".to_string()],
        ];
        let expected = expected
            .into_iter()
            .map(|v| v.into_iter().collect::<BTreeSet<_>>())
            .collect::<HashSet<_>>();

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_sc_components2() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task22.gr".to_string())?;
        let components = tasks::task2::solve21(&gr)?;

        let expected = vec![
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
            ],
            vec!["F".to_string()],
            vec!["G".to_string()],
        ];
        let expected = expected
            .into_iter()
            .map(|v| v.into_iter().collect::<BTreeSet<_>>())
            .collect::<HashSet<_>>();

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_sc_components3() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task23.gr".to_string())?;
        let components = tasks::task2::solve21(&gr)?;

        let expected = vec![
            vec!["A".to_string()],
            vec!["B".to_string()],
            vec!["C".to_string()],
            vec!["D".to_string()],
        ];
        let expected = expected
            .into_iter()
            .map(|v| v.into_iter().collect::<BTreeSet<_>>())
            .collect::<HashSet<_>>();

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_sc_components4() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task24.gr".to_string())?;
        let components = tasks::task2::solve21(&gr)?;

        let expected = vec![
            vec!["A".to_string()],
            vec!["B".to_string(), "C".to_string()],
            vec![
                "D".to_string(),
                "E".to_string(),
                "F".to_string(),
                "G".to_string(),
            ],
            vec!["H".to_string()],
            vec!["J".to_string()],
        ];
        let expected = expected
            .into_iter()
            .map(|v| v.into_iter().collect::<BTreeSet<_>>())
            .collect::<HashSet<_>>();

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_sc_components5() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task25.gr".to_string())?;
        let components = tasks::task2::solve21(&gr)?;

        let expected = vec![
            vec!["A".to_string()],
            vec!["B".to_string()],
            vec!["C".to_string()],
            vec!["D".to_string()],
            vec!["E".to_string()],
            vec!["F".to_string()],
            vec!["G".to_string()],
            vec!["H".to_string()],
            vec!["J".to_string()],
        ];
        let expected = expected
            .into_iter()
            .map(|v| v.into_iter().collect::<BTreeSet<_>>())
            .collect::<HashSet<_>>();

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_sc_components6() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task26.gr".to_string())?;
        let components = tasks::task2::solve21(&gr)?;

        let expected = vec![
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
                "F".to_string(),
                "G".to_string(),
                "H".to_string(),
            ],
            vec![
                "J".to_string(),
                "K".to_string(),
                "L".to_string(),
                "M".to_string(),
            ],
        ];
        let expected = expected
            .into_iter()
            .map(|v| v.into_iter().collect::<BTreeSet<_>>())
            .collect::<HashSet<_>>();

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_shortest_paths1() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task21.gr".to_string())?;
        let components = tasks::task2::solve22(&gr, "A".to_string());

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), vec!["A".to_string()]);
        expected.insert("B".to_string(), vec!["A".to_string(), "B".to_string()]);
        expected.insert(
            "C".to_string(),
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
        );
        expected.insert(
            "D".to_string(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
            ],
        );
        expected.insert(
            "E".to_string(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
            ],
        );
        expected.insert(
            "F".to_string(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "F".to_string(),
            ],
        );
        expected.insert(
            "G".to_string(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "G".to_string(),
            ],
        );
        expected.insert(
            "H".to_string(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "G".to_string(),
                "H".to_string(),
            ],
        );

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_shortest_paths2() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task22.gr".to_string())?;
        let components = tasks::task2::solve22(&gr, "A".to_string());

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), vec!["A".to_string()]);
        expected.insert("B".to_string(), vec!["A".to_string(), "B".to_string()]);
        expected.insert(
            "C".to_string(),
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
        );
        expected.insert(
            "D".to_string(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
            ],
        );
        expected.insert(
            "E".to_string(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
            ],
        );
        expected.insert(
            "F".to_string(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "F".to_string(),
            ],
        );
        expected.insert("G".to_string(), vec![]);

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_shortest_paths3() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task23.gr".to_string())?;
        let components = tasks::task2::solve22(&gr, "A".to_string());

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), vec!["A".to_string()]);
        expected.insert("B".to_string(), vec![]);
        expected.insert("C".to_string(), vec!["A".to_string(), "C".to_string()]);
        expected.insert("D".to_string(), vec!["A".to_string(), "D".to_string()]);

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_shortest_paths4() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task24.gr".to_string())?;
        let components = tasks::task2::solve22(&gr, "E".to_string());

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), vec![]);
        expected.insert("B".to_string(), vec![]);
        expected.insert("C".to_string(), vec![]);
        expected.insert(
            "D".to_string(),
            vec![
                "E".to_string(),
                "F".to_string(),
                "G".to_string(),
                "D".to_string(),
            ],
        );
        expected.insert("E".to_string(), vec!["E".to_string()]);
        expected.insert("F".to_string(), vec!["E".to_string(), "F".to_string()]);
        expected.insert(
            "G".to_string(),
            vec!["E".to_string(), "F".to_string(), "G".to_string()],
        );
        expected.insert(
            "H".to_string(),
            vec!["E".to_string(), "F".to_string(), "H".to_string()],
        );
        expected.insert(
            "J".to_string(),
            vec![
                "E".to_string(),
                "F".to_string(),
                "H".to_string(),
                "J".to_string(),
            ],
        );

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_shortest_paths5() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task25.gr".to_string())?;
        let components = tasks::task2::solve22(&gr, "A".to_string());

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), vec!["A".to_string()]);
        expected.insert("B".to_string(), vec![]);
        expected.insert("C".to_string(), vec![]);
        expected.insert("D".to_string(), vec![]);
        expected.insert("E".to_string(), vec![]);
        expected.insert("F".to_string(), vec![]);
        expected.insert("G".to_string(), vec![]);
        expected.insert("H".to_string(), vec![]);
        expected.insert("J".to_string(), vec![]);

        assert_eq!(components, expected);
        Ok(())
    }

    #[test]
    fn test_shortest_paths6() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/task26.gr".to_string())?;
        let components = tasks::task2::solve22(&gr, "A".to_string());

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), vec!["A".to_string()]);
        expected.insert("B".to_string(), vec!["A".to_string(), "B".to_string()]);
        expected.insert(
            "C".to_string(),
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
        );
        expected.insert("D".to_string(), vec!["A".to_string(), "D".to_string()]);
        expected.insert("E".to_string(), vec!["A".to_string(), "E".to_string()]);
        expected.insert(
            "F".to_string(),
            vec!["A".to_string(), "E".to_string(), "F".to_string()],
        );
        expected.insert(
            "G".to_string(),
            vec![
                "A".to_string(),
                "E".to_string(),
                "F".to_string(),
                "G".to_string(),
            ],
        );
        expected.insert(
            "H".to_string(),
            vec![
                "A".to_string(),
                "E".to_string(),
                "F".to_string(),
                "G".to_string(),
                "H".to_string(),
            ],
        );
        expected.insert("J".to_string(), vec!["A".to_string(), "J".to_string()]);
        expected.insert(
            "K".to_string(),
            vec!["A".to_string(), "J".to_string(), "K".to_string()],
        );
        expected.insert(
            "L".to_string(),
            vec![
                "A".to_string(),
                "J".to_string(),
                "K".to_string(),
                "L".to_string(),
            ],
        );
        expected.insert(
            "M".to_string(),
            vec![
                "A".to_string(),
                "J".to_string(),
                "K".to_string(),
                "L".to_string(),
                "M".to_string(),
            ],
        );

        assert_eq!(components, expected);
        Ok(())
    }
}
