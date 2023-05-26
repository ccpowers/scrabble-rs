pub struct Selectable {
    pub selected: bool
}

pub trait SetSelected {
    fn set_selected(&mut self, selected: bool) -> ();
}
impl SetSelected for Selectable {
    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}