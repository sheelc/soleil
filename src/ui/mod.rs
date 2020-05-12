use cursive::view::{SizeConstraint, View, Nameable};
use cursive::views::{LinearLayout, PaddedView, Panel, ResizedView, ScrollView, SelectView};
use cursive::Cursive;

use std::sync::mpsc::{Sender, Receiver};

use crate::apps::AppEvent;

pub struct Ui {
  siv: Cursive,
  app_events: Receiver<AppEvent>,
}

struct UiState {
  ui_events: Sender<UiEvent>,
}

#[derive(Debug)]
pub enum UiEvent {
  SelectApp(String),
}

impl Ui {
    pub fn new(app_events: Receiver<AppEvent>, ui_events: Sender<UiEvent>) -> Ui {
    let mut ui = Ui { siv: Cursive::default(), app_events };
    let siv = &mut ui.siv;

    siv.set_user_data(UiState { ui_events });
    siv
      .load_toml(include_str!("../../assets/style.toml"))
      .unwrap();

    let panel = LinearLayout::vertical()
      .child(apps_view())
      .child(control_view());

    let right_panel = Panel::new(SelectView::new().item("A", 2))
      .title("Recent Logs")
      .title_position(cursive::align::HAlign::Left)
      .with_name("log_panel");
    let resized_right_panel = ResizedView::new(
      SizeConstraint::Full,
      SizeConstraint::Full,
      right_panel,
    );

    let layout = LinearLayout::horizontal()
      .child(panel)
      .child(resized_right_panel);

    siv.add_layer(layout);

    ui
  }

  pub fn start(&mut self) {
    let siv = &mut self.siv;
    siv.refresh();

    while siv.is_running() {
      siv.step();
      let mut needs_refresh = false;
      for m in self.app_events.try_iter() {
        siv.call_on_name("log_panel", |view: &mut Panel<SelectView<i32>>| {
          let AppEvent::SelectedApp(appid) = m;
          view.set_title(appid);
          needs_refresh = true;
        });
      }

      if needs_refresh {
        siv.refresh();
      }
    }
  }
}

fn apps_view() -> Box<dyn View> {
  let apps_view = SelectView::new()
    .item("kafka", "kafka")
    .item("zookeeper", "zookeeper")
    .item("postgres", "postgres")
    .item("elasticsearch", "elasticsearch")
    .autojump()
    .on_select(|siv, item| {
      siv.with_user_data(|state: &mut UiState| {
        state.ui_events.send(UiEvent::SelectApp(item.to_string()))
      });
    });

  let panel = Panel::new(ScrollView::new(apps_view))
    .title("Apps")
    .title_position(cursive::align::HAlign::Left);

  Box::new(ResizedView::new(
    SizeConstraint::Fixed(20),
    SizeConstraint::AtMost(20),
    PaddedView::lrtb(1, 2, 0, 1, panel),
  ))
}

fn control_view() -> Box<dyn View> {
  let panel = Panel::new(
    SelectView::new()
      .item("start", 1)
      .item("stop", 1)
      .item("restart", 1),
  )
    .title("Actions")
    .title_position(cursive::align::HAlign::Left);

  Box::new(ResizedView::with_fixed_width(
    20,
    PaddedView::lrtb(1, 2, 0, 0, panel),
  ))
}
