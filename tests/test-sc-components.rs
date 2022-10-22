#[cfg(test)]

mod tests {
    use std::collections::{BTreeSet, HashSet};

    use graphs_at_ssu::*;

    #[test]
    fn test_sc_components1() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/task2/sc_components1.gr".to_string())?;
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
        let gr = Graph::from_file("graphs/tests/task2/sc_components2.gr".to_string())?;
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
        let gr = Graph::from_file("graphs/tests/task2/sc_components3.gr".to_string())?;
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
        let gr = Graph::from_file("graphs/tests/task2/sc_components4.gr".to_string())?;
        let components = tasks::task2::solve21(&gr)?;
        
        let expected = vec![
            vec!["A".to_string()],
            vec!["B".to_string(), "C".to_string()],
            vec!["D".to_string(), "E".to_string(), "F".to_string(), "G".to_string()],
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
        let gr = Graph::from_file("graphs/tests/task2/sc_components5.gr".to_string())?;
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
}
