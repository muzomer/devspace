use ratatui::widgets::ListState;

pub enum Focus {
    Filter,
    List,
}

pub trait ListComponent<T> {
    fn select_next(&mut self) {
        if let Some(index) = self.get_state().selected() {
            if index == self.filtered_items().len() - 1 {
                self.get_state().select_first();
            } else {
                self.get_state().select_next();
            }
        } else {
            self.get_state().select_first();
        }
    }
    fn select_previous(&mut self) {
        if let Some(index) = self.get_state().selected() {
            if index == 0 {
                self.get_state().select_last();
            } else {
                self.get_state().select_previous();
            }
        } else {
            self.get_state().select_last();
        }
    }
    fn select_first(&mut self) {
        self.get_state().select_first();
    }

    fn select_last(&mut self) {
        self.get_state().select_last();
    }

    fn filtered_items(&mut self) -> Vec<&T>;
    fn get_state(&mut self) -> &mut ListState;
}
