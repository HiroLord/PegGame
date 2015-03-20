fn main() {
    let init_state = State{board: make_board(), prev: ((0,0), (0,0))};
    recurse(&init_state);
    init_state.print();
}

fn recurse(state: &State) -> bool{
    let succ = get_successors(&state.board);
    for s in range(0,succ.len()) {
        if is_winner(&succ[s].board) { succ[s].print(); return true; } 
        if recurse(&succ[s]) {
            succ[s].print();
            return true;
        }
    }
    false
}

fn is_winner(board: &[[bool; 5]; 5]) -> bool {
    let mut count = 0;
    for r in 0..5 {
        for c in 0..r+1 {
            if board[r][c] {
                count += 1;
            }
        }
    }
    count < 2
}

fn get_successors(board: &[[bool; 5]; 5]) -> Vec<State> {
    let mut new_states = Vec::new();
    for r in 0..5 {
        for c in 0..r+1 {
            if board[r][c] == false {
                let mut nr = (r as i32)-1;
                if nr < 0 { nr = 0; }
                let mut nc = (c as i32)-1;
                if nc < 0 { nc = 0; }
                for rr in nr as usize..r+2 {
                    for cc in nc as usize..c+2 {
                        if cc <= rr  && cc < 5 && rr < 5 && (rr as i32 - r as i32) + (cc as i32 - c as i32) != 0{
                            if board[rr][cc] == true {
                                let roff = 2*(rr as i32) - (r as i32);
                                let coff = 2*(cc as i32) - (c as i32);
                                if roff < 0 || coff < 0 || coff > roff || roff > 4 || coff > 4{
                                    continue;
                                }
                                if board[rr+rr-r][cc+cc-c] == true {
                                    let mut new_board = copy(board);
                                    new_board[r][c] = true;
                                    new_board[rr][cc] = false;
                                    new_board[rr+rr-r][cc+cc-c] = false;
                                    let new_state = State{board: new_board, prev:
                                        ((rr+rr-r,cc+cc-c),(r,c))};
                                    new_states.push(new_state);

                                }
                            }
                        }
                    }
                }
            }
        }
    }
    new_states
}

fn make_board() -> [[bool; 5]; 5] {
    let mut board = [[true; 5]; 5];
    board[0][0] = false;
    board
}

struct State {
    board: [[bool; 5]; 5],
    prev: ((usize, usize), (usize, usize)),
}

impl State {
    fn print(&self) {
        for r in 0..5 {
            for c in 0..r+1 {
                if self.board[r][c] {
                    print!("*\t");
                } else {
                    print!("o\t");
                }
            }
            println!("");
        }
        println!("({}, {}) -> ({}, {})\n", (self.prev.0).0, (self.prev.0).1, (self.prev.1).0,
            (self.prev.1).1);
    }
}

fn copy(board: &[[bool; 5]; 5]) -> [[bool; 5]; 5] {
    let mut new_board = make_board();
    for r in 0..5 {
        for c in 0..r+1 {
            new_board[r][c] = board[r][c];
        }
    }
    new_board
}
