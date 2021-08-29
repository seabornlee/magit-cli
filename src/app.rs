pub struct App<'a> {
    pub should_quit: bool,
    pub title: &'a str,
}

impl<'a> App<'a> {
    pub(crate) fn new(title: &str) -> App {
        App {
            should_quit: false,
            title
        }
    }

    pub fn on_key(&mut self, _c: char) {

    }

    pub fn on_tick(&mut self) {

    }
}