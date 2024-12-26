# 🧬 Input RS Dioxus Usage

Adding Input RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the Input component to your dependencies by including it in your `Cargo.toml` file.

   ```sh
   cargo add input-rs --features=dio
   ```

1. Import the `Input` components into your Dioxus component and start using it in your app.

## 🛠️ Usage

Incorporating the Input component into your application is easy. Follow these steps:

1. Import `Input` into your component:

   ```rust
   use dioxus::prelude::*;
   use input_rs::dioxus::Input;
   ```

1. Use the `Input` component in your application:

   ```rust
   use dioxus::prelude::*;
   use input_rs::dioxus::Input;

   #[component]
   pub fn App() -> Element {
       let input_value = use_signal(|| String::new());
       let is_valid = use_signal(|| true);

       fn validate_input(value: String) -> bool {
           !value.trim().is_empty()
       }

       rsx! {
           div {
               class: "app-container",
               h1 { "Custom Input Example" }
               Input {
                   r#type: "text",
                   label: "Enter your name:",
                   id: "name-input",
                   handle: input_value.clone(),
                   valid_handle: is_valid.clone(),
                   validate_function: validate_input,
                   placeholder: "Type here...",
                   class: "custom-input",
                   label_class: "input-label",
                   field_class: "input-field",
                   error_message: "This field cannot be empty",
                   error_class: "input-error",
               }
               if !is_valid() {
                   p { class: "error-message", "Please correct the input." }
               }
           }
       }
   }
   ```

## 🔧 Props

### Main Props

| Property            | Type                 | Description                                                                          | Default  |
| ------------------- | -------------------- | ------------------------------------------------------------------------------------ | -------- |
| `r#type`            | `&'static str`       | Type of the input (e.g., `text`, `password`, `email`, etc.).                         | `"text"` |
| `label`             | `&'static str`       | Label text for the input field.                                                      | `""`     |
| `id`                | `&'static str`       | Unique ID for the input element.                                                     | `""`     |
| `placeholder`       | `&'static str`       | Placeholder text for the input.                                                      | `""`     |
| `handle`            | `Signal<String>`     | Signal handle for the input value.                                                   | None     |
| `valid_handle`      | `Signal<bool>`       | Signal handle for the validity of the input value.                                   | None     |
| `validate_function` | `fn(String) -> bool` | Validation function for the input value. Returns `true` if valid, `false` otherwise. | None     |
| `required`          | `bool`               | Indicates whether the input is required.                                             | `false`  |
| `error_message`     | `&'static str`       | Error message to display if the input is invalid.                                    | `""`     |

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

| Property      | Type           | Description                                | Default |
| ------------- | -------------- | ------------------------------------------ | ------- |
| `class`       | `&'static str` | CSS class for the input container.         | `""`    |
| `label_class` | `&'static str` | CSS class for the label element.           | `""`    |
| `input_class` | `&'static str` | CSS class applied to the input element.    | `""`    |
| `field_class` | `&'static str` | CSS class for the input field container.   | `""`    |
| `error_class` | `&'static str` | CSS class for the error message container. | `""`    |

### Accessibility Props

| Property           | Type           | Description                                     | Default  |
| ------------------ | -------------- | ----------------------------------------------- | -------- |
| `aria_label`       | `&'static str` | Label for accessibility.                        | `""`     |
| `aria_required`    | `&'static str` | Accessibility hint for required status.         | `"true"` |
| `aria_invalid`     | `&'static str` | Accessibility hint for invalid input.           | `"true"` |
| `aria_describedby` | `&'static str` | Links the input to a description (e.g., error). | `""`     |

## 💡 Notes

- The `Input` component can be used for various input types like text, password, etc.
- You can bind the component to state hooks for two-way data binding.
- Utilize `validate_function` to validate user input and display error messages.
- The `eye_active` and `eye_disabled` props allow for password visibility toggling with FontAwesome icons.
- Customize the appearance with CSS classes for better integration into your app's design.
