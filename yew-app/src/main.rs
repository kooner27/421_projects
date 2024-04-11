use yew::prelude::*;
mod connect4;
use connect4::{Board, Player, State};


#[function_component(ConnectFourGame)]
fn connect_four_game() -> Html {
    let board = use_state(|| Board::new(6, 7)); // Initialize the board

    let on_column_click = {
        let board = board.clone();
        Callback::from(move |col: usize| {
            let mut b = (*board).clone(); // Clone the current board state
            b.insert_disc(col).ok(); // Ignore errors for simplicity
            board.set(b); // Update the board state
        })
    };
    
    let pixel_size = "100px";
    let button_style = format!("display: grid; grid-template-columns: repeat({}, {});", board.cols, pixel_size);
    let grid_style = format!("display: grid; grid-template-columns: repeat({}, {}); grid-auto-rows: {};", board.cols, pixel_size, pixel_size);

    html! {
        <>
            <h1>{ "Connect Four" }</h1>
            <div style={button_style.clone()}>
                {
                    (0..board.cols).map(|col| {
                        html! {
                            <button style="text-align: center;" onclick={on_column_click.reform(move |_| col)}>
                                { format!("Drop in Col {}", col) }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div style={grid_style.clone()}>
                {
                    for board.grid.iter().flatten().map(|cell| {
                        html! {
                            <div style="border: 1px solid black; text-align: center; line-height: 100px;">
                                { format!("{:?}", cell) }
                            </div>
                        }
                    })
                }
            </div>
            <div>
                {
                    match board.state {
                        State::Won(player) => html! { <p>{ format!("Player {:?} wins!", player) }</p> },
                        State::Draw => html! { <p>{ "The game is a draw!" }</p> },
                        State::Running => html! { <p>{ "Game is in progress..." }</p> },
                    }
                }
            </div>
        </>
    }



}

#[function_component(App)]
fn app() -> Html {
    html! {
        <ConnectFourGame />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
