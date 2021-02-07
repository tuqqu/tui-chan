use tui::{
    style::Color,
    widgets::BorderType,
};

pub struct StyleProvider {
    highlight_color: Color,
    default_color: Color,
    highlight_border_type: BorderType,
    default_border_type: BorderType,
    highlight_border_color: Color,
    default_border_color: Color,
}

impl StyleProvider {
    pub fn new() -> Self {
        Self {
            highlight_color: Color::DarkGray, //todo try from color
            default_color: Color::Reset,
            highlight_border_type: BorderType::Plain,
            default_border_type: BorderType::Plain,
            highlight_border_color: Color::Blue,
            default_border_color: Color::Reset,
        }
    }

    pub fn default_from_selected_field(&self, selected_field: &SelectedField) -> BlockStyle {
        BlockStyle::from_selected_field(
            &selected_field,
            self.highlight_color,
            self.default_color,
            self.highlight_border_type,
            self.default_border_type,
            self.highlight_border_color,
            self.default_border_color,
        )
    }

    pub fn highlight_color(&self) -> &Color {
        &self.highlight_color
    }
}

pub enum SelectedField {
    BoardList,
    ThreadList,
    Thread,
}

pub struct BlockBorderColor {
    pub board_list: Color,
    pub thread_list: Color,
    pub thread: Color,
}

pub struct BlockBorderType {
    pub board_list: BorderType,
    pub thread_list: BorderType,
    pub thread: BorderType,
}

pub struct BlockStyle {
    pub border_color: BlockBorderColor,
    pub border_type: BlockBorderType,
}

impl BlockStyle {
    pub fn from_selected_field(
        selected_field: &SelectedField,
        _highlight_color: Color,
        _default_color: Color,
        highlight_border_type: BorderType,
        default_border_type: BorderType,
        highlight_border_color: Color,
        default_border_color: Color,
    ) -> Self {
        Self::new(
            BlockBorderColor::from_selected_field(selected_field, highlight_border_color, default_border_color),
            BlockBorderType::from_selected_field(selected_field, highlight_border_type, default_border_type),
        )
    }

    fn new(border_color: BlockBorderColor, border_type: BlockBorderType) -> Self {
        Self {
            border_color,
            border_type,
        }
    }
}

impl BlockBorderColor {
    fn new(board_list: Color, thread_list: Color, thread: Color) -> Self {
        Self {
            board_list,
            thread_list,
            thread,
        }
    }

    fn from_selected_field(
        selected_field: &SelectedField,
        highlight_color: Color,
        default_color: Color,
    ) -> Self {
        match selected_field {
            SelectedField::BoardList => {
                Self::new(highlight_color, default_color, default_color)
            }
            SelectedField::ThreadList => {
                Self::new(default_color, highlight_color, default_color)
            }
            SelectedField::Thread => {
                Self::new(default_color, default_color, highlight_color)
            }
        }
    }
}

impl BlockBorderType {
    fn new(board_list: BorderType, thread_list: BorderType, thread: BorderType) -> Self {
        Self {
            board_list,
            thread_list,
            thread,
        }
    }

    fn from_selected_field(
        selected_field: &SelectedField,
        highlight_border: BorderType,
        default_border: BorderType,
    ) -> Self {
        match selected_field {
            SelectedField::BoardList => {
                Self::new(highlight_border, default_border, default_border)
            }
            SelectedField::ThreadList => {
                Self::new(default_border, highlight_border, default_border)
            }
            SelectedField::Thread => {
                Self::new(default_border, default_border, highlight_border)
            }
        }
    }
}
