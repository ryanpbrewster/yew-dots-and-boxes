#![recursion_limit = "512"]

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use log::info;

#[derive(Clone, Copy)]
pub enum Color {
    RED,
    BLUE,
}
impl Default for Color {
    fn default() -> Self {
        Color::RED
    }
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Color::RED => f.write_str("red"),
            Color::BLUE => f.write_str("blue"),
        }
    }
}


// Grid of `width` x `height` boxes.
// Box corners are at coordinates [0 .. width] x [0 .. height]
pub struct Model {
    width: usize,
    height: usize,
    boxes: Vec<Option<Color>>, // `height` x `width`, row-major
    hlines: Vec<Option<Color>>, // `height+1` x `width`, row-major
    vlines: Vec<Option<Color>>, // `height` x `width+1`, row-major
    turn: Color,
}

impl Model {
    fn new(width: usize, height: usize) -> Model {
        Model {
            width,
            height,
            boxes: vec![None; width * height],
            hlines: vec![None; width * (height + 1)],
            vlines: vec![None; (width + 1) * height],
            turn: Color::RED,
        }
    }
    fn get_vline(&self, i: usize, j: usize) -> Option<Color> {
        assert!(i < self.height);
        assert!(j <= self.width);
        self.vlines[(self.width + 1) * i + j]
    }
    fn get_hline(&self, i: usize, j: usize) -> Option<Color> {
        assert!(i <= self.height);
        assert!(j < self.width);
        self.hlines[self.width * i + j]
    }

    fn color_vline(&mut self, i: usize, j: usize, c: Color) {
        info!("coloring vertical line ({}, {}) = {}", i, j, c);
        assert!(i < self.height);
        assert!(j <= self.width);
        self.vlines[(self.width + 1) * i + j] = Some(c);
    }

    fn color_hline(&mut self, i: usize, j: usize, c: Color) {
        info!("coloring horizontal line ({}, {}) = {}", i, j, c);
        assert!(i <= self.height);
        assert!(j < self.width);
        self.hlines[self.width * i + j] = Some(c);
    }

    fn view_dots(&self) -> Html<Model> {
        html! {
            { for (0 ..= self.height).map(|i| self.view_dots_row(i)) }
        }
    }

    fn view_dots_row(&self, i: usize) -> Html<Model> {
        html! {
            { for (0 ..= self.width).map(|j| self.view_dot(i, j)) }
        }
    }

    fn view_dot(&self, i: usize, j: usize) -> Html<Model> {
        html! {
        <div class="game-dot",
             style=format!("top:{}px;left:{}px;", 64 * i, 64 * j),>
        </div>
        }
    }

    fn view_hlines(&self) -> Html<Model> {
        html! {
            { for (0 ..= self.height).map(|i| self.view_hlines_row(i)) }
        }
    }

    fn view_hlines_row(&self, i: usize) -> Html<Model> {
        html! {
            { for (0 .. self.width).map(|j| self.view_hline(i, j)) }
        }
    }

    fn view_hline(&self, i: usize, j: usize) -> Html<Model> {
        let claim = match self.get_hline(i, j) {
            None => format!("game-unclaimed-{}", self.turn),
            Some(c) => format!("game-claimed-{}", c),
        };
        let color = self.turn;
        html! {
        <div class=("game-hline", &claim),
             style=format!("top:{}px;left:{}px;", 64 * i, 64 * j),
             onclick=|_| Msg::ColorHline(i, j, color),>
        </div>
        }
    }

    fn view_vlines(&self) -> Html<Model> {
        html! {
            { for (0 .. self.height).map(|i| self.view_vlines_row(i)) }
        }
    }

    fn view_vlines_row(&self, i: usize) -> Html<Model> {
        html! {
            { for (0 ..= self.width).map(|j| self.view_vline(i, j)) }
        }
    }

    fn view_vline(&self, i: usize, j: usize) -> Html<Model> {
        let claim = match self.get_vline(i, j) {
            None => format!("game-unclaimed-{}", self.turn),
            Some(c) => format!("game-claimed-{}", c),
        };
        let color = self.turn;
        html! {
        <div class=("game-vline", &claim),
             style=format!("top:{}px;left:{}px;", 64 * i, 64 * j),
             onclick=|_| Msg::ColorVline(i, j, color),>
        </div>
        }
    }
}

pub enum Msg {
    ColorVline(usize, usize, Color),
    ColorHline(usize, usize, Color),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model::new(8, 6)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ColorHline(i, j, c) => {
                self.color_hline(i, j, c);
            },
            Msg::ColorVline(i, j, c) => {
                self.color_vline(i, j, c);
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="game-container",>
                <div id="game-dots",>
                { self.view_dots() }
                </div>

                <div id="game-hlines",>
                { self.view_hlines() }
                </div>

                <div id="game-vlines",>
                { self.view_vlines() }
                </div>
            </div>
        }
    }
}
