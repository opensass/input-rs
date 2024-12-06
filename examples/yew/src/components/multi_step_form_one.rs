use crate::components::common::{validate_email, validate_input};
use input_rs::yew::Input;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(MultiStepFormOne)]
pub fn multi_step_form_one() -> Html {
    // TODO:  Figure out how to refactor all of this mess!
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let email_valid_handle = use_state(|| true);
    let email_valid = (*email_valid_handle).clone();

    let full_name_valid_handle = use_state(|| true);
    let full_name_valid = (*full_name_valid_handle).clone();

    let phone_number_valid_handle = use_state(|| true);
    let phone_number_valid = (*phone_number_valid_handle).clone();

    let address_valid_handle = use_state(|| true);
    let address_valid = (*address_valid_handle).clone();

    let birthday_valid_handle = use_state(|| true);
    let birthday_valid = (*birthday_valid_handle).clone();

    let gender_valid_handle = use_state(|| true);
    let gender_valid = (*gender_valid_handle).clone();

    let username_valid_handle = use_state(|| true);
    let username_valid = (*username_valid_handle).clone();

    let password_valid_handle = use_state(|| true);
    let password_valid = (*password_valid_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);

    let input_full_name_ref = use_node_ref();
    let input_full_name_handle = use_state(String::default);

    let input_subject_ref = use_node_ref();
    let input_subject_handle = use_state(String::default);

    let input_phone_number_ref = use_node_ref();
    let input_phone_number_handle = use_state(String::default);

    let input_address_ref = use_node_ref();
    let input_address_handle = use_state(String::default);

    let input_birthday_ref = use_node_ref();
    let input_birthday_handle = use_state(String::default);

    let input_gender_ref = use_node_ref();
    let input_gender_handle = use_state(String::default);
    let input_gender = (*input_gender_handle).clone();

    let input_username_ref = use_node_ref();
    let input_username_handle = use_state(String::default);
    let input_username_number = (*input_username_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password_number = (*input_password_handle).clone();

    let current_step_handle = use_state(|| 0);
    let current_step = (*current_step_handle).clone();

    let on_next = {
        let error_handle = error_handle.clone();
        let counter = current_step_handle.clone();
        move |_| match *counter {
            0 => {
                if full_name_valid && email_valid {
                    let value = *counter + 1;
                    counter.set(value);
                    error_handle.set("".to_string());
                } else {
                    error_handle
                        .set("Please provide a valid full name and email address!".to_string());
                }
            }
            1 => {
                if phone_number_valid && address_valid {
                    let value = *counter + 1;
                    counter.set(value);
                    error_handle.set("".to_string());
                } else {
                    error_handle
                        .set("Please provide a valid phone number and address!".to_string());
                }
            }
            2 => {
                if birthday_valid && gender_valid {
                    let value = *counter + 1;
                    counter.set(value);
                    error_handle.set("".to_string());
                } else {
                    error_handle.set("Please provide a valid birth date and gender!".to_string());
                }
            }
            _ => println!("Ain't special"),
        }
    };

    let on_previous = {
        let counter = current_step_handle.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    let on_gender_change = {
        let input_gender_ref = input_gender_ref.clone();

        Callback::from(move |_| {
            let input = input_gender_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_gender_handle.set(value);
                gender_valid_handle.set(validate_input(input.value()));
            }
        })
    };

    // TODO: add an onclick handler on last button

    let render_progress_item = |(index, _): (usize, &str)| {
        // TODO: Refactor and convert to a for loop/map, make this tag show up: <span class="check bg-pink-800 z-50"></span>
        html! {
            <div class="flex items-center text-left justify-center my-10">
                <div class="step">
                    <p class="font-semibold mb-2 text-pink-800">{ "Step 1" }</p>
                    <div class="flex items-center">
                        <div
                            class="outer-check bullet border-2 border-pink-800 rounded-full h-7 w-7 flex items-center justify-center"
                        >
                            <span class="step-index font-semibold text-pink-800">{ "1" }</span>
                            if index > 0 {
                                <span class="check bg-pink-800 z-50" />
                            }
                        </div>
                        if index > 0 {
                            <span class="line bg-pink-800 h-1 w-10" />
                        } else {
                            <span class="line bg-grey-400 h-1 w-10" />
                        }
                    </div>
                </div>
                <div class="step">
                    <p class="font-semibold mb-2 text-pink-800">{ "Step 2" }</p>
                    <div class="flex items-center">
                        <div
                            class="outer-check bullet border-2 border-pink-800 rounded-full h-7 w-7 flex items-center justify-center"
                        >
                            <span class="step-index font-semibold text-pink-800">{ "2" }</span>
                            if index > 1 {
                                <span class="check bg-pink-800" />
                            }
                        </div>
                        if index > 1 {
                            <span class="line bg-pink-800 h-1 w-10" />
                        } else {
                            <span class="line bg-grey-400 h-1 w-10" />
                        }
                    </div>
                </div>
                <div class="step">
                    <p class="font-semibold mb-2 text-pink-800">{ "Step 3" }</p>
                    <div class="flex items-center">
                        <div
                            class="outer-check bullet border-2 border-pink-800 rounded-full h-7 w-7 flex items-center justify-center"
                        >
                            <span class="step-index font-semibold text-pink-800">{ "3" }</span>
                            if index > 2 {
                                <span class="check bg-pink-800" />
                            }
                        </div>
                        if index > 2 {
                            <span class="line bg-pink-800 h-1 w-10" />
                        } else {
                            <span class="line bg-grey-400 h-1 w-10" />
                        }
                    </div>
                </div>
                <div class="step">
                    <p class="font-semibold mb-2 text-pink-800">{ "Step 4" }</p>
                    <div class="flex items-center">
                        <div
                            class="outer-check bullet border-2 border-pink-800 rounded-full h-7 w-7 flex items-center justify-center"
                        >
                            <span class="step-index font-semibold text-pink-800">{ "4" }</span>
                        </div>
                    </div>
                </div>
            </div>
        }
    };

    let current_step_content = match current_step {
        0 => html! {
            <div class="page ml-0 transition-transform duration-300">
                <div class="title text-left text-xl font-semibold mb-4">
                    { "Personal Information" }
                </div>
                <Input
                    r#type="text"
                    label="Full Name"
                    handle={input_full_name_handle}
                    name="full-name"
                    r#ref={input_full_name_ref}
                    placeholder="Full Name"
                    error_message="Full name can't be blank!"
                    class="field mb-6"
                    field_class="validate-input mb-6"
                    label_class="label font-semibold text-pink-800"
                    input_class="w-full border border-pink-800 rounded px-4 py-2"
                    error_class="text-red-500 text-sm my-2"
                    required=true
                    valid_handle={full_name_valid_handle}
                    validate_function={validate_input}
                />
                <Input
                    r#type="text"
                    label="Email"
                    handle={input_email_handle}
                    name="email"
                    r#ref={input_email_ref}
                    placeholder="Email"
                    error_message="Enter a valid email address!"
                    class="field mb-6"
                    field_class="validate-input mb-6"
                    label_class="label font-semibold text-pink-800"
                    input_class="w-full border border-pink-800 rounded px-4 py-2"
                    error_class="text-red-500 text-sm my-2"
                    required=true
                    valid_handle={email_valid_handle}
                    validate_function={validate_email}
                />
                <button
                    class="next bg-purple-800 hover:bg-purple-700 text-white rounded px-4 mt-10 py-2 w-full font-semibold mb-4"
                    onclick={on_next}
                >
                    { "Next" }
                </button>
            </div>
        },
        1 => html! {
            <div class="page transition-transform duration-300">
                <div class="title text-left text-xl font-semibold mb-4">{ "Contact Details" }</div>
                <Input
                    r#type="tel"
                    label="Phone Number"
                    handle={input_phone_number_handle}
                    name="phone-number"
                    r#ref={input_phone_number_ref}
                    placeholder="+19999"
                    error_message="Phone number can't be blank!"
                    class="field mb-6"
                    field_class="flex telephone-input validate-input mb-6"
                    label_class="label font-semibold text-pink-800"
                    input_class="w-full border border-pink-800 rounded px-4 py-2"
                    error_class="text-red-500 text-sm my-2"
                    required=true
                    valid_handle={phone_number_valid_handle}
                    validate_function={validate_input}
                />
                <Input
                    r#type="text"
                    label="Address"
                    handle={input_address_handle}
                    name="address"
                    r#ref={input_address_ref}
                    placeholder="Address"
                    error_message="Address can't be blank!"
                    class="field mb-6"
                    field_class="validate-input mb-6"
                    label_class="label font-semibold text-pink-800"
                    input_class="w-full border border-pink-800 rounded px-4 py-2"
                    error_class="text-red-500 text-sm my-2"
                    required=true
                    valid_handle={address_valid_handle}
                    validate_function={validate_input}
                />
                <div class="field btns text-center space-x-5 mt-10">
                    <button
                        class="prev bg-pink-800 hover:bg-pink-700 text-white rounded px-4 py-2 font-semibold"
                        onclick={on_previous}
                    >
                        { "Previous" }
                    </button>
                    <button
                        class="next bg-purple-800 hover:bg-purple-700 text-white rounded px-4 py-2 font-semibold"
                        onclick={on_next}
                    >
                        { "Next" }
                    </button>
                </div>
            </div>
        },
        2 => html! {
            <div class="page transition-transform duration-300">
                <div class="title text-left text-xl font-semibold mb-4">{ "Date of Birth" }</div>
                <Input
                    r#type="date"
                    label="Date of Birth"
                    handle={input_birthday_handle}
                    name="birthday"
                    r#ref={input_birthday_ref}
                    placeholder="Birthday"
                    error_message="Birthday can't be blank!"
                    class="field mb-6"
                    field_class="validate-input mb-6"
                    label_class="label font-semibold text-pink-800"
                    input_class="w-full border border-pink-800 rounded px-4 py-2"
                    error_class="text-red-500 text-sm my-2"
                    required=true
                    valid_handle={birthday_valid_handle}
                    validate_function={validate_input}
                />
                <div class="field mb-6">
                    <div class="label font-semibold text-pink-800">{ "Gender" }</div>
                    <select
                        class="w-full border border-pink-800 rounded px-4 py-2"
                        id="gender"
                        name="gender"
                        ref={input_gender_ref}
                        aria-required="true"
                        required=true
                        placeholder="Gender"
                        oninput={on_gender_change}
                    >
                        <option>{ "Male" }</option>
                        <option>{ "Female" }</option>
                        <option>{ "Non-binary" }</option>
                        <option>{ "Other" }</option>
                    </select>
                </div>
                if !gender_valid {
                    <div class="error-txt text-red-500 text-sm my-2">
                        { "Gender can't be blank!" }
                    </div>
                }
                <div class="field btns text-center space-x-5 mt-10">
                    <button
                        class="prev bg-pink-800 hover:bg-pink-700 text-white rounded px-4 py-2 font-semibold"
                        onclick={on_previous}
                    >
                        { "Previous" }
                    </button>
                    <button
                        class="next bg-purple-800 hover:bg-purple-700 text-white rounded px-4 py-2 font-semibold"
                        onclick={on_next}
                    >
                        { "Next" }
                    </button>
                </div>
            </div>
        },
        3 => html! {
            <div class="page transition-transform duration-300">
                <div class="title text-left text-xl font-semibold mb-4">{ "Account Details" }</div>
                <Input
                    r#type="text"
                    label="Username"
                    handle={input_username_handle}
                    name="username"
                    r#ref={input_username_ref}
                    placeholder="Username"
                    error_message="Username can't be blank!"
                    class="field mb-6"
                    field_class="validate-input mb-6"
                    label_class="label font-semibold text-pink-800"
                    input_class="w-full border border-pink-800 rounded px-4 py-2"
                    error_class="text-red-500 text-sm my-2"
                    required=true
                    valid_handle={username_valid_handle}
                    validate_function={validate_input}
                />
                <Input
                    r#type={"password"}
                    label={"Password"}
                    handle={input_password_handle}
                    name={"password"}
                    r#ref={input_password_ref}
                    placeholder={"Password"}
                    error_message={"Password can't be blank!"}
                    field_class={"relative mt-2 mb-2"}
                    input_class={"input w-full px-4 py-2 rounded border border-pink-800 bg-gray-100"}
                    error_class={"text-red-500 absolute text-sm"}
                    required={true}
                    valid_handle={password_valid_handle}
                    validate_function={validate_input}
                    eye_active={"cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-2xl text-gray-600 toggle-button fa fa-eye"}
                    eye_disabled={"cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-2xl text-gray-600 toggle-button fa fa-eye-slash"}
                />
                <div class="field btns text-center space-x-5 mt-10">
                    <button
                        class="prev bg-pink-800 hover:bg-pink-700 text-white rounded px-4 py-2 font-semibold"
                        onclick={on_previous}
                    >
                        { "Previous" }
                    </button>
                    <button
                        class="submit bg-purple-800 hover:bg-purple-700 text-white rounded px-4 py-2 font-semibold"
                    >
                        { "Submit" }
                    </button>
                </div>
            </div>
        },
        _ => html! {},
    };

    html! {
        <div
            class="min-h-screen bg-gradient-to-tr from-indigo-500 to-pink-500 flex items-center justify-center"
        >
            <div
                class="container mx-auto p-10 bg-white text-center rounded w-2/3 md:w-1/3 lg:w-1/3"
            >
                if !error.is_empty() {
                    <div
                        class="error bg-red-600 text-white px-4 py-3 mb-5 font-semibold rounded-md text-center text-base"
                    >
                        { error }
                    </div>
                }
                <header class="text-4xl font-semibold mb-3">{ "Multi-Step Form" }</header>
                <div class="flex items-center text-left justify-center my-3">
                    { render_progress_item((current_step, "")) }
                </div>
                <div class="form-outer slide-page  text-left">{ current_step_content }</div>
            </div>
        </div>
    }
}
