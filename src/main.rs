use simple_genetic::agent::{self, Agent};

struct Board {
    board: Vec<Vec<i8>>,
}

impl Board {
    fn check_win(&self) -> i8 {
        for row in 0..self.board.len() {
            if &self.board[row][0] == &self.board[row][1]
                && &self.board[row][1] == &self.board[row][2]
                && self.board[row][0] != 0
            {
                return self.board[row][0];
            }
        }

        for col in 0..self.board.len() {
            if &self.board[0][col] == &self.board[1][col]
                && &self.board[1][col] == &self.board[2][col]
                && self.board[2][col] != 0
            {
                return self.board[0][col];
            }
        }

        if &self.board[0][0] == &self.board[1][1]
            && &self.board[1][1] == &self.board[2][2]
            && self.board[0][0] != 0
        {
            return self.board[0][0];
        }

        if &self.board[0][2] == &self.board[1][1]
            && &self.board[1][1] == &self.board[2][0]
            && self.board[0][2] != 0
        {
            return self.board[0][2];
        }

        0
    }

    fn check_draw(&self) -> bool {
        for i in 0..self.board.len() {
            for j in 0..self.board.len() {
                if self.board[i][j] == 0 {
                    return false;
                }
            }
        }
        true
    }

    fn play_piece(&mut self, row: usize, col: usize, value: i8) -> () {
        self.board[row][col] = value;
    }

    fn play_game(&mut self, agent_1: Agent, agnet_2: Agent) -> () {
        while self.check_win() == 0 || !self.check_draw() {
            for row in 0..self.board.len() {
                for col in 0..self.board[row].len() {
                    agent_1.set_value(self.board[row][col] as f32, row + col * 3, 0)
                }
            }
            agent_1.calculate();
            let row = agent_1.output_list
            self.play_piece(row, col, 1);
            self.play_piece(row, col, -1);
        }
    }
}

fn make_boards(amount: usize) -> Vec<Board> {
    let mut board_list: Vec<Board> = Vec::with_capacity(amount);
    for _ in 0..amount {
        board_list.push(Board {
            board: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        })
    }
    board_list
}

fn main() {
    let generations = 20;
    let num_agents: usize = 50;
    let agents: Vec<agent::Agent> =
        simple_genetic::agent::create_agents(num_agents, 35, 9, 20, 9, 0.002);
    for generation in 0..generations {
        let mut board_list = make_boards(num_agents / 2);
        for i in 0..board_list.len() {}
    }
}
