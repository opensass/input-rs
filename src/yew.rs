use crate::countries::COUNTRY_CODES;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Props for a custom input component.
/// This struct includes all possible attributes for an HTML `<input>` element.
/// See [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) for more details.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// The type of the input, e.g., "text", "password", etc.
    #[prop_or("text")]
    pub r#type: &'static str,

    /// The label to be displayed for the input field.
    #[prop_or_default]
    pub label: &'static str,

    /// The name of the input field, used for form submission and accessibility.
    #[prop_or_default]
    pub name: &'static str,

    /// Indicates whether the input is required or not.
    #[prop_or_default]
    pub required: bool,

    /// A reference to the DOM node of the input element.
    pub r#ref: NodeRef,

    /// The error message to display when there is a validation error.
    #[prop_or_default]
    pub error_message: &'static str,

    /// The CSS class to be applied to all inner elements.
    #[prop_or_default]
    pub input_class: &'static str,

    /// The CSS class to be applied to the inner input element and icon.
    #[prop_or_default]
    pub field_class: &'static str,

    /// The CSS class to be applied to the label for the input element.
    #[prop_or_default]
    pub label_class: &'static str,

    /// The CSS class to be applied to the input element.
    #[prop_or_default]
    pub class: &'static str,

    /// The CSS class to be applied to the error div element.
    #[prop_or_default]
    pub error_class: &'static str,

    /// The CSS class to be applied to the icon element.
    #[prop_or_default]
    pub icon_class: &'static str,

    /// The state handle for managing the value of the input.
    pub handle: UseStateHandle<String>,

    /// The state handle for managing the validity state of the input.
    pub valid_handle: UseStateHandle<bool>,

    /// A callback function to validate the input value. It takes a `String` as input and returns a `bool`.
    pub validate_function: Callback<String, bool>,

    /// The icon when the password is visible. Assuming fontawesome icons are used by default.
    #[prop_or("cursor-pointer right-4 top-1 text-2xl text-gray-600 toggle-button fa fa-eye")]
    pub eye_active: &'static str,

    /// The icon when the password is not visible. Assuming fontawesome icons are used by default.
    #[prop_or("cursor-pointer right-4 top-1 text-2xl text-gray-600 toggle-button fa fa-eye-slash")]
    pub eye_disabled: &'static str,

    // Accessibility and SEO-related attributes:
    /// The ID attribute of the input element.
    #[prop_or_default]
    pub id: &'static str,

    /// The placeholder text to be displayed in the input element.
    #[prop_or_default]
    pub placeholder: &'static str,

    /// The aria-label attribute for screen readers, providing a label for accessibility.
    #[prop_or_default]
    pub aria_label: &'static str,

    /// The aria-required attribute for screen readers, indicating whether the input is required.
    #[prop_or("true")]
    pub aria_required: &'static str,

    /// The aria-invalid attribute for screen readers, indicating whether the input value is invalid.
    #[prop_or("true")]
    pub aria_invalid: &'static str,

    /// The aria-describedby attribute for screen readers, describing the input element's error message.
    #[prop_or_default]
    pub aria_describedby: &'static str,

    // Newly added attributes from MDN:
    /// Hint for expected file type in file upload controls.
    #[prop_or_default]
    pub accept: &'static str,

    /// The alternative text for `<input type="image">`. Required for accessibility.
    #[prop_or_default]
    pub alt: &'static str,

    /// Controls automatic capitalization in inputted text.
    #[prop_or_default]
    pub autocapitalize: &'static str,

    /// Hint for the browser's autofill feature.
    #[prop_or_default]
    pub autocomplete: &'static str,

    /// Media capture input method in file upload controls.
    #[prop_or_default]
    pub capture: &'static str,

    /// Whether the control is checked (for checkboxes or radio buttons).
    #[prop_or_default]
    pub checked: bool,

    /// Name of the form field to use for sending the element's directionality in form submission.
    #[prop_or_default]
    pub dirname: &'static str,

    /// Whether the form control is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Associates the input with a specific form element.
    #[prop_or_default]
    pub form: &'static str,

    /// URL to use for form submission (for `<input type="image" | "submit">`).
    #[prop_or_default]
    pub formaction: &'static str,

    /// Form data set encoding type for submission (for `<input type="image" | "submit">`).
    #[prop_or_default]
    pub formenctype: &'static str,

    /// HTTP method to use for form submission (for `<input type="image" | "submit">`).
    #[prop_or_default]
    pub formmethod: &'static str,

    /// Bypass form validation for submission (for `<input type="image" | "submit">`).
    #[prop_or_default]
    pub formnovalidate: bool,

    /// Browsing context for form submission (for `<input type="image" | "submit">`).
    #[prop_or_default]
    pub formtarget: &'static str,

    /// Same as the `height` attribute for `<img>` elements.
    #[prop_or_default]
    pub height: Option<u32>,

    /// ID of the `<datalist>` element to use for autocomplete suggestions.
    #[prop_or_default]
    pub list: &'static str,

    /// The maximum value for date, number, range, etc.
    #[prop_or_default]
    pub max: &'static str,

    /// Maximum length of the input value (in characters).
    #[prop_or_default]
    pub maxlength: Option<usize>,

    /// The minimum value for date, number, range, etc.
    #[prop_or_default]
    pub min: &'static str,

    /// Minimum length of the input value (in characters).
    #[prop_or_default]
    pub minlength: Option<usize>,

    /// Boolean indicating whether multiple values are allowed (for file inputs, emails, etc.).
    #[prop_or_default]
    pub multiple: bool,

    /// Regex pattern the value must match to be valid.
    #[prop_or_default]
    pub pattern: &'static str,

    /// Boolean indicating whether the input is read-only.
    #[prop_or_default]
    pub readonly: bool,

    /// Size of the input field (e.g., character width).
    #[prop_or_default]
    pub size: Option<u32>,

    /// Address of the image resource for `<input type="image">`.
    #[prop_or_default]
    pub src: &'static str,

    /// Incremental values that are valid for the input.
    #[prop_or_default]
    pub step: &'static str,

    /// The value of the control (used for two-way data binding).
    #[prop_or_default]
    pub value: &'static str,

    /// Same as the `width` attribute for `<img>` elements.
    #[prop_or_default]
    pub width: Option<u32>,
}

/// A custom input component that handles user input and validation.
///
/// # Arguments
/// * `props` - The properties of the component.
///   - `valid_handle` - A handle to track the validity of the input.
///   - `aria_invalid` - A string representing the 'aria-invalid' attribute value for accessibility. Defaults to "true".
///   - `aria_required` - A string representing the 'aria-required' attribute value for accessibility. Defaults to "true".
///   - `r#type` - The type of the input element. Defaults to "text".
///   - `r#ref` - A reference to the input element.
///   - `handle` - A handle to set the value of the input.
///   - `validate_function` - A callback function to validate the input value.
///
/// # Returns
/// (Html): An HTML representation of the input component.
///
/// # Examples
/// ```
/// use regex::Regex;
/// use serde::{Deserialize, Serialize};
/// use input_rs::yew::Input;
/// use yew::prelude::*;
///
/// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// struct LoginUserSchema {
///     email: String,
///     password: String,
/// }
///
/// fn validate_email(email: String) -> bool {
///     let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
///     pattern.is_match(&email)
/// }
///
/// fn validate_password(password: String) -> bool {
///     !&password.is_empty()
/// }
///
/// #[function_component(LoginFormOne)]
/// pub fn login_one() -> Html {
///     let error_handle = use_state(String::default);
///     let error = (*error_handle).clone();;
///
///     let email_valid_handle = use_state(|| true);
///     let email_valid = (*email_valid_handle).clone();;
///
///     let password_valid_handle = use_state(|| true);
///     let password_valid = (*password_valid_handle).clone();;
///
///     let email_ref = use_node_ref();
///     let email_handle = use_state(String::default);
///     let email = (*email_handle).clone();;
///
///     let password_ref = use_node_ref();
///     let password_handle = use_state(String::default);
///     let password = (*password_handle).clone();;
///
///     let onsubmit = Callback::from(move |event: SubmitEvent| {
///         event.prevent_default();
///
///         let email_ref = password.clone();
///         let password_ref = password.clone();
///         let error_handle = error_handle.clone();
///
///         // Custom logic for your endpoint goes here: `spawn_local`
///     });
///
///     html! {
///         <div class="form-one-content" role="main" aria-label="Sign In Form">
///           <div class="text">
///             <h2>{"Sign In"}</h2>
///             if !error.is_empty() {
///               <div class="error">{error}</div>
///             }
///           </div>
///           <form action="#" aria-label="Sign In Form" onsubmit={onsubmit}>
///               <Input
///                 r#type={"text"}
///                 handle={email_handle}
///                 name={"email"}
///                 r#ref={email_ref}
///                 placeholder={"Email"}
///                 icon_class={"fas fa-user"}
///                 error_message={"Enter a valid email address"}
///                 field_class={"form-one-field"}
///                 error_class={"error-txt"}
///                 required={true}
///                 valid_handle={email_valid_handle}
///                 validate_function={validate_email}
///               />
///               <Input
///                 r#type={"password"}
///                 handle={password_handle}
///                 name={"password"}
///                 r#ref={password_ref}
///                 placeholder={"Password"}
///                 icon_class={"fas fa-lock"}
///                 error_message={"Password can't be blank!"}
///                 field_class={"form-one-field"}
///                 error_class={"error-txt"}
///                 required={true}
///                 valid_handle={password_valid_handle}
///                 validate_function={validate_password}
///                 eye_active={"fa fa-eye"}
///                 eye_disabled={"fa fa-eye-slash"}
///               />
///             <div class="form-one-forgot-pass">
///               <a href="#" aria-label="Forgot Password?">{"Forgot Password?"}</a>
///             </div>
///             <button type="submit">{"Sign in"}</button>
///             <div class="sign-up">
///               {"Not a member?"}
///               <a href="#" aria-label="Sign up now">{"Sign up now"}</a>
///             </div>
///           </form>
///         </div>
///     }
/// }
/// ```
#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let eye_active_handle = use_state(|| false);
    let eye_active = *eye_active_handle;

    let country_ref = use_node_ref();
    let country_handle = use_state(String::default);
    let country = (*country_handle).clone();

    let password_type_handle = use_state(|| "password");
    let password_type = *password_type_handle;

    let valid = *props.valid_handle;

    let eye_icon_active = props.eye_active;

    let eye_icon_disabled = props.eye_disabled;

    let r#type = props.r#type;

    let onchange = {
        let r#ref = props.r#ref.clone();
        let handle = props.handle.clone();
        let valid_handle = props.valid_handle.clone();
        let validate_function = props.validate_function.clone();

        Callback::from(move |_| {
            if let Some(input) = r#ref.cast::<HtmlInputElement>() {
                let value = input.value();
                handle.set(value);
                valid_handle.set(validate_function.emit(input.value()));
            }
        })
    };

    let on_select_change = {
        let country_ref = country_ref.clone();
        let handle = props.handle.clone();
        let country_handle = country_handle.clone();
        Callback::from(move |_| {
            if let Some(input) = country_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                country_handle.set(value);
                handle.set(input.value());
            }
        })
    };

    let on_phone_number_input = {
        let r#ref = props.r#ref.clone();
        let handle = props.handle.clone();
        let country_handle = country_handle;
        Callback::from(move |_| {
            if let Some(input) = r#ref.cast::<HtmlInputElement>() {
                for (code, _, _, _, _, _) in &COUNTRY_CODES {
                    if code.starts_with(&input.value()) {
                        country_handle.set(input.value());
                        break;
                    }
                }
                // Filter out non-numeric characters
                let numeric_value: String =
                    input.value().chars().filter(|c| c.is_numeric()).collect();
                handle.set('+'.to_string() + &numeric_value);
            }
        })
    };

    let on_toggle_password = {
        Callback::from(move |_| {
            if eye_active {
                password_type_handle.set("password")
            } else {
                password_type_handle.set("text")
            }
            eye_active_handle.set(!eye_active);
        })
    };

    let tag = match (*r#type).into() {
        "password" => html! {
            <>
                <input
                    type={password_type}
                    class={props.input_class}
                    id={props.id}
                    name={props.name}
                    value={(*props.handle).clone()}
                    ref={props.r#ref.clone()}
                    placeholder={props.placeholder}
                    aria-label={props.aria_label}
                    aria-required={props.aria_required}
                    aria-invalid={props.aria_invalid}
                    aria-describedby={props.aria_describedby}
                    oninput={onchange}
                    required={props.required}
                    autocomplete={props.autocomplete}
                    autocapitalize={props.autocapitalize}
                    readonly={props.readonly}
                    minlength={props.minlength.map(|v| v.to_string())}
                    maxlength={props.maxlength.map(|v| v.to_string())}
                    pattern={props.pattern}
                    size={props.size.map(|v| v.to_string())}
                    disabled={props.disabled}
                    list={props.list}
                    step={props.step}
                    min={props.min}
                    max={props.max}
                />
                <span
                    class={format!("toggle-button {}", if eye_active { eye_icon_active } else { eye_icon_disabled })}
                    onclick={on_toggle_password}
                />
            </>
        },
        "textarea" => html! {
            <textarea
                class={props.input_class}
                id={props.id}
                name={props.name}
                value={(*props.handle).clone()}
                ref={props.r#ref.clone()}
                placeholder={props.placeholder}
                aria-label={props.aria_label}
                aria-required={props.aria_required}
                aria-invalid={props.aria_invalid}
                aria-describedby={props.aria_describedby}
                oninput={onchange}
                required={props.required}
                autocomplete={props.autocomplete}
                autocapitalize={props.autocapitalize}
                readonly={props.readonly}
                minlength={props.minlength.map(|v| v.to_string())}
                maxlength={props.maxlength.map(|v| v.to_string())}
                disabled={props.disabled}
            />
        },
        "tel" => html! {
            <>
                <select ref={country_ref} onchange={on_select_change}>
                    { for COUNTRY_CODES.iter().map(|(code, emoji, _, name, _, _)| {
                            let selected = *code == country;
                            html! {
                                <option value={*code} selected={selected}>{ format!("{} {} {}", emoji, name, code) }</option>
                            }
                        }) }
                </select>
                <input
                    type="tel"
                    id="telNo"
                    name="telNo"
                    size="20"
                    minlength="9"
                    value={(*props.handle).clone()}
                    maxlength={props.maxlength.map(|v| v.to_string())}
                    class={props.input_class}
                    placeholder={props.placeholder}
                    aria-label={props.aria_label}
                    aria-required={props.aria_required}
                    aria-invalid={props.aria_invalid}
                    oninput={on_phone_number_input}
                    ref={props.r#ref.clone()}
                    autocomplete={props.autocomplete}
                    autocapitalize={props.autocapitalize}
                    readonly={props.readonly}
                    pattern={props.pattern}
                    list={props.list}
                    disabled={props.disabled}
                />
            </>
        },
        _ => html! {
            <input
                type={r#type}
                class={props.input_class}
                id={props.id}
                value={(*props.handle).clone()}
                name={props.name}
                ref={props.r#ref.clone()}
                placeholder={props.placeholder}
                aria-label={props.aria_label}
                aria-required={props.aria_required}
                aria-invalid={props.aria_invalid}
                aria-describedby={props.aria_describedby}
                oninput={onchange}
                required={props.required}
                autocomplete={props.autocomplete}
                autocapitalize={props.autocapitalize}
                readonly={props.readonly}
                minlength={props.minlength.map(|v| v.to_string())}
                maxlength={props.maxlength.map(|v| v.to_string())}
                pattern={props.pattern}
                size={props.size.map(|v| v.to_string())}
                disabled={props.disabled}
                list={props.list}
                step={props.step}
                min={props.min}
                max={props.max}
            />
        },
    };

    html! {
        <div class={props.class}>
            <label class={props.label_class} for={props.id}>{ props.label }</label>
            <div class={props.field_class}>
                { tag }
                <span class={props.icon_class} />
            </div>
            if !valid {
                <div class={props.error_class} id={props.aria_describedby}>
                    { &props.error_message }
                </div>
            }
        </div>
    }
}
