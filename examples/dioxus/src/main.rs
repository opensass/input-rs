use dioxus::prelude::*;
use dioxus_logger::tracing;
use input_rs::dioxus::Input;
use regex::Regex;

pub fn validate_email(email: String) -> bool {
    let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
    pattern.is_match(&email)
}

pub fn validate_input(field: String) -> bool {
    !&field.is_empty()
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Script { src: "https://kit.fontawesome.com/8f223ead6e.js" },
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" },
        MultiStepFormOne {}
    }
}

#[allow(non_snake_case)]
pub fn MultiStepFormOne() -> Element {
    let mut error = use_signal(String::new);
    let email_valid = use_signal(|| true);
    let full_name_valid = use_signal(|| true);
    let phone_number_valid = use_signal(|| true);
    let address_valid = use_signal(|| true);
    let birthday_valid = use_signal(|| true);
    let mut gender_valid = use_signal(|| true);
    let username_valid = use_signal(|| true);
    let password_valid = use_signal(|| true);

    let input_email = use_signal(String::new);
    let input_full_name = use_signal(String::new);
    let input_phone_number = use_signal(String::new);
    let input_address = use_signal(String::new);
    let input_birthday = use_signal(String::new);
    let mut input_gender = use_signal(String::new);
    let input_username = use_signal(String::new);
    let input_password = use_signal(String::new);

    let mut current_step = use_signal(|| 0);

    let on_next = {
        move |_| match current_step() {
            0 => {
                if full_name_valid() && email_valid() {
                    current_step.set(current_step() + 1);
                    error.set(String::new());
                } else {
                    error.set("Please provide a valid full name and email address!".to_string());
                }
            }
            1 => {
                if phone_number_valid() && address_valid() {
                    current_step.set(current_step() + 1);
                    error.set(String::new());
                } else {
                    error.set("Please provide a valid phone number and address!".to_string());
                }
            }
            2 => {
                if birthday_valid() && gender_valid() {
                    current_step.set(current_step() + 1);
                    error.set(String::new());
                } else {
                    error.set("Please provide a valid birth date and gender!".to_string());
                }
            }
            _ => (),
        }
    };

    let on_previous = {
        move |_| {
            if current_step() > 0 {
                current_step.set(current_step() - 1);
            }
        }
    };

    let on_gender_change = {
        move |e: Event<FormData>| {
            let value = e.value();
            input_gender.set(value.clone());
            gender_valid.set(validate_input(value));
        }
    };

    let render_progress_item = |index: usize| {
        rsx! {
            div {
                class: "flex items-center text-left justify-center my-10",
                for step in 0..4 {
                    div {
                        class: "step",
                        p {
                            class: "font-semibold mb-2 text-pink-800",
                            "Step {step + 1}"
                        }
                        div {
                            class: "flex items-center",
                            div {
                                class: "outer-check bullet border-2 border-pink-800 rounded-full h-7 w-7 flex items-center justify-center",
                                span {
                                    class: "step-index font-semibold text-pink-800",
                                    "{step + 1}"
                                }
                                if index > step {
                                    span {
                                        class: "check bg-pink-800 z-50"
                                    }
                                }
                            }
                            if step < 3 {
                                span {
                                    class: if index > step { "line bg-pink-800 h-1 w-10" } else { "line bg-grey-400 h-1 w-10" }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    let current_step_content = match current_step() {
        0 => rsx! {
            div {
                class: "page ml-0 transition-transform duration-300",
                div {
                    class: "title text-left text-xl font-semibold mb-4",
                    "Personal Information"
                }
                Input {
                    r#type: "text",
                    label: "Full Name",
                    handle: input_full_name,
                    placeholder: "Full Name",
                    error_message: "Full name can't be blank!",
                    required: true,
                    valid_handle: full_name_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "label font-semibold text-pink-800",
                    input_class: "w-full border border-pink-800 rounded px-4 py-2",
                    error_class: "text-red-500 text-sm my-2",
                }
                Input {
                    r#type: "text",
                    label: "Email",
                    handle: input_email,
                    placeholder: "Email",
                    error_message: "Enter a valid email address!",
                    required: true,
                    valid_handle: email_valid,
                    validate_function: validate_email,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "label font-semibold text-pink-800",
                    input_class: "w-full border border-pink-800 rounded px-4 py-2",
                    error_class: "text-red-500 text-sm my-2",
                }
                button {
                    class: "next bg-purple-800 hover:bg-purple-700 text-white rounded px-4 mt-10 py-2 w-full font-semibold mb-4",
                    onclick: on_next,
                    "Next"
                }
            }
        },
        1 => rsx! {
            div {
                class: "page transition-transform duration-300",
                div {
                    class: "title text-left text-xl font-semibold mb-4",
                    "Contact Details"
                }
                Input {
                    r#type: "tel",
                    label: "Phone Number",
                    handle: input_phone_number,
                    placeholder: "+19999",
                    error_message: "Phone number can't be blank!",
                    required: true,
                    valid_handle: phone_number_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "flex telephone-input validate-input mb-6",
                    label_class: "label font-semibold text-pink-800",
                    input_class: "w-full border border-pink-800 rounded px-4 py-2",
                    error_class: "text-red-500 text-sm my-2",
                }
                Input {
                    r#type: "text",
                    label: "Address",
                    handle: input_address,
                    placeholder: "Address",
                    error_message: "Address can't be blank!",
                    required: true,
                    valid_handle: address_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "label font-semibold text-pink-800",
                    input_class: "w-full border border-pink-800 rounded px-4 py-2",
                    error_class: "text-red-500 text-sm my-2",
                }
                div {
                    class: "justify-center flex field btns text-center space-x-5 mt-10",
                    button {
                        class: "prev bg-pink-800 hover:bg-pink-700 text-white rounded px-4 py-2 font-semibold",
                        onclick: on_previous,
                        "Previous"
                    }
                    button {
                        class: "next bg-purple-800 hover:bg-purple-700 text-white rounded px-4 py-2 font-semibold",
                        onclick: on_next,
                        "Next"
                    }
                }
            }
        },
        2 => rsx! {
            div {
                class: "page transition-transform duration-300",
                div {
                    class: "title text-left text-xl font-semibold mb-4",
                    "Date of Birth"
                }
                Input {
                    r#type: "date",
                    label: "Date of Birth",
                    handle: input_birthday,
                    placeholder: "Birthday",
                    error_message: "Birthday can't be blank!",
                    required: true,
                    valid_handle: birthday_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "label font-semibold text-pink-800",
                    input_class: "w-full border border-pink-800 rounded px-4 py-2",
                    error_class: "text-red-500 text-sm my-2",
                }
                div {
                    class: "field mb-6",
                    div {
                        class: "label font-semibold text-pink-800",
                        "Gender"
                    }
                    select {
                        class: "w-full border border-pink-800 rounded px-4 py-2",
                        id: "gender",
                        required: true,
                        aria_placeholder: "Gender",
                        oninput: on_gender_change,
                        option { "Male" }
                        option { "Female" }
                        option { "Non-binary" }
                        option { "Other" }
                    }
                }
                if !gender_valid() {
                    div {
                        class: "error-txt text-red-500 text-sm my-2",
                        "Gender can't be blank!"
                    }
                }
                div {
                    class: "justify-center flex field btns text-center space-x-5 mt-10",
                    button {
                        class: "prev bg-pink-800 hover:bg-pink-700 text-white rounded px-4 py-2 font-semibold",
                        onclick: on_previous,
                        "Previous"
                    }
                    button {
                        class: "next bg-purple-800 hover:bg-purple-700 text-white rounded px-4 py-2 font-semibold",
                        onclick: on_next,
                        "Next"
                    }
                }
            }
        },
        3 => rsx! {
            div {
                class: "page transition-transform duration-300",
                div {
                    class: "title text-left text-xl font-semibold mb-4",
                    "Account Details"
                }
                Input {
                    r#type: "text",
                    label: "Username",
                    handle: input_username,
                    placeholder: "Username",
                    error_message: "Username can't be blank!",
                    required: true,
                    valid_handle: username_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "label font-semibold text-pink-800",
                    input_class: "w-full border border-pink-800 rounded px-4 py-2",
                    error_class: "text-red-500 text-sm my-2",
                }
                Input {
                    r#type: "password",
                    label: "Password",
                    handle: input_password,
                    placeholder: "Password",
                    error_message: "Password can't be blank!",
                    required: true,
                    valid_handle: password_valid,
                    validate_function: validate_input,
                    field_class: "relative mt-2 mb-2",
                    input_class: "input w-full px-4 py-2 rounded border border-pink-800 bg-gray-100",
                    error_class: "text-red-500 absolute text-sm",
                    eye_active: "cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-2xl text-gray-600 toggle-button fa fa-eye",
                    eye_disabled: "cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-2xl text-gray-600 toggle-button fa fa-eye-slash",
                }
                div {
                    class: "justify-center flex field btns text-center space-x-5 mt-10",
                    button {
                        class: "prev bg-pink-800 hover:bg-pink-700 text-white rounded px-4 py-2 font-semibold",
                        onclick: on_previous,
                        "Previous"
                    }
                    button {
                        class: "submit bg-purple-800 hover:bg-purple-700 text-white rounded px-4 py-2 font-semibold",
                        "Submit"
                    }
                }
            }
        },
        _ => rsx! {},
    };

    rsx! {
        div {
            class: "text-black min-h-screen bg-gradient-to-tr from-indigo-500 to-pink-500 flex items-center justify-center",
            style: "border: none; width: 100vw; height: 100vh; overflow: hidden; position: fixed; top: 0; left: 0;",
            div {
                class: "container mx-auto p-10 bg-white text-center rounded w-2/3 md:w-1/3 lg:w-1/3",
                if !error().is_empty() {
                    div {
                        class: "error bg-red-600 text-white px-4 py-3 mb-5 font-semibold rounded-md text-center text-base",
                        "{error()}"
                    }
                }
                header {
                    class: "text-4xl font-semibold mb-3",
                    "Multi-Step Form"
                }
                div {
                    class: "flex items-center text-left justify-center my-10",
                    { render_progress_item(current_step())}
                }
                div {
                    class: "form-outer slide-page text-left",
                    {current_step_content}
                }
            }
        }
    }
}
