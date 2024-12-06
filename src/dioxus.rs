use crate::countries::COUNTRY_CODES;
use dioxus::prelude::*;

/// Props for a custom input component.
/// This struct includes all possible attributes for an HTML `<input>` element.
/// See [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) for more details.
#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
    /// The type of the input, e.g., "text", "password", etc.
    #[props(default = "text")]
    pub r#type: &'static str,

    /// The label to be displayed for the input field.
    #[props(default = "")]
    pub label: &'static str,

    /// The name of the input field, used for form submission and accessibility.
    #[props(default = "")]
    pub name: &'static str,

    /// Indicates whether the input is required or not.
    #[props(default = false)]
    pub required: bool,

    /// The error message to display when there is a validation error.
    #[props(default = "")]
    pub error_message: &'static str,

    /// The CSS class to be applied to all inner elements.
    #[props(default = "")]
    pub input_class: &'static str,

    /// The CSS class to be applied to the inner input element and icon.
    #[props(default = "")]
    pub field_class: &'static str,

    /// The CSS class to be applied to the label for the input element.
    #[props(default = "")]
    pub label_class: &'static str,

    /// The CSS class to be applied to the input element.
    #[props(default = "")]
    pub class: &'static str,

    /// The CSS class to be applied to the error div element.
    #[props(default = "")]
    pub error_class: &'static str,

    /// The CSS class to be applied to the icon element.
    #[props(default = "")]
    pub icon_class: &'static str,

    /// The state handle for managing the value of the input.
    pub handle: Signal<String>,

    /// The state handle for managing the validity state of the input.
    pub valid_handle: Signal<bool>,

    /// A callback function to validate the input value. It takes a `String` as input and returns a `bool`.
    pub validate_function: fn(String) -> bool,

    /// The icon when the password is visible. Assuming fontawesome icons are used by default.
    #[props(
        default = "cursor-pointer right-4 top-1 text-2xl text-gray-600 toggle-button fa fa-eye"
    )]
    pub eye_active: &'static str,

    /// The icon when the password is not visible. Assuming fontawesome icons are used by default.
    #[props(
        default = "cursor-pointer right-4 top-1 text-2xl text-gray-600 toggle-button fa fa-eye-slash"
    )]
    pub eye_disabled: &'static str,

    // Accessibility and SEO-related attributes:
    /// The ID attribute of the input element.
    #[props(default = "")]
    pub id: &'static str,

    /// The placeholder text to be displayed in the input element.
    #[props(default = "")]
    pub placeholder: &'static str,

    /// The aria-label attribute for screen readers, providing a label for accessibility.
    #[props(default = "")]
    pub aria_label: &'static str,

    /// The aria-required attribute for screen readers, indicating whether the input is required.
    #[props(default = "true")]
    pub aria_required: &'static str,

    /// The aria-invalid attribute for screen readers, indicating whether the input value is invalid.
    #[props(default = "true")]
    pub aria_invalid: &'static str,

    /// The aria-describedby attribute for screen readers, describing the input element's error message.
    #[props(default = "")]
    pub aria_describedby: &'static str,

    // Newly added attributes from MDN:
    /// Hint for expected file type in file upload controls.
    #[props(default = "")]
    pub accept: &'static str,

    /// The alternative text for `<input type="image">`. Required for accessibility.
    #[props(default = "")]
    pub alt: &'static str,

    /// Controls automatic capitalization in inputted text.
    #[props(default = "")]
    pub autocapitalize: &'static str,

    /// Hint for the browser's autofill feature.
    #[props(default = "")]
    pub autocomplete: &'static str,

    /// Media capture input method in file upload controls.
    #[props(default = "")]
    pub capture: &'static str,

    /// Whether the control is checked (for checkboxes or radio buttons).
    #[props(default = false)]
    pub checked: bool,

    /// Name of the form field to use for sending the element's directionality in form submission.
    #[props(default = "")]
    pub dirname: &'static str,

    /// Whether the form control is disabled.
    #[props(default = false)]
    pub disabled: bool,

    /// Associates the input with a specific form element.
    #[props(default = "")]
    pub form: &'static str,

    /// URL to use for form submission (for `<input type="image" | "submit">`).
    #[props(default = "")]
    pub formaction: &'static str,

    /// Form data set encoding type for submission (for `<input type="image" | "submit">`).
    #[props(default = "")]
    pub formenctype: &'static str,

    /// HTTP method to use for form submission (for `<input type="image" | "submit">`).
    #[props(default = "")]
    pub formmethod: &'static str,

    /// Bypass form validation for submission (for `<input type="image" | "submit">`).
    #[props(default = false)]
    pub formnovalidate: bool,

    /// Browsing context for form submission (for `<input type="image" | "submit">`).
    #[props(default = "")]
    pub formtarget: &'static str,

    /// Same as the `height` attribute for `<img>` elements.
    #[props(default = None)]
    pub height: Option<u32>,

    /// ID of the `<datalist>` element to use for autocomplete suggestions.
    #[props(default = "")]
    pub list: &'static str,

    /// The maximum value for date, number, range, etc.
    #[props(default = "")]
    pub max: &'static str,

    /// Maximum length of the input value (in characters).
    #[props(default = None)]
    pub maxlength: Option<usize>,

    /// The minimum value for date, number, range, etc.
    #[props(default = "")]
    pub min: &'static str,

    /// Minimum length of the input value (in characters).
    #[props(default = None)]
    pub minlength: Option<usize>,

    /// Boolean indicating whether multiple values are allowed (for file inputs, emails, etc.).
    #[props(default = false)]
    pub multiple: bool,

    /// Regex pattern the value must match to be valid.
    #[props(default = ".*")]
    pub pattern: &'static str,

    /// Boolean indicating whether the input is read-only.
    #[props(default = false)]
    pub readonly: bool,

    /// Size of the input field (e.g., character width).
    #[props(default = None)]
    pub size: Option<u32>,

    /// Address of the image resource for `<input type="image">`.
    #[props(default = "")]
    pub src: &'static str,

    /// Incremental values that are valid for the input.
    #[props(default = "")]
    pub step: &'static str,

    /// The value of the control (used for two-way data binding).
    #[props(default = "")]
    pub value: &'static str,

    /// Same as the `width` attribute for `<img>` elements.
    #[props(default = None)]
    pub width: Option<u32>,
}

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
/// (Element): A Dioxus element representation of the input component.
///
/// # Examples
/// ```rust
/// use regex::Regex;
/// use input_rs::dioxus::Input;
/// use dioxus::prelude::*;
///
/// #[derive(Debug, Default, Clone)]
/// struct LoginUserSchema {
///     email: String,
///     password: String,
/// }
///
/// fn LoginFormOne() -> Element {
///     let error_handle = use_signal(|| String::new());
///     let email_valid_handle = use_signal(|| true);
///     let password_valid_handle = use_signal(|| true);
///
///     let email_handle = use_signal(|| String::new());
///     let password_handle = use_signal(|| String::new());
///
///     let validate_email = |email: String| {
///         let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
///         pattern.is_match(&email)
///     };
///     
///     let validate_password = |password: String| {
///         !password.is_empty()
///     };
///
///     let onsubmit = {
///         move |e: FormEvent| {
///             e.stop_propagation();
///             // Custom logic for your endpoint goes here.
///         }
///     };
///
///     rsx! {
///         div {
///             class: "form-one-content",
///             role: "main",
///             aria_label: "Sign In Form",
///             div {
///                 class: "text",
///                 h2 { "Sign In" }
///                 if !error_handle().is_empty() {
///                     div { class: "error", "{error_handle}" }
///                 }
///             }
///             form {
///                 onsubmit: onsubmit,
///                 aria_label: "Sign In Form",
///                 Input {
///                     r#type: "text",
///                     handle: email_handle,
///                     name: "email",
///                     placeholder: "Email",
///                     icon_class: "fas fa-user",
///                     error_message: "Enter a valid email address",
///                     field_class: "form-one-field",
///                     error_class: "error-txt",
///                     required: true,
///                     valid_handle: email_valid_handle,
///                     validate_function: validate_email,
///                 }
///                 Input {
///                     r#type: "password",
///                     handle: password_handle,
///                     name: "password",
///                     placeholder: "Password",
///                     icon_class: "fas fa-lock",
///                     error_message: "Password can't be blank!",
///                     field_class: "form-one-field",
///                     error_class: "error-txt",
///                     required: true,
///                     valid_handle: password_valid_handle,
///                     validate_function: validate_password,
///                     eye_active: "fa fa-eye",
///                     eye_disabled: "fa fa-eye-slash",
///                 }
///                 div {
///                     class: "form-one-forgot-pass",
///                     a {
///                         href: "#",
///                         aria_label: "Forgot Password?",
///                         "Forgot Password?"
///                     }
///                 }
///                 button {
///                     r#type: "submit",
///                     "Sign in"
///                 }
///                 div {
///                     class: "sign-up",
///                     "Not a member? ",
///                     a {
///                         href: "#",
///                         aria_label: "Sign up now",
///                         "Sign up now"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Input(mut props: InputProps) -> Element {
    let mut is_eye_active = use_signal(|| false);
    let password_type = if is_eye_active() { "text" } else { "password" };
    let mut country = use_signal(String::default);

    let onchange = {
        move |e: Event<FormData>| {
            let value = e.value();
            props.handle.set(value.clone());
            props.valid_handle.set((props.validate_function)(value));
        }
    };

    let on_select_change = {
        move |e: Event<FormData>| {
            let value = e.value();
            country.set(value.clone());
            props.handle.set(value);
        }
    };

    let on_phone_number_input = {
        move |e: Event<FormData>| {
            let input_value = e.value();
            for (code, _, _, _, _, _) in &COUNTRY_CODES {
                if code.starts_with(&input_value) {
                    country.set(input_value);
                    break;
                }
            }
            // Filter out non-numeric characters
            let numeric_value: String = e.value().chars().filter(|c| c.is_numeric()).collect();
            props.handle.set('+'.to_string() + &numeric_value);
        }
    };

    let toggle_eye_icon = {
        move |_| {
            is_eye_active.set(!is_eye_active());
        }
    };

    let input_field = match props.r#type {
        "password" => rsx! {
            input {
                r#type: "{password_type}",
                class: "{props.input_class}",
                id: "{props.id}",
                name: "{props.name}",
                value: "{props.handle}",
                placeholder: "{props.placeholder}",
                aria_label: "{props.aria_label}",
                aria_required: "{props.aria_required}",
                aria_invalid: "{props.aria_invalid}",
                aria_describedby: "{props.aria_describedby}",
                oninput: onchange,
                required: props.required,
                autocomplete: props.autocomplete,
                autocapitalize: props.autocapitalize,
                readonly: "{props.readonly}",
                disabled: "{props.disabled}",
                minlength: props.minlength.map(|v| v.to_string()).unwrap_or_default(),
                maxlength: props.maxlength.map(|v| v.to_string()).unwrap_or_default(),
                pattern: "{props.pattern}",
                size: props.size.map(|v| v.to_string()).unwrap_or_default(),
            }
            span {
                class: if is_eye_active() { props.eye_active } else { props.eye_disabled },
                onclick: toggle_eye_icon
            }
        },
        "tel" => rsx! {
            select {
                style: "max-width: 55px; font-size: 14px; padding: 10px;",
                onchange: on_select_change,
                { COUNTRY_CODES.iter().map(|(code, emoji, _, name, _, _)| rsx! {
                    option { value: "{code}", selected: *code == country(), "{emoji} {name} ({code})" }
                })}
            }
            input {
                r#type: "tel",
                class: "{props.input_class}",
                id: "{props.id}",
                name: "{props.name}",
                value: "{props.handle}",
                placeholder: "{props.placeholder}",
                aria_label: "{props.aria_label}",
                aria_required: "{props.aria_required}",
                aria_invalid: "{props.aria_invalid}",
                aria_describedby: "{props.aria_describedby}",
                oninput: on_phone_number_input,
                required: props.required,
                autocomplete: props.autocomplete,
                autocapitalize: props.autocapitalize,
                readonly: "{props.readonly}",
                disabled: "{props.disabled}",
                minlength: props.minlength.map(|v| v.to_string()).unwrap_or_default(),
                maxlength: props.maxlength.map(|v| v.to_string()).unwrap_or_default(),
                pattern: "{props.pattern}",
                size: props.size.map(|v| v.to_string()).unwrap_or_default(),
            }
        },
        "textarea" => rsx! {
            textarea {
                class: "{props.input_class}",
                id: "{props.id}",
                name: "{props.name}",
                value: "{props.handle}",
                placeholder: "{props.placeholder}",
                aria_label: "{props.aria_label}",
                aria_required: "{props.aria_required}",
                aria_invalid: "{props.aria_invalid}",
                aria_describedby: "{props.aria_describedby}",
                oninput: onchange,
                required: props.required,
                readonly: "{props.readonly}",
                disabled: "{props.disabled}",
                minlength: props.minlength.map(|v| v.to_string()).unwrap_or_default(),
                maxlength: props.maxlength.map(|v| v.to_string()).unwrap_or_default(),
            }
        },
        _ => rsx! {
            input {
                r#type: "{props.r#type}",
                class: "{props.input_class}",
                id: "{props.id}",
                name: "{props.name}",
                value: "{props.handle}",
                placeholder: "{props.placeholder}",
                aria_label: "{props.aria_label}",
                aria_required: "{props.aria_required}",
                aria_invalid: "{props.aria_invalid}",
                aria_describedby: "{props.aria_describedby}",
                oninput: onchange,
                required: props.required,
                autocomplete: props.autocomplete,
                autocapitalize: props.autocapitalize,
                readonly: "{props.readonly}",
                disabled: "{props.disabled}",
                minlength: props.minlength.map(|v| v.to_string()).unwrap_or_default(),
                maxlength: props.maxlength.map(|v| v.to_string()).unwrap_or_default(),
                pattern: "{props.pattern}",
                size: props.size.map(|v| v.to_string()).unwrap_or_default(),
            }
        },
    };

    rsx! {
        div {
            class: "{props.class}",
            label {
                class: "{props.label_class}",
                r#for: "{props.id}",
                "{props.label}"
            }
            div {
                class: "{props.field_class}",
                {input_field}
                span {class: "{props.icon_class}" }
            }
            if !(props.valid_handle)() {
                div {
                    class: "{props.error_class}",
                    id: "{props.aria_describedby}",
                    "{props.error_message}"
                }
            }
        }
    }
}
