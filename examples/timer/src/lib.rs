#[macro_use]
extern crate yew;

use std::time::Duration;
use yew::prelude::*;
use yew::services::Task;
use yew::services::timeout::TimeoutService;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::console::ConsoleService;

pub struct Model {
    callback_tick: Callback<()>,
    callback_done: Callback<()>,
    job: Option<Box<Task>>,
    messages: Vec<&'static str>,
    _standalone: IntervalTask,
}

pub enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<IntervalService> + AsMut<TimeoutService> + AsMut<ConsoleService> + 'static,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<CTX, Self>, ctx: &mut CTX) -> Self {
        // This callback doesn't send any message to a scope
        let callback = |_| {
            println!("Example of a standalone callback.");
        };
        let interval: &mut IntervalService = ctx.as_mut();
        let handle = interval.spawn(Duration::from_secs(10), callback.into());

        Model {
            callback_tick: link.send_back(|_| Msg::Tick),
            callback_done: link.send_back(|_| Msg::Done),
            job: None,
            messages: Vec::new(),
            _standalone: handle,
        }
    }

    fn update(&mut self, msg: Self::Message, ctx: &mut CTX) -> ShouldRender {
        match msg {
            Msg::StartTimeout => {
                {
                    let timeout: &mut TimeoutService = ctx.as_mut();
                    let handle = timeout.spawn(Duration::from_secs(3), self.callback_done.clone());
                    self.job = Some(Box::new(handle));
                }
                let console: &mut ConsoleService = ctx.as_mut();
                self.messages.clear();
                console.clear();
                self.messages.push("Timer started!");
                console.time_named("Timer");
            }
            Msg::StartInterval => {
                {
                    let interval: &mut IntervalService = ctx.as_mut();
                    let handle = interval.spawn(Duration::from_secs(1), self.callback_tick.clone());
                    self.job = Some(Box::new(handle));
                }
                let console: &mut ConsoleService = ctx.as_mut();
                self.messages.clear();
                console.clear();
                self.messages.push("Interval started!");
                console.log("Interval started!");
            }
            Msg::Cancel => {
                if let Some(mut task) = self.job.take() {
                    task.cancel();
                }
                self.messages.push("Canceled!");
                let console: &mut ConsoleService = ctx.as_mut();
                console.warn("Canceled!");
                console.assert(self.job.is_none(), "Job still exists!");
            }
            Msg::Done => {
                self.messages.push("Done!");
                let console: &mut ConsoleService = ctx.as_mut();
                console.group();
                console.info("Done!");
                console.time_named_end("Timer");
                console.group_end();
                self.job = None;
            }
            Msg::Tick => {
                self.messages.push("Tick...");
                let console: &mut ConsoleService = ctx.as_mut();
                console.count_named("Tick");
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<IntervalService> + AsMut<TimeoutService> + AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        let view_message = |message| {
            html! { <p>{ message }</p> }
        };
        let has_job = self.job.is_some();
        html! {
            <div>
                <button disabled= has_job, onclick=|_| Msg::StartTimeout,>{ "Start Timeout" }</button>
                <button disabled= has_job, onclick=|_| Msg::StartInterval,>{ "Start Interval" }</button>
                <button disabled=!has_job, onclick=|_| Msg::Cancel,>{ "Cancel!" }</button>
                <div>
                    { for self.messages.iter().map(view_message) }
                </div>
            </div>
        }
    }
}
