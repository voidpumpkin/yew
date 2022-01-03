use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_run: Callback<usize>,
    pub on_add: Callback<usize>,
    pub on_update: Callback<usize>,
    pub on_clear: Callback<()>,
    pub on_swap: Callback<()>,
}

pub struct Jumbotron {}

impl Component for Jumbotron {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="jumbotron">
                <div class="row">
                    <div class="col-md-6">
                        <h1>{ "Yew" }</h1>
                    </div>
                    <div class="col-md-6">
                        <div class="row">
                            <div class="col-sm-6 smallpad">
                                <button type="button" id="run" class="btn btn-primary btn-block" onclick={ctx.props().on_run.reform(|_| 1_000)}>{ "Create 1,000 rows" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_run.reform(|_| 10_000)} id="runlots">{ "Create 10,000 rows" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_add.reform(|_| 1_000)} id="add">{ "Append 1,000 rows" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_update.reform(|_| 10)} id="update">{ "Update every 10th row" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_clear.reform(|_| ())} id="clear">{ "Clear" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_swap.reform(|_| ())} id="swaprows">{ "Swap Rows" }</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
