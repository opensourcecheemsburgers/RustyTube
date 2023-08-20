pub use icons::*;

use leptos::{Children, component, IntoView, Scope, view};

#[component]
pub fn Svg(cx: Scope, children: Children, height: u8, width: u8) -> impl IntoView {
    use leptos::{Children, component, IntoView, Scope, view};
    use stylist::{Style, css};
    /// UI Icons from https://lucide.dev

    pub static RESPONSIVE_ICON_LG: &'static str = "h-5 w-5 lg:h-6 lg:w-6 2xl:h-7 2xl:w-7";
    pub static RESPONSIVE_ICON: &'static str = "h-4 w-4 lg:h-5 lg:w-5 2xl:h-6 2xl:w-6";

    const STROKE: &'static str = "currentColor";
    const STROKE_WIDTH: &'static str = "1.5";
    const BOX: &'static str = "0 0 24 24";
    const FILL: &'static str = "none";
    const LCAP: &'static str = "round";
    const LJOIN: &'static str = "round";

    view! {cx,
        <svg
            height={height}
            width={width}
            viewBox={BOX}
            fill={FILL}
            stroke={STROKE}
            stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP}
            stroke-linejoin={LJOIN}>
            {children(cx)}
        </svg>
    }
}

mod icons {
    use std::ops::Div;

    use leptos::{component, IntoView, Scope, view};
    use stylist::Style;
    use crate::icons::Svg;
    #[component]
    pub fn HamburgerIcon(cx: Scope) -> impl IntoView {
        view! {cx,
            <Svg height={24} width={24}>
                <line x1="4" x2="20" y1="12" y2="12"/>
                <line x1="4" x2="20" y1="6" y2="6"/>
                <line x1="4" x2="20" y1="18" y2="18"/>
            </Svg>
        }
    }

    #[component]
    pub fn FerrisIcon(cx: Scope) -> impl IntoView {
        view! {cx,
            <img class="h-10 w-10" src="ferris/cute.svg" />
        }
    }

    #[component]
    pub fn FerrisWaveIcon(cx: Scope) -> impl IntoView {
        view! {cx,
            <img class="h-30 w-30" src="ferris/wave.svg" />
        }
    }

    #[component]
    pub fn FerrisWtfIcon(cx: Scope, width: u8) -> impl IntoView {
        let width_style = format!(r#"width: {}rem"#, width.div(4));

        view! {cx,
            <img style=width_style src="ferris/wtf.svg" />
        }
    }


    #[component]
    pub fn PaletteIcon(cx: Scope) -> impl IntoView {
        view! {cx,
            <Svg height={24} width={24}>
                <circle cx="13.5" cy="6.5" r=".5"/>
                <circle cx="17.5" cy="10.5" r=".5"/>
                <circle cx="8.5" cy="7.5" r=".5"/>
                <circle cx="6.5" cy="12.5" r=".5"/>
                <path
                    d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z">
                </path>
            </Svg>
        }
    }

    #[component]
    pub fn ServerIcon(cx: Scope) -> impl IntoView {
        view! {cx,
            <Svg height={24} width={24}>
                <rect width="20" height="8" x="2" y="2" rx="2" ry="2"/>
                <rect width="20" height="8" x="2" y="14" rx="2" ry="2"/>
                <line x1="6" x2="6.01" y1="6" y2="6"/>
                <line x1="6" x2="6.01" y1="18" y2="18"/>
            </Svg>
        }
    }

}