use input_rs::leptos::Input;
use leptos::{prelude::*, task::spawn_local};
use regex::Regex;
use serde::{Deserialize, Serialize};
use leptos::logging::log;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    email: String,
    password: String,
}

fn validate_email(email: String) -> bool {
    let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
    pattern.is_match(&email)
}

fn validate_password(password: String) -> bool {
    !&password.is_empty()
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <LoginForm />
    }
}

#[component]
fn LoginForm() -> impl IntoView {
    let error_handle = signal(String::default());
    let error = error_handle.0.get();

    let email_valid_handle = signal(true);
    let email_valid = email_valid_handle.0.get();

    let password_valid_handle = signal(true);
    let password_valid = password_valid_handle.0.get();

    let email_handle = signal(String::default());
    let email = email_handle.0.get();

    let password_handle = signal(String::default());
    let password = password_handle.0.get();

    let onsubmit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let email_ref = email.clone();
        let password_ref = password.clone();
        let error_handle = error_handle.clone();

        spawn_local(async move {
            if email_valid && password_valid {
                // API call
                log!(
                    "Logged in with Email: {}, Password: {}",
                    email_ref,
                    password_ref
                );
            } else {
                error_handle
                    .1
                    .set("Please provide a valid email and password!".to_string());
            }
        });
    };

    view! {
        <div class="text-black min-h-screen bg-gradient-to-tr from-indigo-500 to-pink-500 flex items-center justify-center"
            style="border: none; width: 100vw; height: 100vh; overflow: hidden; position: fixed; top: 0; left: 0;"
            role="main" aria-label="Sign In Form">
            // TODO: Why the flex styling is not applied?
            // <div class="w-96 bg-white rounded-lg shadow-lg p-8">
            //     <h2 class="text-4xl text-gray-800 pb-6">{"Sign In"}</h2>
            //     { move || if !error.is_empty() {
            //             Some(view! {<div class="mb-3 error bg-red-600 text-white px-4 py-3 font-semibold rounded-md text-center text-base">error</div>})
            //         }
            //         else {None}
            //     }
            // </div>
            <form on:submit={onsubmit}>
                <Input
                    r#type="text"
                    handle={email_handle}
                    name="email"
                    label="Email"
                    placeholder="Your Email"
                    icon_class="text-2xl fa fa-person text-gray-500 absolute left-3 top-1/2 transform -translate-y-1/2"
                    field_class="relative"
                    input_class="input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 pl-12"
                    error_class="text-red-500 absolute text-sm mb-2"
                    required=true
                    valid_handle={email_valid_handle}
                    validate_function={validate_email}
                    error_message="Enter a valid email address"
                />
                <Input
                    r#type="password"
                    handle={password_handle}
                    name="password"
                    label="Password"
                    placeholder="Password"
                    icon_class="text-2xl fa fa-lock text-gray-500 absolute left-3 top-1/2 transform -translate-y-1/2"
                    field_class="relative mt-2 mb-2"
                    input_class="input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 pl-12"
                    error_class="text-red-500 absolute text-sm"
                    required=true
                    valid_handle={password_valid_handle}
                    validate_function={validate_password}
                    error_message="Password can't be blank!"
                    eye_active="cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-2xl text-gray-600 toggle-button fa fa-eye"
                    eye_disabled="cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-2xl text-gray-600 toggle-button fa fa-eye-slash"
                />
                <div class="w-full">
                    <button type="submit" class="btn-social bg-indigo-600 hover:bg-indigo-700 text-white w-full py-3 rounded-lg text-base font-medium tracking-wide">
                        <i class="fa fa-box-arrow-in-right me-2"></i>
                        {"Sign In"}
                    </button>
                </div>
                <div class="w-full text-center">
                    <span class="text-base text-gray-800">{"Not a member?"}</span>
                    <a href="#" class="text-base text-gray-800">{"Sign Up Now"}</a>
                </div>
                <div class="mt-8">
                    <div class="relative text-center text-gray-800 mt-4">
                        <span class="px-4 bg-white relative z-10">{"Or Sign In With"}</span>
                        <div class="absolute inset-0 flex items-center">
                            <div class="w-full h-px bg-gray-400"></div>
                        </div>
                    </div>
                    <div class="flex justify-center space-x-4 mt-12">
                        <button class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12" aria-label="Sign in with Facebook">
                            <i class="fa fa-facebook"></i>
                        </button>
                        <button class="btn-social bg-red-600 hover:bg-red-700 text-white w-12 h-12 rounded-lg text-xl leading-12" aria-label="Sign in with Google">
                            <i class="fa fa-google"></i>
                        </button>
                        <button class="btn-social bg-blue-400 hover:bg-blue-500 text-white w-12 h-12 rounded-lg text-xl leading-12" aria-label="Sign in with Twitter">
                            <i class="fa fa-twitter"></i>
                        </button>
                        <button class="btn-social bg-blue-900 hover:bg-blue-800 text-white w-12 h-12 rounded-lg text-xl leading-12" aria-label="Sign in with LinkedIn">
                            <i class="fa fa-linkedin"></i>
                        </button>
                    </div>
                </div>
            </form>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount::mount_to_body(|| view! { <App/> })
}
