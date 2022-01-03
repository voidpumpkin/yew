use rand::prelude::*;
use std::cmp::min;
use yew::prelude::*;

use crate::jumbotron::Jumbotron;
use crate::row::Row;
use crate::row_data::RowData;

pub struct App {
    rows: Vec<RowData>,
    next_id: usize,
    selected_id: Option<usize>,
    rng: SmallRng,
    on_select: Callback<usize>,
    on_remove: Callback<usize>,
}

pub enum Msg {
    Run(usize),
    Add(usize),
    Update(usize),
    Clear,
    Swap,
    Remove(usize),
    Select(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App {
            rows: Vec::new(),
            next_id: 1,
            selected_id: None,
            rng: SmallRng::from_entropy(),
            on_select: ctx.link().callback(Msg::Select),
            on_remove: ctx.link().callback(Msg::Remove),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Run(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                let update_amount = min(amount, self.rows.len());
                for index in 0..update_amount {
                    self.rows[index] = RowData::new(next_id + index, rng);
                }
                self.rows.extend(
                    (update_amount..amount).map(|index| RowData::new(next_id + index, rng)),
                );
                self.next_id += amount;
            }
            Msg::Add(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                self.rows
                    .extend((0..amount).map(|index| RowData::new(next_id + index, rng)));
                self.next_id += amount;
            }
            Msg::Update(step) => {
                for index in (0..self.rows.len()).step_by(step) {
                    self.rows[index].label += " !!!";
                }
            }
            Msg::Clear => {
                self.rows.clear();
            }
            Msg::Swap => {
                if self.rows.len() > 998 {
                    self.rows.swap(1, 998);
                }
            }
            Msg::Remove(id) => {
                if let Some(index) = self.rows.iter().position(|row| row.id == id) {
                    self.rows.remove(index);
                }
            }
            Msg::Select(id) => {
                self.selected_id = Some(id);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let rows: Html = self
            .rows
            .iter()
            .map(|row| {
                html! {
                    <Row
                        key={row.id}
                        data={row.clone()}
                        selected={self.selected_id == Some(row.id)}
                        on_select={self.on_select.clone()}
                        on_remove={self.on_remove.clone()}
                    />
                }
            })
            .collect();

        html! {
            <div class="container">
                <Jumbotron
                    on_run={ctx.link().callback(Msg::Run)}
                    on_add={ctx.link().callback(Msg::Add)}
                    on_update={ctx.link().callback(Msg::Update)}
                    on_clear={ctx.link().callback(|_| Msg::Clear)}
                    on_swap={ctx.link().callback(|_| Msg::Swap)}
                />
                <table class="table table-hover table-striped test-data">
                    <tbody id="tbody">
                        { rows }
                    </tbody>
                </table>
                <span class="preloadicon glyphicon glyphicon-remove" aria-hidden="true"></span>
            </div>
        }
    }
}
