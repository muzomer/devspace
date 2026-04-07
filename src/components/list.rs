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
        let len = self.filtered_items().len();
        if len == 0 {
            return;
        }
        let new_index = match item_order {
            ItemOrder::Next => match self.get_state().selected() {
                Some(i) if i >= len - 1 => 0,
                Some(i) => i + 1,
                None => 0,
            },
            ItemOrder::Previous => match self.get_state().selected() {
                Some(0) | None => len - 1,
                Some(i) => i - 1,
            },
            ItemOrder::Last => len - 1,
            ItemOrder::First => 0,
        };
        self.get_state().select(Some(new_index));
        self.update_selected_index(new_index);
    }

    fn filtered_items(&mut self) -> Vec<&T>;
    fn get_state(&mut self) -> &mut ListState;
    fn update_selected_index(&mut self, index: usize);
}
