use jsx_macro::*;
use jsx_types::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::input;
use super::view_picker;
use super::todo_item_display;

#[derive(Clone)]
pub struct AppState {
  pub todo_items: Vec<TodoItem>,
  pub current_text: String,
  pub view: View,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum View {
  All,
  Done,
  Incomplete,
}

#[derive(Clone)]
pub struct TodoItem {
  pub text: String,
  pub is_done: bool,
}

impl AppState {
  pub fn new() -> AppState {
    AppState {
      todo_items: vec![],
      current_text: "".to_string(),
      view: View::All,
    }
  }

  // todo &mut self
  pub fn create_todo_item(state: &mut AppState) {
    let todo_item = TodoItem {
      // TODO avoid cloning
      text: state.current_text.clone(),
      is_done: false,
    };
    state.current_text = "".to_string();
    state.todo_items.push(todo_item);
  }
}

impl<'a> Component<'a, ()> for AppState {
  fn render(&'a mut self, _props: ()) -> HtmlToken<'a> {
    let self_2 = self.clone();
    let cell = Rc::new(RefCell::new(self));
    let cell_2 = cell.clone();
    let cell_3 = cell.clone();
    let cell_4 = cell.clone();

    let input_props: input::InputProps<'a> = input::InputProps {
      value: self_2.current_text,
      on_input: Box::new(move |e| {
        let mut state = cell.borrow_mut();
        if let Some(ref val) = e.value {
          state.current_text = val.to_string();
        }
      }),
      on_keydown: Box::new(move |e| {
        let mut state = cell_2.borrow_mut();
        if e.key_code == 13 {
          AppState::create_todo_item(&mut state);
        }
      }),
    };

    let view_picker_props = view_picker::ViewPickerProps {
      view: self_2.view,
      on_select_view: Box::new(
        move |view| {
          let mut state = cell_3.borrow_mut();
          state.view = view;
        }
      ),
    };

    let view = self_2.view;
    let is_visible = |todo_item: &TodoItem| {
      match view {
        View::All => true,
        View::Done => todo_item.is_done,
        View::Incomplete => !todo_item.is_done,
      }
    };

    jsx!(<div>
      Smithy Todo List
      <div>{view_picker::ViewPicker::render(view_picker_props)}</div>
      <div>{input::Input::render(input_props)}</div>
      <div>{
        self_2.todo_items
          .iter()
          // .filter(is_visible) does not work. Why?
          .filter(|t| is_visible(t))
          .map(|todo_item| {
            let cell = cell_4.clone();
            let todo_item = todo_item.clone();
            let todo_item_display_props = todo_item_display::TodoItemDisplayProps {
              todo_item,
              on_complete_item: Box::new(move || {
                let mut state = cell.borrow_mut();
                state.todo_items = vec![];
              }),
            };
            todo_item_display::TodoItemDisplay::render(todo_item_display_props)
          })
          .collect::<Vec<HtmlToken>>()
      }</div>
    </div>)
  }
}
