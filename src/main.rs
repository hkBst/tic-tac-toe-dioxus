use dioxus::prelude::*;
use tic_tac_toe::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let game = use_ref(cx, Game::new);

    let board = [Hor::Left, Hor::Mid, Hor::Right].into_iter().flat_map(|h| {
        [Vert::Top, Vert::Mid, Vert::Bottom]
            .into_iter()
            .map(move |v| {
                rsx!(Field {
                    game: game.clone(),
                    field: FieldName { v, h }
                })
            })
    });

    cx.render(rsx!(
        div { class: "game",
            div { class: "board", board }
            div { class: "state", game.with(|g| g.state().to_string()) }
        }
    ))
}

#[inline_props]
#[allow(non_snake_case)]
fn Field(cx: Scope, game: UseRef<Game>, field: FieldName) -> Element {
    let class = match game.with(|g| g.get(*field).0) {
        Some(Side::X) => "cell side-X",
        Some(Side::O) => "cell side-O",
        None => "cell",
    };
    let action = move |_| {
        game.with_mut(|g| {
            g.act(*field);
        })
    };
    let value = game.with(|g| g.get(*field).to_string());

    cx.render(rsx!(div {
        class: class,
        onclick: action,
        value
    }))
}
