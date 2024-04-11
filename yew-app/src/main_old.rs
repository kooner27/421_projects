
use yew::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    P1(char), // T or O for player 1
    P2(char), // T or O for player 2
}

#[derive(PartialEq, Clone, Copy)]
enum GameType {
    Otto,
    Connect4,
}

#[derive(PartialEq, Clone, Copy)]
enum Difficulty {
    Easy,
    Hard,
}

#[function_component(Game)]
fn app() -> Html {
    // game dimension
    let width = 6;
    let height = 4;
    let board_ot = use_state(|| vec![vec![Tile::Empty; width]; height]);
    // player 1 starts the game
    let current_player = use_state(|| 1);
    // the winner
    let winner = use_state(|| -1);
    // game type
    let game_type = use_state(|| GameType::Otto);

    // game difficulty
    let difficulty = use_state(|| Difficulty::Easy);

    // game started
    let game_started = use_state(|| false);

    // Function to handle a player move
    // let handle_click = {
    //     let board_ot = board_ot.clone();
    //     let current_player = current_player.clone();
    //     let winner = winner.clone();
    //     Callback::from(move |(row, col): (usize, usize)| {
    //         let mut board = (*board_ot).clone();
    //         if board[row][col] == Tile::Empty {
    //             board[row][col] = if *current_player == 1 {
    //                 Tile::P1('T') // For Player 1, 'T'
    //             } else {
    //                 Tile::P2('O') // For Player 2, 'O'
    //             };
    //             board_ot.set(board);
    //             // Switch player turn
    //             current_player.set(if *current_player == 1 { 2 } else { 1 });
    //         }
    //     })
    // };
    let handle_click = {
        let board_ot = board_ot.clone();
        let current_player = current_player.clone();
        let winner = winner.clone();
        let game_type = game_type.clone();
        Callback::from(move |(row, col): (usize, usize)| {
            let mut board = (*board_ot).clone();
            // Iterate from the bottom of the column upwards
            for row in (0..height).rev() {
                if board[row][col] == Tile::Empty {
                    board[row][col] = match *game_type {
                        GameType::Otto => {
                            if *current_player == 1 {
                                Tile::P1('T') // For Player 1, 'T'
                            } else {
                                Tile::P2('O') // For Player 2, 'O'
                            }
                        }
                        GameType::Connect4 => {
                            if *current_player == 1 {
                                Tile::P1('*') // For Player 1, '*'
                            } else {
                                Tile::P2('@') // For Player 2, '@'
                            }
                        }
                    };
                    board_ot.set(board);
                    // Switch player turn
                    current_player.set(if *current_player == 1 { 2 } else { 1 });
                    break; // Exit the loop once we've found the first empty tile from the bottom
                }
            }
        })
    };

    let handle_start = {
        let game_started = game_started.clone();
        Callback::from(move |_| {
            game_started.set(true);
        })
    };

    let handle_cancel = {
        let game_started = game_started.clone();
        let board_ot = board_ot.clone();
        Callback::from(move |_| {
            game_started.set(false);
            board_ot.set(vec![vec![Tile::Empty; width]; height]);
        })
    };
    let game_type_clone1 = game_type.clone();
    let game_type_clone2 = game_type.clone();
    html! {
            <div>
                <h1>{ "Play Game of Connect4 or Toot-Otto" }</h1>
                <div>
                    
                    <input type="radio" id="otto" name="gameType" value="otto" checked={*game_type == GameType::Otto} disabled={*game_started} onchange={Callback::from(move |_| game_type_clone1.set(GameType::Otto))} />
                    <label for="otto">{"Otto"}</label>
                    
                    
                    <input type="radio" id="connect4" name="gameType" value="connect4" checked={*game_type == GameType::Connect4} disabled={*game_started} onchange={Callback::from(move |_| game_type_clone2.set(GameType::Connect4))} />
                    <label for="connect4">{"Connect 4"}</label>
                    <input type="radio" id="easy" name="difficulty" value="easy" checked={*difficulty == Difficulty::Easy} disabled={*game_started} />
                    <label for="easy">{"Easy"}</label>
                    <input type="radio" id="hard" name="difficulty" value="hard" checked={*difficulty == Difficulty::Hard} disabled={*game_started} />
                    <label for="hard">{"Hard"}</label>
                    <button onclick={handle_start} disabled={*game_started}>{"Start"}</button>
                    <button onclick={handle_cancel} disabled={!*game_started}>{"Cancel"}</button>
                </div>
                { if *game_started {
                    html! {
                        <div style="display: grid; grid-template-columns: repeat(6, 50px); gap: 5px;">
                            { for board_ot.iter().enumerate().map(|(row, line)| html! {
                                { for line.iter().enumerate().map(|(col, &tile)| {
                                    let on_click = {
                                        let row = row;
                                        let col = col;
                                        handle_click.reform(move |_| (row, col))
                                    };

                                    html! {
                                        <div style="border: 1px solid black; text-align: center; line-height: 50px; width: 50px; height: 50px;" onclick={on_click}>
                                            { match tile {
                                                Tile::Empty => ' ',
                                                Tile::P1(letter) | Tile::P2(letter) => letter,
                                            }}
                                        </div>
                                    }
                                })}
                            })}
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        }
}

// C:\Users\ponti\Documents\doc_rustproj\421_projects\yew-app>trunk serve
fn main() {
    yew::Renderer::<Game>::new().render();
}
