use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="w-screen h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 flex items-center justify-center p-4 overflow-hidden">
            <div class="absolute inset-0 bg-black opacity-50"></div>

            // Floating shapes for decoration
            <div class="absolute top-20 left-20 w-72 h-72 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-pulse"></div>
            <div class="absolute top-40 right-32 w-80 h-80 bg-blue-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-pulse animation-delay-2000"></div>
            <div class="absolute -bottom-8 left-40 w-96 h-96 bg-indigo-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-pulse animation-delay-4000"></div>

            <div class="relative z-10 w-full max-w-lg">
                <div class="bg-white/10 backdrop-blur-lg rounded-3xl p-8 shadow-2xl border border-white/20">
                    <div class="text-center mb-8">
                        <div class="w-20 h-20 bg-gradient-to-r from-purple-500 to-blue-500 rounded-full mx-auto mb-4 flex items-center justify-center shadow-lg">
                            <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.862 9.862 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                            </svg>
                        </div>
                        <h1 class="text-3xl font-bold text-white mb-2">{"Welcome Back"}</h1>
                        <p class="text-gray-300">{"Enter your username to start chatting"}</p>
                    </div>

                    <form class="space-y-6">
                        <div class="relative">
                            <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                                <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                </svg>
                            </div>
                            <input
                                {oninput}
                                class="w-full pl-12 pr-4 py-5 bg-white/10 border border-white/20 rounded-xl text-white placeholder-gray-400 text-center focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent backdrop-blur-sm transition-all duration-300 hover:bg-white/20 text-lg"
                                placeholder="Enter your username"
                                type="text"
                            />
                        </div>

                        <Link<Route> to={Route::Chat}>
                            <button
                                {onclick}
                                disabled={username.len() < 1}
                                class="mt-4 w-full py-3 px-4 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 disabled:from-gray-600 disabled:to-gray-700 disabled:opacity-50 disabled:cursor-not-allowed text-white font-semibold rounded-xl shadow-lg hover:shadow-xl transform hover:scale-105 disabled:hover:scale-100 transition-all duration-300 flex items-center justify-center space-x-2 text-base"
                            >
                                <span>{"Start Chatting"}</span>
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"></path>
                                </svg>
                            </button>
                        </Link<Route>>
                    </form>

                    <div class="mt-8 text-center">
                        <p class="text-sm text-gray-400">
                            {"Ready to connect with others? "}
                            <span class="text-purple-400 font-medium">{"Let's get started!"}</span>
                        </p>
                    </div>
                </div>

                // Floating elements around the form
                <div class="absolute -top-4 -right-4 w-8 h-8 bg-purple-500 rounded-full opacity-60 animate-bounce"></div>
                <div class="absolute -bottom-2 -left-2 w-6 h-6 bg-blue-500 rounded-full opacity-60 animate-bounce animation-delay-1000"></div>
                <div class="absolute top-1/2 -left-8 w-4 h-4 bg-indigo-500 rounded-full opacity-60 animate-ping"></div>
                <div class="absolute top-1/4 -right-6 w-3 h-3 bg-pink-500 rounded-full opacity-60 animate-ping animation-delay-2000"></div>
            </div>
        </div>
    }
}