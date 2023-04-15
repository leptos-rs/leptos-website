use crate::{app::DarkModeContext, components::SphereLogo::*};
use leptos::*;
use leptos_router::*;

#[component]
pub fn HeroHeader(cx: Scope) -> impl IntoView {
    let (hamburger_menu_open, set_hamburger_menu_open) = create_signal(cx, false);

    let set_dark_mode = use_context::<DarkModeContext>(cx).unwrap().set_dark_mode;
    let dark_mode = use_context::<DarkModeContext>(cx).unwrap().dark_mode;

    view! { cx,
        <div class="max-w-[1920px] mx-auto relative">
            <div class="relative bg-no-repeat bg-center bg-[length:100%_100%] w-[calc(100%+25px)]  h-full top-0 4xl:top-[-10px] left-[-15px] 4xl:left-0  bg-[url('/images/wave_small.svg')] lg:bg-[url('/images/wave.svg')] drop-shadow-[10px_10px_0px_#190E3825]">
                <div class="pl-6 pr-4 md:pl-8 md:pr-6 lg:pl-12 lg:pr-8 pt-2 md:pt-4 lg:pt-8 text-white  w-full ">
                    <div class="flex w-full items-center justify-between max-w-4xl mx-auto">
                        <A href="/">
                            <img class="block w-[90px]  sm:w-28" src="/images/header_logo.svg"/>
                        </A>
                        {move || match hamburger_menu_open() {
                            true => {
                                view! { cx, <></> }
                            }
                            false => {
                                view! { cx,
                                    <>
                                        <div class="hidden lg:block">
                                            <div class="flex gap-1 xl:gap-4 justify-center items-center">
                                                <A
                                                    href="/"
                                                    exact=true
                                                    class="font-bold transition-all  text-white  aria-[current]:bg-purple  hover:bg-purple hover:bg-opacity-50 px-4 py-2 rounded-md "
                                                >
                                                    "Home"
                                                </A>
                                                <a
                                                    href="https://leptos-rs.github.io/leptos/"
                                                    exact=true
                                                    class="font-bold transition-all  text-white  aria-[current]:bg-purple  hover:bg-purple hover:bg-opacity-50 px-4 py-2 rounded-md "
                                                >
                                                    "Docs"
                                                </a>
                                                <a
                                                    href="https://codesandbox.io/p/sandbox/16-router-forked-hrrt3h?file=%2Fsrc%2Fmain.rs"
                                                    exact=true
                                                    class="font-bold transition-all  text-white  aria-[current]:bg-purple  hover:bg-purple hover:bg-opacity-50 px-4 py-2 rounded-md "
                                                >
                                                    "Playground"
                                                </a>
                                                <a
                                                    href="https://docs.rs/leptos/latest/leptos/"
                                                    exact=true
                                                    class="font-bold transition-all  text-white  aria-[current]:bg-purple  hover:bg-purple hover:bg-opacity-50 px-4 py-2 rounded-md "
                                                >
                                                    "API"
                                                </a>
                                            </div>
                                        </div>
                                    </>
                                }
                            }
                        }}
                        <div>
                            <div class="flex gap-5 md:gap-8 justify-center items-center">
                                <a href="https://discord.gg/x8NhWWYTV2">
                                    <img class="block h-6 w-6" src="/images/discord_logo.svg"/>
                                </a>
                                <a href="https://github.com/leptos-rs/leptos">
                                    <img class="block h-6 w-6" src="/images/github_logo.svg"/>
                                </a>
                                <button on:click=move |_| set_dark_mode.update(|n| *n = !*n)>
                                    <img
                                        class=" h-6 w-6 block"
                                        src={
                                            move || match dark_mode() {
                                                true => "/images/sun.svg",
                                                false => "/images/moon.svg",
                                            }
                                        }
                                    />
                                </button>
                                <button on:click=move |_| set_hamburger_menu_open.update(|n| *n = !*n)>
                                    <img
                                        class=" h-6 w-6 block lg:hidden"
                                        src={
                                            move || match hamburger_menu_open() {
                                                true => "/images/x_close.svg",
                                                false => "/images/mobile_menu.svg",
                                            }
                                        }
                                    />
                                </button>
                            </div>
                        </div>
                    </div>
                    {move || match hamburger_menu_open() {
                        true => {
                            view! { cx,
                                <>
                                    <div class="max-w-4xl mx-auto mt-8 pb-16 lg:hidden">
                                        <div class="flex flex-col gap-2 ">
                                            <A
                                                href="/"
                                                exact=true
                                                class="font-bold transition-all  text-white  aria-[current]:bg-purple  hover:bg-purple hover:bg-opacity-50 px-4 py-2 rounded-md "
                                            >
                                                "Home"
                                            </A>
                                            <a
                                                href="https://leptos-rs.github.io/leptos/"
                                                exact=true
                                                class="font-bold transition-all  text-white  aria-[current]:bg-purple  hover:bg-purple hover:bg-opacity-50 px-4 py-2 rounded-md "
                                            >
                                                "Documentation"
                                            </a>
                                            <a
                                                href="https://codesandbox.io/p/sandbox/16-router-forked-hrrt3h?file=%2Fsrc%2Fmain.rs"
                                                exact=true
                                                class="font-bold transition-all  text-white  aria-[current]:bg-purple  hover:bg-purple hover:bg-opacity-50 px-4 py-2 rounded-md "
                                            >
                                                "Playground"
                                            </a>
                                            <a
                                                href="https://docs.rs/leptos/latest/leptos/"
                                                exact=true
                                                class="font-bold transition-all  text-white  aria-[current]:bg-purple  hover:bg-purple hover:bg-opacity-50 px-4 py-2 rounded-md "
                                            >
                                                "API"
                                            </a>
                                        </div>
                                    </div>
                                </>
                            }
                        }
                        false => {
                            view! { cx, <></> }
                        }
                    }}
                    {move || match hamburger_menu_open() {
                        true => {
                            view! { cx, <></> }
                        }
                        false => {
                            view! { cx,
                                <>
                                    <div class="max-w-4xl mx-auto relative  ">
                                        <div class="flex gap-12 justify-start xl:justify-between items-center pt-12 pb-24 md:py-24">
                                            <div class="md:mt-[-60px] lg:mt-[-60px]">
                                                <h1 class="font-bold text-4xl lg:text-5xl tracking-tight">
                                                    "Full stack, fully typed."
                                                </h1>
                                                <p class="mt-2 opacity-90 max-w-[40ch] ">
                                                    "A cutting-edge Rust framework for the modern web."
                                                </p>
                                                <div class="mt-4 flex gap-3">
                                                    <a
                                                        href="https://www.youtube.com/watch?v=AD3FHodVgE8"
                                                        class="font-semibold text-lg py-2 px-4 text-purple bg-light_blue rounded-md shadow-[3px_3px_0px_#5e7a7b50]  hover:saturate-200 transition-all"
                                                    >
                                                        "Video"
                                                    </a>
                                                    <a
                                                        href="https://leptos-rs.github.io/leptos/"
                                                        class="font-semibold text-lg py-2 px-4 text-purple bg-beige rounded-md shadow-[3px_3px_0px_#7e816e50] hover:saturate-200 transition-all"
                                                    >
                                                        "Documentation"
                                                    </a>
                                                </div>
                                            </div>
                                            <div class="w-[40%]"></div>
                                            <div class="max-w-[320px] md:max-w-[360px] lg:max-w-[400px]  hidden md:block w-full h-full absolute right-0 top-4 aspect-square">
                                                <SphereLogo/>
                                            </div>
                                        </div>
                                    </div>
                                </>
                            }
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
