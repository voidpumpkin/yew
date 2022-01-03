use yew::prelude::*;

use crate::row_data::RowData;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_select: Callback<usize>,
    pub on_remove: Callback<usize>,
    pub selected: bool,
    pub data: RowData,
}

pub struct Row {
    on_select: Callback<MouseEvent>,
    on_remove: Callback<MouseEvent>,
}

impl Component for Row {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().data.id;
        Self {
            on_select: ctx.props().on_select.reform(move |_| id),
            on_remove: ctx.props().on_remove.reform(move |_| id),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let id = ctx.props().data.id;
        self.on_select = ctx.props().on_select.reform(move |_| id);
        self.on_remove = ctx.props().on_remove.reform(move |_| id);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <tr class={if ctx.props().selected { "danger" } else  { "" }}>
                <td class="col-md-1">{ ctx.props().data.id }</td>
                <td class="col-md-4" onclick={self.on_select.clone()}>
                    <a class="lbl">{ ctx.props().data.label.clone() }</a>
                </td>
                <td class="col-md-1">
                    <a class="remove" onclick={self.on_remove.clone()}>
                        <span class="glyphicon glyphicon-remove remove" aria-hidden="true"></span>
                    </a>
                </td>
                <td class="col-md-6"></td>
            </tr>
        }
    }
}
