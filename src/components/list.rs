use log::debug;
use ratatui::widgets::ListState;

pub enum Focus {
    Filter,
    List,
}

pub enum ItemOrder {
    Next,
    Previous,
    Last,
    First,
}

pub trait ListComponent<T> {
    fn select(&mut self, item_order: ItemOrder) {
        match item_order {
            ItemOrder::Next => {
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
            ItemOrder::Previous => {
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
            ItemOrder::Last => self.get_state().select_last(),
            ItemOrder::First => self.get_state().select_first(),
        }

        if let Some(selected_index) = self.get_state().selected() {
            debug!("Updating selected index to {}", selected_index);
            self.update_selected_index(selected_index);
        }
    }

    fn filtered_items(&mut self) -> Vec<&T>;
    fn get_state(&mut self) -> &mut ListState;
    fn update_selected_index(&mut self, index: usize);
}
