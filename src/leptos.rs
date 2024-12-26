#![allow(unused)]

use crate::countries::COUNTRY_CODES;
use leptos::{ev::MouseEvent, prelude::*, *};

/// A custom input component that handles user input and validation.
///
/// # Arguments
/// * `props` - The properties of the component.
///   - `valid_handle` - A state hook to track the validity of the input.
///   - `aria_invalid` - A string representing the 'aria-invalid' attribute value for accessibility. Defaults to "true".
///   - `aria_required` - A string representing the 'aria-required' attribute value for accessibility. Defaults to "true".
///   - `r#type` - The type of the input element. Defaults to "text".
///   - `handle` - A state hook to set the value of the input.
///   - `validate_function` - A function to validate the input value.
///
/// # Returns
/// (IntoView): A Leptos element representation of the input component.
///
/// # Examples
/// ```rust
/// use leptos::{prelude::*, *};
/// use regex::Regex;
/// use serde::{Deserialize, Serialize};
/// use input_rs::leptos::Input;
///
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
/// #[component]
/// fn LoginForm() -> impl IntoView {
///     let error_handle = signal(String::default());
///     let error = error_handle.0.get();
///
///     let email_valid_handle = signal(true);
///     let email_valid = email_valid_handle.0.get();
///
///     let password_valid_handle = signal(true);
///     let password_valid = password_valid_handle.0.get();
///
///     let email_handle = signal(String::default());
///     let email = email_handle.0.get();
///
///     let password_handle = signal(String::default());
///     let password = password_handle.0.get();
///
///     let onsubmit = move |ev: leptos::ev::SubmitEvent| {
///         ev.prevent_default();
///
///         let email_ref = email.clone();
///         let password_ref = password.clone();
///         let error_handle = error_handle.clone();
///
///         // Custom logic for your endpoint goes here
///     };
///
///     view! {
///         <div class="form-one-content" role="main" aria-label="Sign In Form">
///             <div class="text">
///                 <h2>{"Sign In"}</h2>
///                 { move || if !error.is_empty() {
///                         Some(view! {<div class="error">error</div>})
///                     }
///                         else {None}
///                 }
///             </div>
///             <form on:submit={onsubmit}>
///                 <Input
///                     r#type="text"
///                     handle={email_handle}
///                     name="email"
///                     label="Email"
///                     placeholder="Email"
///                     input_class="form-one-field"
///                     field_class="form-one-field"
///                     error_class="error-txt"
///                     required=true
///                     valid_handle={email_valid_handle}
///                     validate_function={validate_email}
///                     error_message="Enter a valid email address"
///                 />
///                 <Input
///                     r#type="password"
///                     handle={password_handle}
///                     name="password"
///                     label="Password"
///                     placeholder="Password"
///                     input_class="form-one-field"
///                     field_class="form-one-field"
///                     error_class="error-txt"
///                     required=true
///                     valid_handle={password_valid_handle}
///                     validate_function={validate_password}
///                     error_message="Password can't be blank!"
///                     eye_active="fa fa-eye"
///                     eye_disabled="fa fa-eye-slash"
///                 />
///                 <div class="form-one-forgot-pass">
///                     <a href="#">{"Forgot Password?"}</a>
///                 </div>
///                 <button type="submit">{"Sign in"}</button>
///                 <div class="sign-up">
///                     {"Not a member?"}
///                     <a href="#">{"Sign up now"}</a>
///                 </div>
///             </form>
///         </div>
///     }
/// }
/// ```
#[component]
pub fn Input(
    /// Props for a custom input component.
    /// This struct includes all possible attributes for an HTML `<input>` element.
    /// See [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) for more details.
    ///
    /// The type of the input, e.g., "text", "password", etc.
    #[prop(default = "text")]
    r#type: &'static str,

    /// The label to be displayed for the input field.
    #[prop(default = "")]
    label: &'static str,

    /// The name of the input field, used for form submission and accessibility.
    #[prop(default = "")]
    name: &'static str,

    /// Indicates whether the input is required or not.
    #[prop(default = false)]
    required: bool,

    /// The error message to display when there is a validation error.
    #[prop(default = "")]
    error_message: &'static str,

    /// The CSS class to be applied to all inner elements.
    #[prop(default = "")]
    input_class: &'static str,

    /// The CSS class to be applied to the inner input element and icon.
    #[prop(default = "")]
    field_class: &'static str,

    /// The CSS class to be applied to the label for the input element.
    #[prop(default = "")]
    label_class: &'static str,

    /// The CSS class to be applied to the input element.
    #[prop(default = "")]
    class: &'static str,

    /// The CSS class to be applied to the error div element.
    #[prop(default = "")]
    error_class: &'static str,

    /// The CSS class to be applied to the icon element.
    #[prop(default = "")]
    icon_class: &'static str,

    /// The state handle for managing the value of the input.
    handle: (ReadSignal<String>, WriteSignal<String>),

    /// The state handle for managing the validity state of the input.
    valid_handle: (ReadSignal<bool>, WriteSignal<bool>),

    /// A callback function to validate the input value. It takes a `String` as input and returns a `bool`.
    validate_function: fn(String) -> bool,

    /// The icon when the password is visible. Assuming fontawesome icons are used by default.
    #[prop(
        default = "cursor-pointer right-4 top-1 text-2xl text-gray-600 toggle-button fa fa-eye"
    )]
    eye_active: &'static str,

    /// The icon when the password is not visible. Assuming fontawesome icons are used by default.
    #[prop(
        default = "cursor-pointer right-4 top-1 text-2xl text-gray-600 toggle-button fa fa-eye-slash"
    )]
    eye_disabled: &'static str,

    // Accessibility and SEO-related attributes:
    /// The ID attribute of the input element.
    #[prop(default = "")]
    id: &'static str,

    /// The placeholder text to be displayed in the input element.
    #[prop(default = "")]
    placeholder: &'static str,

    /// The aria-label attribute for screen readers, providing a label for accessibility.
    #[prop(default = "")]
    aria_label: &'static str,

    /// The aria-required attribute for screen readers, indicating whether the input is required.
    #[prop(default = "true")]
    aria_required: &'static str,

    /// The aria-invalid attribute for screen readers, indicating whether the input value is invalid.
    #[prop(default = "true")]
    aria_invalid: &'static str,

    /// The aria-describedby attribute for screen readers, describing the input element's error message.
    #[prop(default = "")]
    aria_describedby: &'static str,

    // Newly added attributes from MDN:
    /// Hint for expected file type in file upload controls.
    #[prop(default = "")]
    accept: &'static str,

    /// The alternative text for `<input r#type="image">`. Required for accessibility.
    #[prop(default = "")]
    alt: &'static str,

    /// Controls automatic capitalization in inputted text.
    #[prop(default = "")]
    autocapitalize: &'static str,

    /// Hint for the browser's autofill feature.
    #[prop(default = "")]
    autocomplete: &'static str,

    /// Media capture input method in file upload controls.
    #[prop(default = "")]
    capture: &'static str,

    /// Whether the control is checked (for checkboxes or radio buttons).
    #[prop(default = false)]
    checked: bool,

    /// Name of the form field to use for sending the element's directionality in form submission.
    #[prop(default = "")]
    dirname: &'static str,

    /// Whether the form control is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Associates the input with a specific form element.
    #[prop(default = "")]
    form: &'static str,

    /// URL to use for form submission (for `<input r#type="image" | "submit">`).
    #[prop(default = "")]
    formaction: &'static str,

    /// Form data set encoding type for submission (for `<input r#type="image" | "submit">`).
    #[prop(default = "")]
    formenctype: &'static str,

    /// HTTP method to use for form submission (for `<input r#type="image" | "submit">`).
    #[prop(default = "")]
    formmethod: &'static str,

    /// Bypass form validation for submission (for `<input r#type="image" | "submit">`).
    #[prop(default = false)]
    formnovalidate: bool,

    /// Browsing context for form submission (for `<input r#type="image" | "submit">`).
    #[prop(default = "")]
    formtarget: &'static str,

    /// Same as the `height` attribute for `<img>` elements.
    #[prop(default = None)]
    height: Option<u32>,

    /// ID of the `<datalist>` element to use for autocomplete suggestions.
    #[prop(default = "")]
    list: &'static str,

    /// The maximum value for date, number, range, etc.
    #[prop(default = "")]
    max: &'static str,

    /// Maximum length of the input value (in characters).
    #[prop(default = None)]
    maxlength: Option<usize>,

    /// The minimum value for date, number, range, etc.
    #[prop(default = "")]
    min: &'static str,

    /// Minimum length of the input value (in characters).
    #[prop(default = None)]
    minlength: Option<usize>,

    /// Boolean indicating whether multiple values are allowed (for file inputs, emails, etc.).
    #[prop(default = false)]
    multiple: bool,

    /// Regex pattern the value must match to be valid.
    #[prop(default = ".*")]
    pattern: &'static str,

    /// Boolean indicating whether the input is read-only.
    #[prop(default = false)]
    readonly: bool,

    /// Size of the input field (e.g., character width).
    #[prop(default = None)]
    size: Option<u32>,

    /// Address of the image resource for `<input r#type="image">`.
    #[prop(default = "")]
    src: &'static str,

    /// Incremental values that are valid for the input.
    #[prop(default = "")]
    step: &'static str,

    /// The value of the control (used for two-way data binding).
    #[prop(default = "")]
    value: &'static str,

    /// Same as the `width` attribute for `<img>` elements.
    #[prop(default = None)]
    width: Option<u32>,
) -> impl IntoView {
    let (eye_active_handle, set_eye_active_handle) = signal(false);
    let (password_type, set_password_type) = signal("password".to_string());
    let valid = valid_handle;
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    let onchange = {
        move |ev: web_sys::Event| {
            let input_value = input_ref.get().expect("<input> should be mounted").value();
            handle.1.set(input_value.clone());
            valid.1.set(validate_function(input_value));
        }
    };

    let on_toggle_password = {
        move |ev: MouseEvent| {
            if eye_active_handle.get() {
                set_password_type.set("password".to_string());
            } else {
                set_password_type.set("text".to_string());
            }
            set_eye_active_handle.set(!eye_active_handle.get());
        }
    };

    let tag = {
        move || {
            match r#type {
            "password" => Some(view! {
                <>
                    <input
                        r#type={password_type.get()}
                        class={input_class}
                        id={id}
                        name={name}
                        value={handle.0.get()}
                        placeholder={placeholder}
                        aria-label={aria_label}
                        aria-required={aria_required}
                        aria-invalid={aria_invalid}
                        aria-describedby={aria_describedby}
                        on:input={onchange}
                        required={required}
                        node_ref={input_ref}
                        autocomplete={autocomplete}
                        autocapitalize={autocapitalize}
                        readonly={readonly}
                        minlength={minlength.map(|v| v.to_string())}
                        maxlength={maxlength.map(|v| v.to_string())}
                        pattern={pattern}
                        size={size.map(|v| v.to_string())}
                        disabled={disabled}
                        list={list}
                        step={step}
                        min={min}
                        max={max}
                        accept={accept}
                    />
                    <span
                        class={if eye_active_handle.get() { eye_active } else { eye_disabled }}
                        on:click={on_toggle_password}
                    />
                </>
            }.into_any()),
            // "textarea" => Some(view! {
            //     <>
            //         <textarea
            //             class={input_class}
            //             id={id}
            //             name={name}
            //             placeholder={placeholder}
            //             aria-label={aria_label}
            //             aria-required={aria_required}
            //             aria-invalid={aria_invalid}
            //             aria-describedby={aria_describedby}
            //             on:input={onchange}
            //             required={required}
            //             node_ref={input_ref}
            //         />
            //     </>
            // }.into_any()),
            "tel" => Some(view! {
                <>
                    <select class={field_class} on:change={onchange}>
                        <For
                            each=move || COUNTRY_CODES
                            key=|country| *country
                            let:country
                        >
                            {move || {
                                let (code, emoji, _, name, _, _) = country;
                                view! {
                                    <option value={code} selected={*code == handle.0.get()}>{ format!("{} {} {}", emoji, name, code) }</option>
                                }
                            }}
                        </For>
                    </select>
                    <input
                        r#type={"tel"}
                        class={input_class}
                        id={id}
                        name={name}
                        value={handle.0.get()}
                        placeholder={placeholder}
                        aria-label={aria_label}
                        aria-required={aria_required}
                        aria-invalid={aria_invalid}
                        aria-describedby={aria_describedby}
                        on:input={onchange}
                        required={required}
                        node_ref={input_ref}
                        autocomplete={autocomplete}
                        autocapitalize={autocapitalize}
                        readonly={readonly}
                        minlength={minlength.map(|v| v.to_string())}
                        maxlength={maxlength.map(|v| v.to_string())}
                        pattern={pattern}
                        size={size.map(|v| v.to_string())}
                        disabled={disabled}
                        list={list}
                        step={step}
                        min={min}
                        max={max}
                        accept={accept}
                    />
                </>
            }.into_any()),
            _ => Some(view! {
                <>
                    <input
                        r#type={r#type.to_string()}
                        class={input_class}
                        id={id}
                        name={name}
                        value={handle.0.get()}
                        placeholder={placeholder}
                        aria-label={aria_label}
                        aria-required={aria_required}
                        aria-invalid={aria_invalid}
                        aria-describedby={aria_describedby}
                        on:input={onchange}
                        required={required}
                        node_ref={input_ref}
                        autocomplete={autocomplete}
                        autocapitalize={autocapitalize}
                        readonly={readonly}
                        minlength={minlength.map(|v| v.to_string())}
                        maxlength={maxlength.map(|v| v.to_string())}
                        pattern={pattern}
                        size={size.map(|v| v.to_string())}
                        disabled={disabled}
                        list={list}
                        step={step}
                        min={min}
                        max={max}
                        accept={accept}
                    />
                </>
            }.into_any()),
        }
        }
    };

    view! {
        <div class={class}>
            <label class={label_class} for={id}>{label}</label>
            <div class={field_class}>
                {tag}
                <span class={icon_class} />
            </div>
            {move ||
                if !valid.0.get() {
                    Some(view! {
                        <div class={error_class} id={aria_describedby}>
                            {error_message}
                        </div>
                    })
                } else {
                    None
                }
            }
        </div>
    }
}
