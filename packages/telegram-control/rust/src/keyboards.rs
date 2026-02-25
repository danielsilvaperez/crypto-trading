use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyKeyboardMarkup};

/// Builder for inline keyboards
#[derive(Debug, Default)]
pub struct InlineKeyboardBuilder {
    rows: Vec<Vec<InlineKeyboardButton>>,
    current_row: Vec<InlineKeyboardButton>,
}

impl InlineKeyboardBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a button to current row
    pub fn button(mut self, text: impl Into<String>, callback_data: impl Into<String>) -> Self {
        self.current_row.push(InlineKeyboardButton::callback(
            text.into(),
            callback_data.into(),
        ));
        self
    }
    
    /// Add a URL button
    pub fn url_button(mut self, text: impl Into<String>, url: impl Into<String>) -> Self {
        self.current_row.push(InlineKeyboardButton::url(
            text.into(),
            url.into().parse().expect("Invalid URL"),
        ));
        self
    }
    
    /// Finish current row and start new one
    pub fn row(mut self) -> Self {
        if !self.current_row.is_empty() {
            self.rows.push(std::mem::take(&mut self.current_row));
        }
        self
    }
    
    /// Build the keyboard
    pub fn build(mut self) -> InlineKeyboardMarkup {
        if !self.current_row.is_empty() {
            self.rows.push(self.current_row);
        }
        InlineKeyboardMarkup::new(self.rows)
    }
}

/// Builder for reply keyboards
#[derive(Debug, Default)]
pub struct ReplyKeyboardBuilder {
    rows: Vec<Vec<KeyboardButton>>,
    current_row: Vec<KeyboardButton>,
    resize: bool,
    one_time: bool,
}

impl ReplyKeyboardBuilder {
    pub fn new() -> Self {
        Self {
            resize: true,
            one_time: false,
            ..Default::default()
        }
    }
    
    /// Add a button to current row
    pub fn button(mut self, text: impl Into<String>) -> Self {
        self.current_row.push(KeyboardButton::new(text.into()));
        self
    }
    
    /// Finish current row and start new one
    pub fn row(mut self) -> Self {
        if !self.current_row.is_empty() {
            self.rows.push(std::mem::take(&mut self.current_row));
        }
        self
    }
    
    /// Set resize option
    pub fn resize(mut self, resize: bool) -> Self {
        self.resize = resize;
        self
    }
    
    /// Set one_time option
    pub fn one_time(mut self, one_time: bool) -> Self {
        self.one_time = one_time;
        self
    }
    
    /// Build the keyboard
    pub fn build(mut self) -> ReplyKeyboardMarkup {
        if !self.current_row.is_empty() {
            self.rows.push(self.current_row);
        }
        ReplyKeyboardMarkup::new(self.rows)
            .resize_keyboard(self.resize)
            .one_time_keyboard(self.one_time)
    }
}

/// Pre-built keyboard layouts
pub mod layouts {
    use super::*;
    
    /// Confirmation keyboard (Yes/No)
    pub fn confirm(action: &str) -> InlineKeyboardMarkup {
        InlineKeyboardBuilder::new()
            .button("✅ Yes", format!("{}:yes", action))
            .button("❌ No", format!("{}:no", action))
            .build()
    }
    
    /// Pagination keyboard
    pub fn pagination(current: usize, total: usize, prefix: &str) -> InlineKeyboardMarkup {
        let mut builder = InlineKeyboardBuilder::new();
        
        if current > 0 {
            builder = builder.button("⬅️ Prev", format!("{}:page:{}", prefix, current - 1));
        }
        
        builder = builder.button(
            format!("{}/{}", current + 1, total),
            format!("{}:noop", prefix),
        );
        
        if current < total - 1 {
            builder = builder.button("Next ➡️", format!("{}:page:{}", prefix, current + 1));
        }
        
        builder.build()
    }
    
    /// Trading action keyboard
    pub fn trading_actions(symbol: &str) -> InlineKeyboardMarkup {
        InlineKeyboardBuilder::new()
            .button(format!("📈 Buy {}", symbol), format!("trade:buy:{}", symbol))
            .button(format!("📉 Sell {}", symbol), format!("trade:sell:{}", symbol))
            .row()
            .button("📊 View Chart", format!("chart:{}", symbol))
            .button("⚙️ Settings", "settings:open")
            .build()
    }
    
    /// Main menu keyboard
    pub fn main_menu() -> ReplyKeyboardMarkup {
        ReplyKeyboardBuilder::new()
            .button("📊 Dashboard")
            .button("💼 Portfolio")
            .row()
            .button("📈 Markets")
            .button("⚙️ Settings")
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_inline_keyboard() {
        let kb = InlineKeyboardBuilder::new()
            .button("Btn1", "cb1")
            .button("Btn2", "cb2")
            .row()
            .button("Btn3", "cb3")
            .build();
        
        assert_eq!(kb.inline_keyboard.len(), 2);
        assert_eq!(kb.inline_keyboard[0].len(), 2);
        assert_eq!(kb.inline_keyboard[1].len(), 1);
    }
}
