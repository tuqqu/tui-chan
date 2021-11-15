use tui::style::Color;
use tui::widgets::BorderType;

pub(crate) struct StyleProvider {
    highlight_color: Color,
    default_color: Color,
    highlight_border_type: BorderType,
    default_border_type: BorderType,
    highlight_border_color: Color,
    default_border_color: Color,
}

impl StyleProvider {
    pub(crate) fn new() -> Self {
        Self {
            highlight_color: Color::DarkGray,
            default_color: Color::Reset,
            highlight_border_type: BorderType::Plain,
            default_border_type: BorderType::Plain,
            highlight_border_color: Color::Blue,
            default_border_color: Color::Reset,
        }
    }

    pub(crate) fn default_from_selected_field(&self, selected_field: &SelectedField) -> BlockStyle {
        BlockStyle::from_selected_field(
            selected_field,
            self.highlight_color,
            self.default_color,
            self.highlight_border_type,
            self.default_border_type,
            self.highlight_border_color,
            self.default_border_color,
        )
    }

    pub(crate) fn highlight_color(&self) -> &Color {
        &self.highlight_color
    }
}

pub(crate) enum SelectedField {
    BoardList,
    ThreadList,
    Thread,
}

pub(crate) struct BlockBorderColor {
    board_list: Color,
    thread_list: Color,
    thread: Color,
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
            SelectedField::BoardList => Self::new(highlight_color, default_color, default_color),
            SelectedField::ThreadList => Self::new(default_color, highlight_color, default_color),
            SelectedField::Thread => Self::new(default_color, default_color, highlight_color),
        }
    }

    pub(crate) fn board_list(&self) -> Color {
        self.board_list
    }

    pub(crate) fn thread_list(&self) -> Color {
        self.thread_list
    }

    pub(crate) fn thread(&self) -> Color {
        self.thread
    }
}

pub(crate) struct BlockBorderType {
    board_list: BorderType,
    thread_list: BorderType,
    thread: BorderType,
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
            SelectedField::BoardList => Self::new(highlight_border, default_border, default_border),
            SelectedField::ThreadList => {
                Self::new(default_border, highlight_border, default_border)
            }
            SelectedField::Thread => Self::new(default_border, default_border, highlight_border),
        }
    }

    pub(crate) fn board_list(&self) -> BorderType {
        self.board_list
    }

    pub(crate) fn thread_list(&self) -> BorderType {
        self.thread_list
    }

    pub(crate) fn thread(&self) -> BorderType {
        self.thread
    }
}

pub(crate) struct BlockStyle {
    border_color: BlockBorderColor,
    border_type: BlockBorderType,
}

impl BlockStyle {
    pub(crate) fn from_selected_field(
        selected_field: &SelectedField,
        _highlight_color: Color,
        _default_color: Color,
        highlight_border_type: BorderType,
        default_border_type: BorderType,
        highlight_border_color: Color,
        default_border_color: Color,
    ) -> Self {
        Self::new(
            BlockBorderColor::from_selected_field(
                selected_field,
                highlight_border_color,
                default_border_color,
            ),
            BlockBorderType::from_selected_field(
                selected_field,
                highlight_border_type,
                default_border_type,
            ),
        )
    }

    fn new(border_color: BlockBorderColor, border_type: BlockBorderType) -> Self {
        Self {
            border_color,
            border_type,
        }
    }

    pub(crate) fn border_color(&self) -> &BlockBorderColor {
        &self.border_color
    }

    pub(crate) fn border_type(&self) -> &BlockBorderType {
        &self.border_type
    }
}
