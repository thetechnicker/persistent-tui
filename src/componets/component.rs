use crate::widgets::Widget;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

type WidgetType = Rc<RefCell<dyn Widget>>;

#[derive(Debug, Clone)]
pub enum Component {
    Widget(WidgetType),
    ListView(Rc<[Component]>),
    GridView(Rc<[Rc<[Component]>]>),
    Floating(WidgetType),
}

impl<T> From<&[T]> for Component
where
    T: Widget + 'static + Clone,
{
    fn from(from: &[T]) -> Self {
        let from_vec: Vec<Component> = from.to_vec().into_iter().map(|i| i.into()).collect();

        let from_box = from_vec.into_boxed_slice();
        Self::ListView(Rc::from(from_box))
    }
}

impl<T, const N: usize> From<[T; N]> for Component
where
    T: Widget + 'static + Clone,
{
    fn from(from: [T; N]) -> Self {
        let from_vec: Vec<Component> = from.to_vec().into_iter().map(|i| i.into()).collect();

        let from_box = from_vec.into_boxed_slice();
        Self::ListView(Rc::from(from_box))
    }
}

impl<T> From<T> for Component
where
    T: Widget + 'static,
{
    fn from(from: T) -> Self {
        Self::Widget(from.boxed())
    }
}

// Iterator that recursively traverses WidgetElement and yields Rc<RefCell<dyn Widget>>
pub struct WidgetElementIter<'a> {
    // Stack of slices iterators for recursive traversal
    stack: Vec<Iter<'a, Component>>,
    // Current item if at leaf
    current_item: Option<&'a Rc<RefCell<dyn Widget>>>,
}

impl<'a> WidgetElementIter<'a> {
    pub fn new(root: &'a Component) -> Self {
        let mut stack = Vec::new();
        let current_item = match root {
            Component::Floating(item) | Component::Widget(item) => Some(item),
            Component::ListView(collection) => {
                stack.push(collection.iter());
                None
            }
            Component::GridView(collection) => {
                for iter in collection.iter() {
                    stack.push(iter.iter());
                }
                None
            }
        };
        Self {
            stack,
            current_item,
        }
    }
}

impl<'a> Iterator for WidgetElementIter<'a> {
    type Item = &'a Rc<RefCell<dyn Widget>>;

    fn next(&mut self) -> Option<Self::Item> {
        // If currently at an item, return it once and clear
        if let Some(item) = self.current_item.take() {
            return Some(item);
        }

        // Otherwise, iterate through the stack of collections
        while let Some(top_iter) = self.stack.last_mut() {
            if let Some(next_element) = top_iter.next() {
                match next_element {
                    Component::Floating(item) | Component::Widget(item) => {
                        // Return the current item reference
                        return Some(item);
                    }
                    Component::ListView(collection) => {
                        // Push the iterator of this collection onto stack
                        self.stack.push(collection.iter());
                    }
                    Component::GridView(collection) => {
                        // Push the iterator of this collection onto stack
                        for iter in collection.iter() {
                            self.stack.push(iter.iter());
                        }
                    }
                }
            } else {
                // Current iterator exhausted, pop it off
                self.stack.pop();
            }
        }
        // Exhausted all
        None
    }
}

// Convenience method for usage
impl Component {
    pub fn iter(&self) -> WidgetElementIter<'_> {
        WidgetElementIter::new(self)
    }

    pub fn num_rows(&self) -> usize {
        match self {
            Component::Item(i) => i.borrow().get_len(),
            Component::Collection(c) => c.len(),
            Component::CollectionWithLongElement(c, long_item) => {
                c.len() + c[*long_item].num_rows()
            }
        }
    }
    pub fn num_col(&self, row: usize) -> usize {
        match self {
            Component::Item(i) => i.borrow().get_len(),
            Component::Collection(c) => c[row].num_rows(),
            Component::CollectionWithLongElement(c, long_item) => {
                c.len() + c[*long_item].num_rows()
            }
        }
    }

    pub fn get_widget(&self, indecies: &[usize]) -> Option<Rc<RefCell<dyn Widget>>> {
        let mut current_item: Self = self.clone();
        for index in indecies {
            current_item = match current_item {
                Self::Item(item) => Self::Item(item.clone()),
                Self::Collection(collection) => collection[*index].clone(),
                Self::CollectionWithLongElement(collection, _) => collection[*index].clone(),
            };
            if let Self::Item(item) = current_item {
                return Some(item);
            }
        }
        None
    }

    pub fn get_item_2d(&self, row: usize, column: usize) -> Option<Rc<RefCell<dyn Widget>>> {
        match self {
            Component::CollectionWithLongElement(c, _) | Component::Collection(c) => {
                match c[row].clone() {
                    Component::CollectionWithLongElement(c, _) | Component::Collection(c) => {
                        match c[column].clone() {
                            Component::CollectionWithLongElement(_, _)
                            | Component::Collection(_) => None,
                            Component::Item(item) => Some(item.clone()),
                        }
                    }
                    Component::Item(item) => Some(item.clone()),
                }
            }
            Component::Item(item) => Some(item.clone()),
        }
    }
}

#[macro_export]
macro_rules! widget_element {
    // Match an array (slice) of widgets/elements, recursively construct WidgetElement::Collection
    ([ $($elem:tt),* $(,)? ]) => {{
        let elements = vec![
            $(widget_element!($elem)),*
        ];
        Component::ListView(Rc::from(elements.into_boxed_slice()))
    }};
    // For a single widget, convert it using From implementation into WidgetElement::Item
    ($item:expr) => {{
        Component::from($item)
    }};
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let a = crate::widgets::Button::new("abc", 'c', "abc");
        let b = crate::widgets::Button::new("abc", 'c', "abc");
        let c = crate::widgets::Button::new("abc", 'c', "abc");
        let d = crate::widgets::Button::new("abc", 'c', "abc");
        let y = widget_element!([a, [b, c], d]);
        assert!(matches!(y, Component::Collection(_)));
        assert!(matches!(y[0], Component::Item(_)));
        assert!(matches!(y[1], Component::Collection(_)));
        assert!(matches!(y[1][0], Component::Item(_)));
        assert!(matches!(y[1][1], Component::Item(_)));
        assert!(matches!(y[2], Component::Item(_)));
    }
}
