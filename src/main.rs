use dioxus::prelude::*;
use tic_tac_toe::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let game = use_context_provider(|| Signal::new(Game::new()));

    let board = [Hor::Left, Hor::Mid, Hor::Right].into_iter().flat_map(|h| {
        [Vert::Top, Vert::Mid, Vert::Bottom]
            .into_iter()
            .map(move |v| {
                rsx!(Field {
                    game: game,
                    field: FieldName { v, h }
                })
            })
    });

    rsx!(
        div { class: "game",
              div { class: "board", {board} }
              div { class: "state", {game.with(|g| g.state().to_string())} }
        }
    )
}

#[component]
#[allow(non_snake_case)]
fn Field(game: Signal<Game>, field: FieldName) -> Element {
    let class = match game.with(|g| g.get(field).0) {
        Some(Side::X) => "cell side-X",
        Some(Side::O) => "cell side-O",
        None => "cell",
    };
    let action = move |_| {
        game.with_mut(|g| {
            g.act(field);
        })
    };
    let value = game.with(|g| g.get(field).to_string());

    rsx!(
        div { class: class,
              onclick: action,
              {value}
        }
    )
}
