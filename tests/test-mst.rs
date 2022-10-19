#[cfg(test)]

mod tests {
    use graphs_at_ssu::*;

    #[test]
    fn test_prim1() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/mst/mst1.gr".to_string())?;
        let mst = algorithms::mst::prim(&gr)?;
        let mst_weight: i32 = mst.iter().map(|edge| edge.2).sum();
        assert_eq!(mst_weight, 120);
        Ok(())
    }

    #[test]
    fn test_prim2() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/mst/mst2.gr".to_string())?;
        let mst = algorithms::mst::prim(&gr)?;
        let mst_weight: i32 = mst.iter().map(|edge| edge.2).sum();
        assert_eq!(mst_weight, 30);
        Ok(())
    }

    #[test]
    fn test_prim3() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/mst/mst3.gr".to_string())?;
        let mst = algorithms::mst::prim(&gr)?;
        let mst_weight: i32 = mst.iter().map(|edge| edge.2).sum();
        assert_eq!(mst_weight, 4_903);
        Ok(())
    }

    #[test]
    fn test_prim4() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/mst/mst4.gr".to_string())?;
        let mst = algorithms::mst::prim(&gr)?;
        let mst_weight: i32 = mst.iter().map(|edge| edge.2).sum();
        assert_eq!(mst_weight, 11);
        Ok(())
    }

    #[test]
    fn test_kruskal1() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/mst/mst1.gr".to_string())?;
        let mst = algorithms::mst::kruskal(&gr)?;
        let mst_weight: i32 = mst.iter().map(|edge| edge.2).sum();
        assert_eq!(mst_weight, 120);
        Ok(())
    }

    #[test]
    fn test_kruskal2() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/mst/mst2.gr".to_string())?;
        let mst = algorithms::mst::kruskal(&gr)?;
        let mst_weight: i32 = mst.iter().map(|edge| edge.2).sum();
        assert_eq!(mst_weight, 30);
        Ok(())
    }

    #[test]
    fn test_kruskal3() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/mst/mst3.gr".to_string())?;
        let mst = algorithms::mst::kruskal(&gr)?;
        let mst_weight: i32 = mst.iter().map(|edge| edge.2).sum();
        assert_eq!(mst_weight, 4_903);
        Ok(())
    }

    #[test]
    fn test_kruskal4() -> GraphResult<()> {
        let gr = Graph::from_file("graphs/tests/mst/mst4.gr".to_string())?;
        let mst = algorithms::mst::kruskal(&gr)?;
        let mst_weight: i32 = mst.iter().map(|edge| edge.2).sum();
        assert_eq!(mst_weight, 11);
        Ok(())
    }
}
