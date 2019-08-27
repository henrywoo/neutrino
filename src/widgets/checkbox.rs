use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

/// # Checkbox
///
/// A togglable checkbox with a label.
///
/// ## Fields
///
/// ```text
/// pub struct CheckBox {
///     name: String,
///     checked: bool,
///     text: String,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_checkbox = CheckBox::new("my_checkbox")
///     .text("Toggle me !")
///     .checked(true)
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct CheckBox {
    name: String,
    checked: bool,
    text: String,
    listener: Option<Box<dyn Listener>>,
    observer: Option<Box<dyn Observer>>,
    stretch: String,
}

impl CheckBox {
    /// Create a CheckBox
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// checked: false,
    /// text: "CheckBox".to_string(),
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        CheckBox {
            name: name.to_string(),
            checked: false,
            text: "CheckBox".to_string(),
            listener: None,
            observer: None,
            stretch: "".to_string(),
        }
    }

    /// Set the checked flag
    pub fn checked(self, checked: bool) -> Self {
        CheckBox {
            name: self.name,
            checked: checked,
            text: self.text,
            listener: self.listener,
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the label
    pub fn text(self, text: &str) -> Self {
        CheckBox {
            name: self.name,
            checked: self.checked,
            text: text.to_string(),
            listener: self.listener,
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn Listener>) -> Self {
        CheckBox {
            name: self.name,
            checked: self.checked,
            text: self.text,
            listener: Some(listener),
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<dyn Observer>) -> Self {
        CheckBox {
            name: self.name,
            checked: self.checked,
            text: self.text,
            listener: self.listener,
            observer: Some(observer),
            stretch: self.stretch,
        }
    }

    pub fn stretch(self) -> Self {
        CheckBox {
            name: self.name,
            checked: self.checked,
            text: self.text,
            listener: self.listener,
            observer: self.observer,
            stretch: "stretch".to_string(),
        }
    }
}

impl Widget for CheckBox {
    /// Return the HTML representation
    ///
    /// # Events
    ///
    /// ```text
    /// click -> ""
    /// ```
    ///
    /// # Styling
    ///
    /// ```text
    /// class = checkbox
    /// class = checkbox-outer [checked]
    /// class = checkbox-inner [checked]
    /// ```
    fn eval(&self) -> String {
        let checked = if self.checked { "checked" } else { "" };
        format!(
            r#"<div class="checkbox {}" onmousedown="{}"><div class="checkbox-outer {}"><div class="checkbox-inner {}"></div></div><label>{}</label></div>"#, 
            self.stretch,
            Event::change_js(&self.name, "''"), 
            checked, 
            checked, 
            self.text,
        )
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```text
    /// update -> self.on_update()
    /// click -> self.checked = != self.checked
    ///          self.listener.on_click()
    /// ```
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.checked = !self.checked;
                    match &self.listener {
                        None => (),
                        Some(listener) => {
                            listener.on_change(value);
                        }
                    }
                }
            },
            _ => (),
        }
    }

    /// Set the values of the widget using the fields of the HashMap
    /// returned by the observer
    ///
    /// # Fields
    ///
    /// ```text
    /// text
    /// checked
    /// ```
    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                let hash = observer.observe();
                self.text = hash["text"].to_string();
                self.checked = hash["checked"].parse().unwrap();
            }
        }
    }
}
