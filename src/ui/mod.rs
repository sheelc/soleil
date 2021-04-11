use cursive::backends::curses::n::Backend;
use cursive::view::{Nameable, SizeConstraint, View};
use cursive::views::{LinearLayout, PaddedView, Panel, ResizedView, ScrollView, SelectView};
use cursive::{Cursive, CursiveRunner};

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use crate::apps::{AppActionCmd, AppEvent, AppsConfig};

type AppId = String;

#[derive(Debug)]
pub enum UiEvent {
  SelectApp(String),
  AppAction(AppId, AppActionCmd),
}

pub struct Ui {
  siv: CursiveRunner<Cursive>,
  apps_config: Arc<AppsConfig>,
  app_events: Receiver<AppEvent>,
}

struct UiState {
  ui_events: Sender<UiEvent>,
  selected_app_id: AppId,
}

impl Ui {
  pub fn new(
    apps_config: Arc<AppsConfig>,
    app_events: Receiver<AppEvent>,
    ui_events: Sender<UiEvent>,
  ) -> Ui {
    let mut ui = Ui {
      siv: Cursive::default().into_runner(Backend::init().unwrap()),
      apps_config,
      app_events,
    };
    let siv = &mut ui.siv;

    let selected_app_id = String::from("kafka");
    siv.set_user_data(UiState {
      ui_events,
      selected_app_id,
    });
    siv
      .load_toml(include_str!("../../assets/style.toml"))
      .unwrap();

    let panel = LinearLayout::vertical()
      .child(apps_view(&ui.apps_config))
      .child(control_view());

    let right_panel = Panel::new(SelectView::new().item("A", 2))
      .title("Recent Logs")
      .title_position(cursive::align::HAlign::Left)
      .with_name("log_panel");
    let resized_right_panel =
      ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, right_panel);

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
      for msg in self.app_events.try_iter() {
        match msg {
          AppEvent::SelectedApp(appid) => {
            siv.call_on_name("log_panel", |view: &mut Panel<SelectView<i32>>| {
              view.set_title(appid.clone());
              needs_refresh = true;
            });
            siv.with_user_data(|state: &mut UiState| {
              state.selected_app_id = appid;
            });
          }
        }
      }

      if needs_refresh {
        siv.refresh();
      }
    }
  }
}

fn apps_view(apps_config: &Arc<AppsConfig>) -> Box<dyn View> {
  let mut apps_view = SelectView::new();

  for app in apps_config.apps().iter() {
    apps_view = apps_view.item(app.name.clone(), app.name.clone());
  }

  apps_view = apps_view.autojump().on_select(|siv, item| {
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
      .item("Start", AppActionCmd::Start)
      .item("Stop", AppActionCmd::Stop)
      .item("Restart", AppActionCmd::Restart)
      .on_submit(|siv: &mut Cursive, item: &AppActionCmd| {
        siv.with_user_data(|state: &mut UiState| {
          state
            .ui_events
            .send(UiEvent::AppAction(
              state.selected_app_id.clone(),
              item.clone(),
            ))
            .unwrap();
        });
      }),
  )
  .title("Actions")
  .title_position(cursive::align::HAlign::Left);

  Box::new(ResizedView::with_fixed_width(
    20,
    PaddedView::lrtb(1, 2, 0, 0, panel),
  ))
}
