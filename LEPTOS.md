# 🌱 Leptos Input RS Usage

Adding Input RS to your Leptos project is simple:

1. Make sure your project is set up with Leptos. Refer to their [Getting Started Guide](https://book.leptos.dev/getting_started/index.html) for setup instructions.

1. Add `input-rs` to your dependencies:

   ```sh
   cargo add input-rs --features=lep
   ```

1. Import the `Input` component into your Leptos component and start using it in your app.

## 🛠️ Usage

Incorporating the `Input` component into your Leptos application is easy. Follow these steps:

1. Import the `Input` component into your Leptos project:

   ```rust
   use leptos::{prelude::*, *};
   use input_rs::leptos::Input;
   use regex::Regex;
   ```

1. Use the `Input` component within your Leptos application:

   ```rust
   use leptos::{prelude::*, *};
   use input_rs::leptos::Input;
   use regex::Regex;


   fn validate_input(value: String) -> bool {
       !value.trim().is_empty()
   }

   #[component]
   pub fn app() -> impl IntoView {
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

           // Custom logic for your endpoint goes here
       };

       view! {
           <div class="form-one-content" role="main" aria-label="Sign In Form">
               <div class="text">
                   <h2>{"Sign In"}</h2>
                   { move || if !error.is_empty() {
                           Some(view! {<div class="error">error</div>})
                       }
                       else {None}
                   }
               </div>
               <form on:submit={onsubmit}>
                   <Input
                       r#type="text"
                       handle={email_handle}
                       name="email"
                       label="Email"
                       placeholder="Email"
                       input_class="form-one-field"
                       field_class="form-one-field"
                       error_class="error-txt"
                       required=true
                       valid_handle={email_valid_handle}
                       validate_function={validate_input}
                       error_message="Enter a valid email address"
                   />
                   <Input
                       r#type="password"
                       handle={password_handle}
                       name="password"
                       label="Password"
                       placeholder="Password"
                       input_class="form-one-field"
                       field_class="form-one-field"
                       error_class="error-txt"
                       required=true
                       valid_handle={password_valid_handle}
                       validate_function={validate_input}
                       error_message="Password can't be blank!"
                       eye_active="fa fa-eye"
                       eye_disabled="fa fa-eye-slash"
                   />
                   <div class="form-one-forgot-pass">
                       <a href="#">{"Forgot Password?"}</a>
                   </div>
                   <button type="submit">{"Sign in"}</button>
                   <div class="sign-up">
                       {"Not a member?"}
                       <a href="#">{"Sign up now"}</a>
                   </div>
               </form>
           </div>
       }
   }
   ```

## 🔧 Props

### `Input` Props

#### Main Props

| Property            | Type                                        | Description                                                | Default  |
| ------------------- | ------------------------------------------- | ---------------------------------------------------------- | -------- |
| `r#type`            | `&'static str`                              | The type of the input element (e.g., `text`, `password`).  | `"text"` |
| `handle`            | `(ReadSignal<String>, WriteSignal<String>)` | State handle for managing the value of the input.          | `""`     |
| `valid_handle`      | `(ReadSignal<bool>, WriteSignal<bool>)`     | State handle for managing the validity state of the input. | `""`     |
| `validate_function` | `fn(String) -> bool`                        | Callback function to validate the input value.             | `""`     |
| `error_message`     | `&'static str`                              | Error message displayed when the input is invalid.         | `""`     |

#### Accessibility and SEO Props

| Property           | Type           | Description                                                     | Default  |
| ------------------ | -------------- | --------------------------------------------------------------- | -------- |
| `id`               | `&'static str` | The ID attribute of the input element.                          | `""`     |
| `aria_label`       | `&'static str` | The aria-label for screen readers.                              | `""`     |
| `aria_required`    | `&'static str` | Indicates whether the input is required.                        | `"true"` |
| `aria_invalid`     | `&'static str` | Indicates whether the input value is invalid.                   | `"true"` |
| `aria_describedby` | `&'static str` | Describes the input element's error message for screen readers. | `""`     |

#### Styling Props

| Property      | Type           | Description                                                          | Default |
| ------------- | -------------- | -------------------------------------------------------------------- | ------- |
| `class`       | `&'static str` | The CSS class for the container element of the input.                | `""`    |
| `input_class` | `&'static str` | The CSS class for the inner input element.                           | `""`    |
| `field_class` | `&'static str` | The CSS class for the input field.                                   | `""`    |
| `label_class` | `&'static str` | The CSS class for the label element.                                 | `""`    |
| `error_class` | `&'static str` | The CSS class for the error message container.                       | `""`    |
| `icon_class`  | `&'static str` | The CSS class for the icon element (for password visibility toggle). | `""`    |

#### Behavioral Props

| Property       | Type           | Description                                     | Default             |
| -------------- | -------------- | ----------------------------------------------- | ------------------- |
| `eye_active`   | `&'static str` | The icon used when the password is visible.     | `"fa fa-eye"`       |
| `eye_disabled` | `&'static str` | The icon used when the password is not visible. | `"fa fa-eye-slash"` |

#### Additional Props

| Property      | Type           | Description                                          | Default |
| ------------- | -------------- | ---------------------------------------------------- | ------- |
| `placeholder` | `&'static str` | The placeholder text displayed in the input element. | `""`    |
| `required`    | `bool`         | Specifies whether the input is required or not.      | `false` |
| `disabled`    | `bool`         | Disables the input when true.                        | `false` |
| `readonly`    | `bool`         | Makes the input read-only when true.                 | `false` |

#### Input Element Specific Attributes

| Property    | Type            | Description                                                     | Default |
| ----------- | --------------- | --------------------------------------------------------------- | ------- |
| `size`      | `Option<u32>`   | The size of the input element (character width).                | `None`  |
| `maxlength` | `Option<usize>` | The maximum number of characters allowed in the input.          | `None`  |
| `pattern`   | `&'static str`  | Regex pattern for input validation.                             | `".*"`  |
| `minlength` | `Option<usize>` | The minimum length of the input value.                          | `None`  |
| `multiple`  | `bool`          | Whether multiple values are allowed (for file or email inputs). | `false` |

#### Behavioral Props

| Property   | Type               | Description                                      | Default |
| ---------- | ------------------ | ------------------------------------------------ | ------- |
| `onchange` | `Callback<String>` | Callback triggered when the input value changes. | No-op   |

## 💡 Notes

- The `Input` component can be used for various input types like text, password, etc.
- You can bind the component to state hooks for two-way data binding.
- Utilize `validate_function` to validate user input and display error messages.
- The `eye_active` and `eye_disabled` props allow for password visibility toggling with FontAwesome icons.
- Customize the appearance with CSS classes for better integration into your app's design.
