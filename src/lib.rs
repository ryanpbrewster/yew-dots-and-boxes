#![recursion_limit = "512"]

use log::info;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(Clone, Copy, PartialEq)]
enum LifeState {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
struct Cellule {
    life_state: LifeState,
}

pub struct Model {
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
}

impl Cellule {
    pub fn set_alive(&mut self) {
        self.life_state = LifeState::Alive;
    }

    pub fn set_dead(&mut self) {
        self.life_state = LifeState::Dead;
    }

    pub fn alive(&self) -> bool {
        self.life_state == LifeState::Alive
    }

    pub fn count_alive_neighbors(neighbors: &[Cellule]) -> usize {
        neighbors.iter().filter(|n| n.alive()).count()
    }

    pub fn alone(neighbors: &[Cellule]) -> bool {
        Self::count_alive_neighbors(neighbors) < 2
    }

    pub fn overpopulated(neighbors: &[Cellule]) -> bool {
        Self::count_alive_neighbors(neighbors) > 3
    }

    pub fn can_be_revived(neighbors: &[Cellule]) -> bool {
        Self::count_alive_neighbors(neighbors) == 3
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        (coord + range)
    } else if coord >= range {
        (coord - range)
    } else {
        coord
    };
    result as usize
}

impl Model {
    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    pub fn step(&mut self) {
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let neighbors = self.neighbors(row as isize, col as isize);

                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                if self.cellules[current_idx].alive() {
                    if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else {
                    if Cellule::can_be_revived(&neighbors) {
                        to_live.push(current_idx);
                    }
                }
            }
        }
        to_dead
            .iter()
            .for_each(|idx| self.cellules[*idx].set_dead());
        to_live
            .iter()
            .for_each(|idx| self.cellules[*idx].set_alive());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cellule; 8] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col)],
            self.cellules[self.row_col_as_idx(row + 1, col + 1)],
            self.cellules[self.row_col_as_idx(row + 1, col - 1)],
            self.cellules[self.row_col_as_idx(row - 1, col)],
            self.cellules[self.row_col_as_idx(row - 1, col + 1)],
            self.cellules[self.row_col_as_idx(row - 1, col - 1)],
            self.cellules[self.row_col_as_idx(row, col - 1)],
            self.cellules[self.row_col_as_idx(row, col + 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }

    fn toggle_cellule(&mut self, i: usize, j: usize) {
        let cellule = self.cellules.get_mut(i * self.cellules_width + j).unwrap();

        cellule.life_state = match cellule.life_state {
            LifeState::Alive => LifeState::Dead,
            LifeState::Dead => LifeState::Alive,
        };
    }

    fn view_cellule_grid(&self) -> Html<Model> {
        html! {
            <table>
            { for (0 .. self.cellules_height).map(|i| self.view_cellule_row(i)) }
            </table>
        }
    }

    fn view_cellule_row(&self, i: usize) -> Html<Model> {
        html! {
            <tr>
            { for (0 .. self.cellules_width).map(|j| self.view_cellule(i, j)) }
            </tr>
        }
    }

    fn view_cellule(&self, i: usize, j: usize) -> Html<Model> {
        let cellule = self.cellules[i * self.cellules_width + j];
        let cellule_status = match cellule.life_state {
            LifeState::Alive => "cellule-live",
            LifeState::Dead => "cellule-dead",
        };
        html! {
        <div class=("game-cellule", cellule_status),
            onclick=|_| Msg::ToggleCellule(i, j),> </div>
        }
    }
}

pub enum Msg {
    Step,
    Reset,
    ToggleCellule(usize, usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            cellules: vec![
                Cellule {
                    life_state: LifeState::Dead
                };
                2000
            ],
            cellules_width: 50,
            cellules_height: 40,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Step => {
                self.step();
            }
            Msg::Reset => {
                self.reset();
                info!("Reset");
            }
            Msg::ToggleCellule(i, j) => {
                self.toggle_cellule(i, j);
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <section class="game-container",>
                    <header class="app-header",>
                        <h1 class="app-title",>{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area",>
                        <div class="game-of-life",>
                            { self.view_cellule_grid() }
                        </div>
                        <div class="game-buttons",>
                            <button class="game-button", onclick=|_| Msg::Step,>{ "Step" }</button>
                            <button class="game-button", onclick=|_| Msg::Reset,>{ "Reset" }</button>
                        </div>
                    </section>
                </section>
                <footer class="app-footer",>
                    <strong class="footer-text",>
                      { "Game of Life - a yew experiment" }
                    </strong>
                </footer>
            </div>
        }
    }
}
