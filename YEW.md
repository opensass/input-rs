# Y Input RS Yew Usage

Adding Input RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Input RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add input-rs --features=yew
   ```

1. Import the `Input` component into your Yew component and start using it in your app.

## 🛠️ Usage

Incorporating Yew Input RS into your application is easy. Follow these steps:

1. Import the `Input` component into your Yew project:

   ```rust
   use yew::prelude::*;
   use input_rs::yew::Input;
   use regex::Regex;
   ```

1. Use the `Input` component within your Yew application:

   ```rust
   use yew::prelude::*;
   use regex::Regex;
   use input_rs::yew::Input;

   fn validate_email(email: String) -> bool {
       let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
       pattern.is_match(&email)
   }

   #[function_component(App)]
   pub fn app() -> Html {
       let input_email_ref = use_node_ref();
       let input_email_handle = use_state(String::default);
       let email_valid_handle = use_state(|| true);

       html! {
           <form>
               <Input
                   r#type="email"
                   name="email"
                   r#ref={input_email_ref}
                   handle={input_email_handle}
                   valid_handle={email_valid_handle}
                   validate_function={validate_email}
                   placeholder="Enter your email"
                   label="Email Address"
                   required={true}
                   error_message="Please provide a valid email address"
                   class="form-field"
                   label_class="form-label"
                   input_class="form-input"
                   error_class="error-text"
               />
           </form>
       }
   }
   ```

## 🔧 Props

### Main Props

| Property            | Type                     | Description                                                            | Default  |
| ------------------- | ------------------------ | ---------------------------------------------------------------------- | -------- |
| `type`              | `&'static str`           | Input type, e.g., `"text"`, `"email"`, `"password"`, `"textarea"`.     | `"text"` |
| `name`              | `&'static str`           | Name attribute for the input element.                                  | `""`     |
| `label`             | `&'static str`           | Text label displayed above the input.                                  | `""`     |
| `placeholder`       | `&'static str`           | Placeholder text inside the input field.                               | `""`     |
| `id`                | `&'static str`           | ID attribute for the input element.                                    | `""`     |
| `required`          | `bool`                   | Indicates whether the field is required.                               | `false`  |
| `handle`            | `UseStateHandle<String>` | State handle for managing the value of the input.                      | None     |
| `valid_handle`      | `UseStateHandle<bool>`   | State handle for managing the validity of the input value.             | None     |
| `validate_function` | `Callback<String, bool>` | Validation function that checks the input value and returns a boolean. | None     |
| `error_message`     | `&'static str`           | Message displayed when the input value is invalid.                     | `""`     |

### Styling Props

```sh
+-----------------------------+  <-- `class`
|                             |
|  +-----------------------+  |  <-- `label_class`
|  |       Label           |  |
|  +-----------------------+  |
|                             |
|  +-----------------------+  |  <-- `field_class`
|  | +-------+  +--------+ |  |
|  | | Input |  |  Icon  | |  |  <-- `input_class` and `icon_class`
|  | +-------+  +--------+ |  |
|  +-----------------------+  |
|                             |
|  +-----------------------+  |  <-- `error_class` (if invalid)
|  |       Error Message   |  |
|  +-----------------------+  |
+-----------------------------+
```

| Property      | Type           | Description                                                             | Default |
| ------------- | -------------- | ----------------------------------------------------------------------- | ------- |
| `class`       | `&'static str` | CSS class applied to the wrapper container.                             | `""`    |
| `label_class` | `&'static str` | CSS class applied to the label element.                                 | `""`    |
| `input_class` | `&'static str` | CSS class applied to the input element.                                 | `""`    |
| `field_class` | `&'static str` | CSS class applied to the input wrapper element (includes icons, etc.).  | `""`    |
| `error_class` | `&'static str` | CSS class applied to the error message container when validation fails. | `""`    |
| `icon_class`  | `&'static str` | CSS class applied to the optional icon (if specified).                  | `""`    |

### Password Icon Props

| Property       | Type           | Description                                                        | Default                                                                               |
| -------------- | -------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| `eye_active`   | `&'static str` | Icon CSS class for showing the "visible" state in password fields. | `"cursor-pointer right-4 top-1 text-2xl text-gray-600 toggle-button fa fa-eye"`       |
| `eye_disabled` | `&'static str` | Icon CSS class for showing the "hidden" state in password fields.  | `"cursor-pointer right-4 top-1 text-2xl text-gray-600 toggle-button fa fa-eye-slash"` |

### Accessibility Props

| Property           | Type           | Description                                                                 | Default   |
| ------------------ | -------------- | --------------------------------------------------------------------------- | --------- |
| `aria_label`       | `&'static str` | Aria-label for the input element for screen reader users.                   | `""`      |
| `aria_required`    | `&'static str` | Specifies whether the input is required for screen readers.                 | `"true"`  |
| `aria_invalid`     | `&'static str` | Indicates whether the input value is invalid for screen readers.            | `"false"` |
| `aria_describedby` | `&'static str` | ID of the element that describes the input (e.g., error message container). | `""`      |

## 💡 Notes

- The `Input` component can be used for various input types like text, password, etc.
- You can bind the component to state hooks for two-way data binding.
- Utilize `validate_function` to validate user input and display error messages.
- The `eye_active` and `eye_disabled` props allow for password visibility toggling with FontAwesome icons.
- Customize the appearance with CSS classes for better integration into your app's design.
