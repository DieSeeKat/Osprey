extern crate osprey;

#[cfg(test)]
mod perft_tests {
    use crate::osprey::Board;
    use crate::osprey::perft::perft;

    #[test]
    fn start_depth_4() {
        let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let nodes = perft(&board, 0, 4, 1);

        assert_eq!(nodes, 197_281);
    }

    #[test]
    fn kwikipete() {
        let board =
            Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let nodes = perft(&board, 0, 3, 1);

        assert_eq!(nodes, 97_862);
    }

    #[test]
    fn position_3() {
        let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
        let nodes = perft(&board, 0, 4, 1);

        assert_eq!(nodes, 43_238);
    }

    #[test]
    fn position_4() {
        let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
        let nodes = perft(&board, 0, 3, 1);

        assert_eq!(nodes, 9_467);
    }

    #[test]
    fn position_5() {
        let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
        let nodes = perft(&board, 0, 3, 1);

        assert_eq!(nodes, 62_379);
    }

    #[test]
    fn position_6() {
        let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ");
        let nodes = perft(&board, 0, 3, 1);

        assert_eq!(nodes, 89_890);
    }
}
