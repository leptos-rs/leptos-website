use leptos::*;

struct SpeedStat {
    name: String,
    color: String,
    primary: bool,
    percentage: u8,
}

#[component]
pub fn PercentageBar(
    cx: Scope,
    color: String,
    percentage: u8,
    #[prop(default = false)] primary: bool,
    tech_name: String,
) -> impl IntoView {
    let primary_style = if primary {
        "background: linear-gradient(138deg, rgba(38,16,78,1) 0%, rgba(251,50,50,1) 100%);"
    } else {
        ""
    };

    let primary_class = if primary { "text-bold" } else { "" };

    view! { cx,
        <div class="text-eggshell flex flex-col lg:items-center lg:flex-row">
            <div class="lg:w-[5rem] mb-2 lg:hidden text-xl flex lg:justify-end lg:px-4">
                {tech_name}
            </div>
            <div
                style=format!("border-color: {}40;", color)
                class=format!("w-full h-10 mb-4 lg:my-3 relative rounded-md border-2 border-solid")
            >
                <div
                    style=format!("width: {}%; outline-color: {}; {}", percentage, color, primary_style)
                    class=format!(
                        "h-full absolute top-0 left-0 rounded-md outline-4 outline flex items-center justify-end px-2 {}",
                        primary_class
                    )
                >
                    <span class="relative z-[2]">{percentage} "%"</span>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Label(
    cx: Scope,
    tech_name: String,
    #[prop(default = false)] primary: bool,
) -> impl IntoView {
    let bold_class = if primary { "font-bold" } else { "" };

    view! { cx,
        <div class="text-purple flex flex-col lg:items-center lg:flex-row dark:text-eggshell">
            <div class="w-full h-10 mb-4 lg:my-3 relative"></div>
            <div class=format!("lg:w-[5rem] text-xl flex lg:justify-end lg:px-4 {}", bold_class)>
                {tech_name}
            </div>
        </div>
    }
}

#[component]
pub fn SpeedStats(cx: Scope, shadow: bool, border: bool) -> impl IntoView {
    let shadow_class = if shadow {
        "shadow-[10px_10px_0px_#190E3825]"
    } else {
        ""
    };

    let border_class = if border { "border" } else { "" };

    let labels = vec![
        SpeedStat {
            name: String::from("VanillaJS"),
            color: String::from("#A5D6A7"),
            primary: false,
            percentage: 100,
        },
        SpeedStat {
            name: String::from("Leptos"),
            color: String::from("#ED3135"),
            primary: true,
            percentage: 89,
        },
        SpeedStat {
            name: String::from("Vue"),
            color: String::from("#F0ADA8"),
            primary: false,
            percentage: 82,
        },
        SpeedStat {
            name: String::from("Svelte"),
            color: String::from("#D2D7B4"),
            primary: false,
            percentage: 76,
        },
        SpeedStat {
            name: String::from("React"),
            color: String::from("#A8DADC"),
            primary: false,
            percentage: 22,
        },
    ];

    view! { cx,
        <div class="flex max-w-4xl mx-auto lg:px-4">
            <div class=format!("hidden lg:block h-full pr-2 py-4 mx-auto",)>
                {labels
                    .iter()
                    .map(|row| {
                        view! { cx, <Label tech_name=row.name.clone() primary=row.primary/> }
                    })
                    .collect::<Vec<_>>()}
            </div>
            <div class=format!(
                "w-full h-full px-4 py-4 bg-gradient-to-tr from-[#181139] to-[#324571] rounded-md mx-auto {} {}",
                shadow_class, border_class
            )>
                {labels
                    .iter()
                    .map(|row| {
                        view! { cx,
                            <PercentageBar
                                tech_name=row.name.clone()
                                color=row.color.clone()
                                primary=row.primary
                                percentage=row.percentage
                            />
                        }
                    })
                    .collect::<Vec<_>>()}
                <p class="text-[#F2F8FA70] mt-4">
                    "Source: "
                    <a href="https://krausest.github.io/js-framework-benchmark/2023/table_chrome_112.0.5615.49.html">
                        <code>"js-framework-benchmark"</code> " official results for Chrome 112."
                    </a>
                </p>
            </div>
        </div>
    }
}
