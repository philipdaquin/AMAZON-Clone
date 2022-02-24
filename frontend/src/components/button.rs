use {
    yew::{html, Callback, MouseEvent, Properties},
};

use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Normal,
    Plain,
    Warning,
    Danger,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or(ButtonVariant::Normal)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub icon_name: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub style: &'static str,
}

#[function_component(Button)]
pub fn button(
    ButtonProps {
        variant,
        icon_name,
        label,
        onclick,
        disabled,
        style,
    }: &ButtonProps,
) -> Html {
    let variant = match &variant {
        ButtonVariant::Normal => style,
        ButtonVariant::Plain => "background: rgb(221, 221, 221); color: #0d0d0d",
        ButtonVariant::Warning => "background: yellow",
        ButtonVariant::Danger => "background: #b30000",
    };

    html! {
        <button style={variant} onclick={onclick} class="button" disabled={*disabled}>
            <div style="display: flex; flex: 1; justify-content: center; align-items: center;">
                {if icon_name.is_empty() {
                    html! {}
                } else {
                    html! {
                        <div style="margin-right: 4px;">
                            <i class={icon_name} aria-hidden="true"></i>
                        </div>
                    }
                }}
                {label}
            </div>
        </button>
    }
}
