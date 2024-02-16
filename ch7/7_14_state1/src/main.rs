trait State {
    fn on_state_changed(self: Box<Self>) -> Box<dyn State>;
}

struct Start;
impl State for Start {
    fn on_state_changed(self: Box<Start>) -> Box<dyn State> {
        println!("현재 상태는 [Start]. 다음 상태는 [Running]");
        Box::new(Running {})
    }
}

struct Running;
impl State for Running {
    fn on_state_changed(self: Box<Running>) -> Box<dyn State> {
        println!("현재 상태는 [Running]. 다음 상태는 [Stop]");
        Box::new(Stop {})
    }
}

struct Stop;
impl State for Stop {
    fn on_state_changed(self: Box<Stop>) -> Box<dyn State> {
        println!("현재 상태는 [Stop]. 다음 상태는 [없음]");
        self
    }
}

struct Boot {
    state: Option<Box<dyn State>>,
}

impl Boot {
    fn new() -> Boot {
        Boot {
            state: Some(Box::new(Stop {})),
        }
    }

    fn boot(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(Box::new(Start {}))
        }
    }

    fn next(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.on_state_changed())
        }
    }
}

fn main() {
    let mut post = Boot::new();
    post.boot();
    post.next();
    post.next();
    post.next();
}