pub fn time_tracking_app() {
    let mut store = Store::load();
    let mut timer = Timer::load();
    let Some(command) = std::env::args().nth(1) else {
        println!("provide a subcommand (history, start, end, note)");
        return;
    };
    match command.as_str() {
        "history" => println!("{}", store),
        "start" => match timer.start() {
            Ok(_) => println!("timer started"),
            Err(e) => println!("failed to start timer: {e}"),
        },
        "end" => match timer.end() {
            Ok(time) => {
                store.add_entry(time);
                println!("entry added")
            }
            Err(e) => println!("failed to end timer: {e}"),
        },
        "note" => match timer.add_note(&std::env::args().nth(2).unwrap_or_default()) {
            Ok(_) => println!("added note"),
            Err(e) => println!("couldn't add note: {e}"),
        },
        _ => println!("unknown command, valid commands are history, start, end, note"),
    }
    store.save();
    timer.save();
}

autofill::autofill! {
    pub struct Timer {
        state: TimerState,
        notes: Vec<String>,
    }

    pub enum TimerState {
        Started(SystemTime),
        Idle,
    }

    impl Timer {
        fn load() -> Self {
            todo!("load from ./timer.csv")
        }
        fn save(self) {
            todo!("save to ./timer.csv")
        }
        fn start(&mut self) -> Result<(), String> {
            todo!("if not running, start it, else return an error")
        }

        fn end(&mut self) -> Result<TimeEntry, String> {
            todo!("if running, and and return time entry, else return descriptive error");
        }

        fn add_note(&mut self, note: &str) -> Result<(), String> {
            todo!("add a note if timer is running")
        }
    }

    pub struct Store {
        times: Vec<TimeEntry>,
    }

    pub struct TimeEntry {
        start: SystemTime,
        end: SystemTime,
        notes: Vec<String>,
    }

    impl Store {
        fn load() -> Self {
            todo!("load a store from the csv file at the path ./timestore.csv")
        }

        fn add_entry(&mut self, entry: TimeEntry) { todo!() }

        fn save(self) {
            todo!("save store to ./timestore.csv")
        }

    }

    impl Display for TimeEntry {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            todo!("pretty print single entry")
        }
    }

    impl Display for Store {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            todo!("pretty print history using Display for TimeEntry")
        }
    }
}
