#[cfg(test)]

mod tests {
    use graphs_at_ssu::*;

    #[test]
    fn test_dfs1() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/traversals/traversal1.gr".to_string())?;
        let dfs = algorithms::traversals::dfs(&gr, "A".to_string());
        assert_eq!(dfs, vec!["A", "B", "E", "F", "C", "G", "H", "D", "J", "K"]);
        Ok(())
    }

    #[test]
    fn test_dfs2() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/traversals/traversal2.gr".to_string())?;
        let dfs = algorithms::traversals::dfs(&gr, "A".to_string());
        assert_eq!(dfs, vec!["A", "B", "C", "D", "J", "K", "E", "F"]);
        Ok(())
    }
    
    #[test]
    fn test_dfs3() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/traversals/traversal3.gr".to_string())?;
        let dfs = algorithms::traversals::dfs(&gr, "A".to_string());
        assert_eq!(dfs, vec!["A", "B", "E", "F", "G", "C", "H", "J", "D", "K"]);
        Ok(())
    }
    
    #[test]
    fn test_bfs1() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/traversals/traversal1.gr".to_string())?;
        let bfs = algorithms::traversals::bfs(&gr, "A".to_string());
        assert_eq!(bfs, vec!["E", "F", "B", "G", "H", "C", "J", "K", "D", "A"]);
        Ok(())
    }
    
    #[test]
    fn test_bfs2() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/traversals/traversal2.gr".to_string())?;
        let bfs = algorithms::traversals::bfs(&gr, "A".to_string());
        assert_eq!(bfs, vec!["J", "K", "D", "C", "E", "F", "B", "A"]);
        Ok(())
    }
    
    #[test]
    fn test_bfs3() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/traversals/traversal3.gr".to_string())?;
        let bfs = algorithms::traversals::bfs(&gr, "A".to_string());
        assert_eq!(bfs, vec!["K", "D", "J", "H", "C", "G", "F", "E", "B", "A"]);
        Ok(())
    }
}
