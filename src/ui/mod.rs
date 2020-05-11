use cursive::view::{SizeConstraint, View};
use cursive::views::{LinearLayout, PaddedView, Panel, ResizedView, ScrollView, SelectView};
use cursive::Cursive;

pub struct AppUi {
  root: Cursive,
}

fn apps_view() -> Box<dyn View> {
  let apps_view = SelectView::new()
    .item("kafka", 1)
    .item("zookeeper", 1)
    .item("postgres", 1)
    .item("elasticsearch", 1)
    .autojump()
    .on_select(|_siv, _item| {});

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

impl AppUi {
  pub fn new() -> AppUi {
    let mut siv = Cursive::default();
    siv
      .load_toml(include_str!("../../assets/style.toml"))
      .unwrap();

    let panel = LinearLayout::vertical()
      .child(apps_view())
      .child(control_view());

    let right_panel = Panel::new(SelectView::new().item("A", 2))
      .title("Recent Logs")
      .title_position(cursive::align::HAlign::Left);
    let resized_right_panel = ResizedView::new(
      cursive::view::SizeConstraint::Full,
      cursive::view::SizeConstraint::Full,
      right_panel,
    );

    let layout = LinearLayout::horizontal()
      .child(panel)
      .child(resized_right_panel);

    siv.add_layer(layout);

    AppUi { root: siv }
  }

  pub fn start(&mut self) {
    self.root.run();
  }
}
