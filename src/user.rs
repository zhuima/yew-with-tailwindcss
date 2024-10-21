use yew::prelude::*;

// 定义用户结构体
pub struct User {
    name: String,
}

// 定义消息类型
pub enum Msg {
    UpdateName,
}

impl Component for User {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "ilovek8s".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateName => {
                self.name = "ilovelife".to_string();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
           <div class="bg-white shadow-md rounded-lg p-6">
                <h2 class="text-2xl font-semibold text-gray-800 mb-4">{"用户信息"}</h2>
                <div class="flex items-center mb-4">
                    <img class="w-16 h-16 rounded-full mr-4" src="https://via.placeholder.com/64" alt="User avatar" />
                    <div>
                        <h3 class="text-xl font-semibold text-gray-800">{self.name.clone()}</h3>
                        <p class="text-gray-600">{"用户角色：管理员"}</p>
                    </div>
                </div>
                <button
                    onclick={ctx.link().callback(|_| Msg::UpdateName)}
                    class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition duration-300"
                >
                    {"更新用户名"}
                </button>
            </div>
        }
    }
}
