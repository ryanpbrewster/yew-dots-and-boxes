#![recursion_limit = "512"]

use log::info;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(Clone, Copy)]
pub enum Color {
    RED,
    BLUE,
}
impl Color {
    fn next(&self) -> Color {
        match *self {
            Color::RED => Color::BLUE,
            Color::BLUE => Color::RED,
        }
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
    hlines: Vec<bool>,         // `height+1` x `width`, row-major
    vlines: Vec<bool>,         // `height` x `width+1`, row-major
    turn: Color,
}

impl Model {
    fn new(width: usize, height: usize) -> Model {
        Model {
            width,
            height,
            boxes: vec![None; width * height],
            hlines: vec![false; width * (height + 1)],
            vlines: vec![false; (width + 1) * height],
            turn: Color::RED,
        }
    }
    fn get_vline(&self, i: usize, j: usize) -> bool {
        assert!(i < self.height);
        assert!(j <= self.width);
        self.vlines[(self.width + 1) * i + j]
    }
    fn get_hline(&self, i: usize, j: usize) -> bool {
        assert!(i <= self.height);
        assert!(j < self.width);
        self.hlines[self.width * i + j]
    }
    fn get_box(&self, i: usize, j: usize) -> Option<Color> {
        assert!(i < self.height);
        assert!(j < self.width);
        self.boxes[self.width * i + j]
    }
    fn is_box_filled(&self, i: usize, j: usize) -> bool {
        assert!(i < self.height);
        assert!(j < self.width);
        let ans = self.get_hline(i, j)
            && self.get_hline(i + 1, j)
            && self.get_vline(i, j)
            && self.get_vline(i, j + 1);
        info!("box ({},{}) ? {}", i, j, ans);
        ans
    }

    fn color_vline(&mut self, i: usize, j: usize, c: Color) {
        info!("coloring vertical line ({}, {}) = {}", i, j, c);
        assert!(i < self.height);
        assert!(j <= self.width);

        let filled = &mut self.vlines[(self.width + 1) * i + j];
        if *filled {
            return;
        }
        *filled = true;

        let mut num_converted = 0;
        if self.is_box_filled(i, j) {
            self.boxes[self.width * i + j] = Some(c);
            num_converted += 1;
        }
        if j > 0 && self.is_box_filled(i, j - 1) {
            self.boxes[self.width * i + j - 1] = Some(c);
            num_converted += 1;
        }

        if num_converted == 0 {
            self.turn = self.turn.next();
        } else {
            info!("converted {} boxes", num_converted);
        }
    }

    fn color_hline(&mut self, i: usize, j: usize, c: Color) {
        info!("coloring horizontal line ({}, {}) = {}", i, j, c);
        assert!(i <= self.height);
        assert!(j < self.width);

        let filled = &mut self.hlines[self.width * i + j];
        if *filled {
            return;
        }
        *filled = true;

        let mut num_converted = 0;
        if self.is_box_filled(i, j) {
            self.boxes[self.width * i + j] = Some(c);
            num_converted += 1;
        }
        if i > 0 && self.is_box_filled(i - 1, j) {
            self.boxes[self.width * (i - 1) + j] = Some(c);
            num_converted += 1;
        }

        if num_converted == 0 {
            self.turn = self.turn.next();
        } else {
            info!("converted {} boxes", num_converted);
        }
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
        let fill = if self.get_hline(i, j) {
            "game-line-full"
        } else {
            "game-line-empty"
        };
        let turn = format!("game-turn-{}", self.turn);
        let color = self.turn;
        html! {
        <div class=("game-hline", fill, turn),
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
        let fill = if self.get_vline(i, j) {
            "game-line-full"
        } else {
            "game-line-empty"
        };
        let turn = format!("game-turn-{}", self.turn);
        let color = self.turn;
        html! {
        <div class=("game-vline", fill, turn),
             style=format!("top:{}px;left:{}px;", 64 * i, 64 * j),
             onclick=|_| Msg::ColorVline(i, j, color),>
        </div>
        }
    }

    fn view_boxes(&self) -> Html<Model> {
        html! {
            { for (0 .. self.height).map(|i| self.view_boxes_row(i)) }
        }
    }

    fn view_boxes_row(&self, i: usize) -> Html<Model> {
        html! {
            { for (0 .. self.width).map(|j| self.view_box(i, j)) }
        }
    }

    fn view_box(&self, i: usize, j: usize) -> Html<Model> {
        let fill = match self.get_box(i, j) {
            None => format!("game-box-empty"),
            Some(c) => format!("game-box-full-{}", c),
        };
        html! {
        <div class=("game-box", fill),
             style=format!("top:{}px;left:{}px;", 64 * i, 64 * j),>
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
            }
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

                <div id="game-boxes",>
                { self.view_boxes() }
                </div>
            </div>
        }
    }
}
