use crate::db::Db;
use chrono::{DateTime, Duration, Utc};
use ratatui::widgets::TableState;

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Add,
    Exit,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Name,
    Description,
}

#[derive(Debug)]
pub struct App {
    pub timers: Vec<Timer>,
    pub name_input: String,
    pub description_input: String,
    pub currently_editing: Option<CurrentlyEditing>,
    pub current_screen: CurrentScreen,
    pub(crate) state: TableState,
    pub selectable_rows: Vec<bool>,
    pub db: Db,
}

#[derive(Debug)]
pub struct Timer {
    pub start_time: DateTime<Utc>,
    pub name: String,
    pub(crate) duration: Duration,
    pub description: String,
    pub id: usize,
    pub running: bool,
}

impl App {
    pub fn new(path: &str) -> Self {
        App {
            state: TableState::default().with_selected(1),
            timers: Vec::new(),
            current_screen: CurrentScreen::Main,
            name_input: String::new(),
            description_input: String::new(),
            currently_editing: None,
            selectable_rows: Vec::new(),
            db: Db::new(path),
        }
    }

    pub fn next_row(&mut self) {
        if self.selectable_rows.is_empty() {
            return;
        }

        let current = self.state.selected().unwrap_or(0);
        let mut next = current;

        loop {
            next = (next + 1) % self.selectable_rows.len();
            if self.selectable_rows[next] || next == current {
                self.state.select(Some(next));
                break;
            }
        }
        self.state.select(Some(next));
    }

    pub fn previous_row(&mut self) {
        if self.selectable_rows.is_empty() {
            return;
        }

        let current = self.state.selected().unwrap_or(0);
        let mut prev = current;
        loop {
            prev = (prev + self.selectable_rows.len() - 1) % self.selectable_rows.len();
            if self.selectable_rows[prev] || prev == current {
                self.state.select(Some(prev));
                break;
            }
        }

        self.state.select(Some(prev));
    }

    pub fn add_timer(&mut self) {
        let counter = self.db.get_count_of_timers().expect("TODO: panic message");
        let id = counter + 1;
        let timer = Timer::new(
            self.name_input.clone(),
            self.description_input.clone(),
            id as usize,
        );
        match self.timers.last_mut() {
            Some(t) => t.stop(),
            None => (),
        }
        self.db
            .add_timer_to_db(&timer)
            .expect("TODO: panic message");
        self.timers.push(timer);
        self.name_input = String::new();
        self.description_input = String::new();
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Name => {
                    self.currently_editing = Some(CurrentlyEditing::Description)
                }
                CurrentlyEditing::Description => {
                    self.currently_editing = Some(CurrentlyEditing::Name)
                }
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Name);
        }
    }
}

impl Timer {
    pub fn new(name: String, description: String, id: usize) -> Timer {
        Timer {
            start_time: Utc::now(),
            duration: Duration::zero(),
            name,
            description,
            id,
            running: true,
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self) {
        self.running = true;
    }
    pub fn tick(&mut self) {
        if self.running {
            self.duration += Duration::seconds(1);
        }
    }

    pub fn formatted_duration(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}",
            self.duration.num_hours(),
            self.duration.num_minutes() % 60,
            self.duration.num_seconds() % 60
        )
    }

    pub fn formatted_date(&self) -> String {
        self.start_time.format("%d-%m-%Y").to_string()
    }

    // For Debugging
    pub fn sub_day(&mut self, days: isize) {
        self.start_time = self.start_time - Duration::days(days as i64);
    }
}
