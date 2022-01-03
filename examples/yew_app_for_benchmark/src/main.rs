mod app;
mod constants;
mod jumbotron;
mod row;
mod row_data;

pub fn main() {
    let document = gloo_utils::document();
    let mount_el = document.query_selector("#main").unwrap().unwrap();
    yew::start_app_in_element::<app::App>(mount_el);
}
