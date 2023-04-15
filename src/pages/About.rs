use crate::components::Page::*;

use leptos::*;
use leptos_meta::*;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="About"/>
        <Page>
            <div class="mx-auto max-w-5xl px-4 h-full ">
                <header class="pt-4 lg:pt-12">
                    <h1 class="font-title text-navy text-3xl font-bold tracking-tight">"About"</h1>
                </header>
                <main class="pb-10 mb-auto mx-auto max-w-5xl text-navy pt-4">
                    <p class="max-w-[50ch]">
                        "Houski's primary goal is to organize the world's real estate data, and make it accessible to everyone."
                    </p>
                    <p class="text-navy mt-6 max-w-[50ch]">
                        "Houski's secondary objective is to improve the quality of real estate transactions."
                    </p>
                    <p class="text-navy mt-6 max-w-[50ch]">
                        "Houski is a startup created by the Wilkinson brothers."
                    </p>
                    <p class="text-navy mt-6 max-w-[50ch]">
                        "Our head office is located in Calgary, Alberta, Canada."
                    </p>
                </main>
                <div class="flex max-w-xl flex-grow h-full">
                    <div class=" max-w-xs ">
                        <img
                            src="/images/Alex_signature_navy.png"
                            class="block object-contain  mx-auto "
                        />
                        <div class="max-w-fit mx-auto">
                            <p class="md:text-xl text-navy   mt-2">"Alex Wilkinson"</p>
                            <p class="text-sm md:text-base text-navy block">"CEO, Houski Inc."</p>
                        </div>
                    </div>
                    <div class=" max-w-sm ">
                        <img
                            src="/images/Alex.png"
                            class="block object-contain brightness-[110%] saturate-[75%]"
                        />
                    </div>
                </div>
            </div>
        </Page>
    }
}
