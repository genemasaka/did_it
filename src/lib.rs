use serde_derive::Deserialize;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::html;
use yew::prelude::*;
use yew::services::{
	fetch::{FetchService, FetchTask, Request, Response},
	ConsoleService,
};
use yew_router::{components::RouterAnchor, router::Router, Switch};

mod todo;

#[derive(Debug)]
struct ToDoApp {
	link: ComponentLink<Self>,
	todos: Option<Vec<ToDo>>,
	fetch_task: Option<FetchTask>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToDo {
	pub user_id: u64,
	pub id: u64,
	pub title: String,
	pub completed: bool,
}

//uses MakeReq to make http request and Resp to receive the respone
enum Msg {
	MakeReq,
	Resp(Result<Vec<ToDo>, anyhow::Error>),
}

impl Component for ToDoApp {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		link.send_message(Msg::MakeReq);
		Self {
			link,
			todos: None,
			fetch_task: None,
		}
	}

	fn update(&self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::MakeReq => {
				self.todos = None;
				let req = Request::get("")
				    .body(Nothing)
				    .expect("can't make request to jsonplaceholder");
				let cb = self.link.callback(
					|respone: Response<Json<Result<Vec<ToDo>, anyhow::Error>>>| {
						let Json(data) = response.into_body();
						Msg::Resp(data)
					},
					);

				let task = FetchService::fetch(req, cb).expect("Can't create task");
				self.fetch_task =  Some(task);
				()
			}
			Msg::Resp(resp) => {
				if let Ok(data) = resp {
					self.todos = Some(data);
				}
			}
		}
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let todos = self.todos.clone();
		let cb = self.link.callback(|_| Msg::MakeReq);
		ConsoleService::info(&format!("render ToDoApp: {:?}", todos));
		html! {
			<div class={classes!("todo")}>
			<div>
				<div class={classes!("refresh")}>
					<button onclick=cb.clone()>
					{"refresh"}
					</button>
				</div>
				<todo::list::List todos=todos.clone()/>
			</div>
			</div>
		}
	}
}

#[wasm_bindgen(start)]
pub fn run_app() {
	App::<ToDoApp>::new().mount_to_body();
}