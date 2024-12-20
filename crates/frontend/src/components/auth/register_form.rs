use types::user::RegisterUser;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast, UseStateHandle};
use yew_hooks::use_async;
use gloo_console::error;
use yew_router::history::{History, BrowserHistory};
use yewdux::prelude::*;

use crate::{components::{buttons::button::Button, error_message::ErrorMessage, input::Input}, hooks::StoredUserInfo, services::{self, AuthError}};

#[function_component(RegisterForm)]
pub fn register_form() -> Html {
    let (_user_state, user_dispatch) = use_store::<StoredUserInfo>();
    let error_state = use_state(|| None::<AuthError>);
    let register_user = use_state(RegisterUser::default);

    let oninput = |key, error_state: &UseStateHandle<Option<AuthError>>| {
        let error_state = error_state.clone();
        let register_user = register_user.clone();
        Callback::from(move |e: InputEvent| {
            let error_state = error_state.clone();
            if let Some(_) = *error_state {
                error_state.set(None);
            }
            let input: HtmlInputElement = e.target_unchecked_into();
            match register_user.update_field(key, input.value()) {
                Ok(new_register_user) => {
                    register_user.set(new_register_user);
                }, Err(error) => {error!(error)}
            };
        })
    };

    let handle_register = {
        let register_user = register_user.clone();
        let error_state = error_state.clone();
        use_async(async move {
            let response = services::auth::register_user((*register_user).clone()).await;
            match response {
                Ok(user_info) => {
                    user_dispatch.set(StoredUserInfo {user_info: user_info.clone()});
                    register_user.set(RegisterUser::default());
                    BrowserHistory::new().push("/login");
                    Ok(user_info)
                },
                Err(error) => {
                    error_state.set(Some(error.to_owned()));
                    Err(error)
                }
            }
        })
    };

    let register_onclick = {
        let handle_register = handle_register.clone();
        Callback::from(move |_| {
            handle_register.run();
        })
    };

    let register_onsubmit = {
        let handle_register = handle_register.clone();
        Callback::from(move |ev: SubmitEvent| {
            ev.prevent_default();
            handle_register.run();
        })
    };

    html! {
        <form class="flex flex-col w-64 space-y-2" onsubmit={register_onsubmit}>
            if let Some(error) = (*error_state).to_owned() {
                <ErrorMessage message={error.body().message} />
            }
            <Input input_type="text" placeholder="Username" oninput={oninput("username", &error_state)} value={register_user.username.to_owned()} />
            <Input input_type="password" placeholder="Password" oninput={oninput("pass", &error_state)} value={register_user.pass.to_owned()} />
            <Input input_type="email" placeholder="Email" oninput={oninput("email", &error_state)} value={register_user.email.to_owned()} />
            <Button onclick={register_onclick} label="Register" />
        </form>
    }
}