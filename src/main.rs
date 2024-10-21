use yew::prelude::*;
use yew_router::prelude::*;
mod user;
use user::User;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/menu1")]
    Menu1,
    #[at("/menu2")]
    Menu2,
    #[at("/menu3")]
    Menu3,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <User /> },
        Route::Menu1 => html! {
            <div class="bg-white shadow-md rounded-lg p-6">
                <h2 class="text-2xl font-semibold text-gray-800">{"菜单项 1 内容"}</h2>
            </div>
        },
        Route::Menu2 => html! {
            <div class="bg-white shadow-md rounded-lg p-6">
                <h2 class="text-2xl font-semibold text-gray-800">{"菜单项 2 内容"}</h2>
            </div>
        },
        Route::Menu3 => html! {
            <div class="bg-white shadow-md rounded-lg p-6">
                <h2 class="text-2xl font-semibold text-gray-800">{"菜单项 3 内容"}</h2>
            </div>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    let current_route = use_route::<Route>().unwrap_or(Route::Home);

    let nav_link = |route: Route, icon: &str, label: &str| {
        let is_active = current_route == route;
        let classes = if is_active {
            "block px-6 py-3 text-blue-600 bg-blue-100 font-semibold transition-colors duration-200"
        } else {
            "block px-6 py-3 text-gray-600 hover:bg-gray-100 transition-colors duration-200"
        };

        html! {
            <Link<Route> classes={classes} to={route}>
                <span class="inline-block w-6 mr-2">{icon}</span>
                {label}
            </Link<Route>>
        }
    };

    html! {
        <BrowserRouter>
            <div class="flex h-screen bg-gray-100">
                <aside class="w-64 bg-white shadow-sm">
                    <div class="p-6">
                        <h1 class="text-2xl font-semibold text-gray-800">{"Dashboard"}</h1>
                    </div>
                    <nav class="mt-6">
                        {nav_link(Route::Home, "🏠", "首页")}
                        {nav_link(Route::Menu1, "📊", "菜单项 1")}
                        {nav_link(Route::Menu2, "📁", "菜单项 2")}
                        {nav_link(Route::Menu3, "⚙️", "菜单项 3")}
                    </nav>
                </aside>

                <div class="flex-1 flex flex-col overflow-hidden">
                    <header class="flex justify-end items-center p-4 bg-white shadow-sm">
                        <div class="flex items-center">
                            <img class="w-8 h-8 rounded-full mr-2 object-cover" src="https://via.placeholder.com/32" alt="User avatar" />
                            <span class="text-gray-700">{"ilovek8s"}</span>
                        </div>
                    </header>

                    <main class="flex-1 overflow-x-hidden overflow-y-auto bg-gray-100 p-6">
                        <Switch<Route> render={switch} />
                    </main>
                </div>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
