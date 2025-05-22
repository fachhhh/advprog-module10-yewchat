use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::{services::{event_bus::EventBus, websocket::WebsocketService}, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("Context to be set");

        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        log::debug!("Create function");

        if let Ok(_) = wss.tx.clone().try_send(serde_json::to_string(&message).unwrap()) {
            log::debug!("Message sent successfully!");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://api.dicebear.com/8.x/adventurer-neutral/svg?seed={}",
                                    u
                                )
                                    .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData = serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self.wss.tx.clone().try_send(serde_json::to_string(&message).unwrap()) {
                        log::debug!("Error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let (cur_user, _) = ctx.link().context::<User>(Callback::noop()).expect("Context to be set");
        let cur_username = cur_user.username.borrow().clone();

        html! {
            <div class="fixed inset-0 w-full h-full bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 flex relative overflow-hidden">
                // Background decorative elements
                <div class="absolute inset-0 bg-black opacity-30"></div>
                <div class="absolute top-20 left-20 w-72 h-72 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl opacity-10 animate-pulse"></div>
                <div class="absolute top-40 right-32 w-80 h-80 bg-blue-500 rounded-full mix-blend-multiply filter blur-xl opacity-10 animate-pulse animation-delay-2000"></div>
                <div class="absolute -bottom-8 left-40 w-96 h-96 bg-indigo-500 rounded-full mix-blend-multiply filter blur-xl opacity-10 animate-pulse animation-delay-4000"></div>

                // Users Sidebar
                <div class="relative z-10 flex-none w-80 h-full bg-white/5 backdrop-blur-lg border-r border-white/10">
                    <div class="p-6 border-b border-white/10">
                        <div class="flex items-center space-x-3">
                            <div class="w-12 h-12 bg-gradient-to-r from-purple-500 to-blue-500 rounded-full flex items-center justify-center shadow-lg">
                                <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"></path>
                                </svg>
                            </div>
                            <div>
                                <h2 class="text-xl font-bold text-white">{"Online Users"}</h2>
                                <p class="text-sm text-gray-300">{format!("{} users online", self.users.len())}</p>
                            </div>
                        </div>
                    </div>

                    <div class="p-4 space-y-3 overflow-y-auto h-full">
                        {
                            self.users.clone().iter().map(|u| {
                                let is_current_user = u.name == cur_username;
                                let bg_color = if is_current_user { "bg-green-500/20 border-green-400/50" } else { "bg-white/5 hover:bg-white/10 border-white/5 hover:border-white/20" };
                                let text_color = if is_current_user { "text-green-300" } else { "text-white group-hover:text-purple-300" };

                                html!{
                                    <div class={format!("group flex items-center space-x-3 {} rounded-xl p-3 transition-all duration-300 cursor-pointer border", bg_color)}>
                                        <div class="relative">
                                            <img class="w-12 h-12 rounded-full ring-2 ring-purple-500/50 shadow-lg" src={u.avatar.clone()} alt="avatar"/>
                                            <div class="absolute bottom-0 right-0 w-4 h-4 bg-green-500 rounded-full border-2 border-white"></div>
                                        </div>
                                        <div class="flex-grow">
                                            <div class={format!("font-medium transition-colors {}", text_color)}>
                                                {u.name.clone()}
                                                if is_current_user {
                                                    {" (You)"}
                                                }
                                            </div>
                                            <div class="text-xs text-gray-400">
                                                {"Active now"}
                                            </div>
                                        </div>
                                        <div class="w-2 h-2 bg-purple-500 rounded-full opacity-0 group-hover:opacity-100 transition-opacity"></div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                // Chat Area
                <div class="relative z-10 flex-grow h-full flex flex-col bg-white/5 backdrop-blur-lg">
                    // Chat Header
                    <div class="flex items-center justify-between p-6 border-b border-white/10 bg-white/5">
                        <div class="flex items-center space-x-4">
                            <div class="w-10 h-10 bg-gradient-to-r from-purple-500 to-blue-500 rounded-full flex items-center justify-center shadow-lg">
                                <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.862 9.862 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                                </svg>
                            </div>
                            <div>
                                <h1 class="text-2xl font-bold text-white">{"Global Chat"}</h1>
                                <p class="text-sm text-gray-300">{"Connect with everyone"}</p>
                            </div>
                        </div>
                        <div class="flex items-center space-x-2">
                            <div class="w-3 h-3 bg-green-500 rounded-full animate-pulse"></div>
                            <span class="text-sm text-gray-300">{"Connected"}</span>
                        </div>
                    </div>

                    // Messages Area
                    <div class="flex-grow overflow-y-auto p-6 space-y-4">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                let is_current_user = user.name == cur_username;
                                let message_container_class = if is_current_user {
                                    "flex justify-end" // Ini akan mendorong pesan ke kanan
                                } else {
                                    "flex justify-start" // Ini akan memposisikan pesan di kiri
                                };
                                let bubble_color = if is_current_user {
                                    "bg-green-500/20 border-green-400/30"
                                } else {
                                    "bg-white/10 border-white/10"
                                };
                                let bubble_rounding = if is_current_user {
                                    "rounded-2xl rounded-tr-none"
                                } else {
                                    "rounded-2xl rounded-tl-none"
                                };
                                let name_color = if is_current_user {
                                    "text-green-300"
                                } else {
                                    "text-purple-300"
                                };

                                html! {
                                    <div class={message_container_class}>
                                        <div class={format!("flex items-start space-x-3 max-w-4xl")}>
                                            {if !is_current_user {
                                                html! {
                                                    <img class="w-10 h-10 rounded-full ring-2 ring-purple-500/30 shadow-lg flex-shrink-0"
                                                        src={user.avatar.clone()} alt="avatar"/>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                            <div class={format!("flex flex-col {}", if is_current_user { "items-end" } else { "items-start" })}>
                                                <div class={format!("backdrop-blur-sm p-4 border shadow-lg {} {}", bubble_color, bubble_rounding)}>
                                                    <div class="flex items-center space-x-2 mb-2">
                                                        <span class={format!("text-sm font-semibold {}", name_color)}>
                                                            {m.from.clone()}
                                                            if is_current_user {
                                                                {" (You)"}
                                                            }
                                                        </span>
                                                        <span class="text-xs text-gray-400">
                                                            {"just now"}
                                                        </span>
                                                    </div>
                                                    <div class="text-white">
                                                        if m.message.ends_with(".gif") {
                                                            <img class="mt-2 rounded-lg max-w-xs shadow-lg" src={m.message.clone()}/>
                                                        } else {
                                                            <p class="break-words">{m.message.clone()}</p>
                                                        }
                                                    </div>
                                                </div>
                                            </div>
                                            {if is_current_user {
                                                html! {
                                                    <img class="w-10 h-10 rounded-full ring-2 ring-purple-500/30 shadow-lg flex-shrink-0"
                                                        src={user.avatar.clone()} alt="avatar"/>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    // Message Input
                    <div class="p-6 border-t border-white/10 bg-white/5">
                        <div class="flex items-center space-x-4">
                            <div class="flex-grow relative">
                                <input
                                    ref={self.chat_input.clone()}
                                    type="text"
                                    placeholder="Type your message..."
                                    class="w-full py-4 pl-6 pr-16 bg-white/10 border border-white/20 rounded-2xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent backdrop-blur-sm transition-all duration-300 hover:bg-white/20"
                                    name="message"
                                    required=true
                                />
                                <div class="absolute inset-y-0 right-0 flex items-center pr-4">
                                    <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4V2a1 1 0 011-1h8a1 1 0 011 1v2M7 4h10M7 4l-4 4v10a2 2 0 002 2h14a2 2 0 002-2V8l-4-4"></path>
                                    </svg>
                                </div>
                            </div>
                            <button
                                onclick={submit}
                                class="p-4 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 rounded-2xl shadow-lg hover:shadow-xl transform hover:scale-105 transition-all duration-300 flex items-center justify-center group"
                            >
                                <svg class="w-6 h-6 text-white group-hover:rotate-45 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>

                // Floating decorative elements
                <div class="absolute top-1/4 right-8 w-4 h-4 bg-purple-500 rounded-full opacity-60 animate-ping"></div>
                <div class="absolute top-3/4 left-8 w-3 h-3 bg-blue-500 rounded-full opacity-60 animate-ping animation-delay-1000"></div>
                <div class="absolute bottom-1/4 right-1/4 w-2 h-2 bg-indigo-500 rounded-full opacity-60 animate-bounce"></div>
            </div>
        }
    }
}