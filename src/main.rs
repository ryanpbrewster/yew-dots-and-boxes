use dots_and_boxes::Model;
use log::trace;
use yew::App;

fn main() {
    web_logger::init();
    trace!("Initializing yew...");
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
