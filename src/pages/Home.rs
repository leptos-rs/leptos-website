use crate::components::CodeExample::*;
use crate::components::ExampleServerFunction::*;
use crate::components::ExampleTailwind::*;
use crate::components::FeatureList::*;
use crate::components::HeroHeader::*;
use crate::components::InteractiveCodeExample::*;
use crate::components::Page::*;
use crate::components::SpeedStats::*;
use leptos::*;
use leptos_meta::*;

#[server(PerformMarkdownCodeToHtml, "/api", "GetJSON")]
pub async fn perform_markdown_code_to_html(markdown: String) -> Result<String, ServerFnError> {
    use femark::{process_markdown_to_html, HTMLOutput};

    match process_markdown_to_html(markdown) {
        Ok(HTMLOutput { content, toc: _ }) => Ok(content),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let feature_list_items: Vec<String> = vec![
        String::from("Create full-stack apps that start working immediately and are progressively enhanced with client-side interactivity."),
        String::from("Start projects fast using simple tooling with minimal configuration."),
        String::from("Easily manage state without fighting the borrow checker with reactive signals."),
        String::from("Write “server functions” that work across both the server and client."),
        String::from("Sleep well knowing Rust's type safety is protecting your whole app."),
    ];

    view! { cx,
        <Title text="Home"/>
        <Page>
            <HeroHeader/>
            <div class="mt-12 lg:mt-20 mb-8    px-4  max-w-[1920px] mx-auto flex flex-col gap-8 justify-center lg:flex-row md:gap-12">
                <div class="w-full lg:max-w-[45ch]">
                    <h2 class="font-bold text-2xl lg:text-4xl lg:max-w-[35ch] text-purple dark:text-eggshell">
                        "Sustainable, maintainable, and engaging"
                    </h2>
                    <p class="mt-4 lg:mx-auto text-lg font-medium  text-purple dark:text-eggshell">
                        "Experience the future of Rust web development with a framework that keeps you excited to create exceptional websites and applications."
                    </p>
                    <p class="mt-4 lg:mx-auto text-purple dark:text-eggshell">
                        "Leptos makes it easy to build applications in the most-loved programming language, combining the best paradigms of modern web development with the power of Rust."
                    </p>
                </div>
                <div class="w-full lg:max-w-[45ch]">
                    <FeatureList items=feature_list_items/>
                </div>
            </div>
            <div class="mt-12 lg:mt-24 mb-8    px-4  max-w-[1920px] mx-auto ">


                    <h2 class="max-w-4xl mx-auto font-bold text-2xl lg:text-4xl lg:text-center lg:max-w-[40ch] text-purple dark:text-eggshell">
                    "High-performance"
                </h2>
                <div class="mt-6">
                    <SpeedStats shadow={true} border={false}/>
                    </div>

            </div>
            <div class="mt-12 lg:mt-28 mb-8 px-4  max-w-[1920px] mx-auto ">
                <h2 class="max-w-4xl mx-auto font-bold text-2xl lg:text-4xl lg:text-center lg:max-w-[40ch] text-purple dark:text-eggshell">
                    "Your favorite UI patterns"
                </h2>
                <p class="max-w-[70ch] mt-4 lg:mx-auto lg:text-center  text-purple dark:text-eggshell">
                    "Build websites and apps from self-contained components with reactive state management that's easy to use."
                </p>
                <div class="mt-6">
                    <InteractiveCodeExample
                        shadow=true
                        border=true
                        background="bg-[#F3F4F6] dark:bg-black".to_string()
                    />
                </div>
                <p class="max-w-[70ch] mt-8 lg:mx-auto lg:text-center  text-purple dark:text-eggshell">
                    "Leptos's fine-grained reactive signals make targeted updates to the DOM when your component's state changes, keeping your app responsive to user input."
                </p>
            </div>
            <div class="mt-8 lg:my-32  bg-gradient-to-tr from-red via-purple to-black px-4 py-12 max-w-[1920px] mx-auto  4xl:rounded-md bg-white 4xl:shadow-[10px_10px_0px_#190E3825] ">
                <h2 class="max-w-4xl mx-auto font-bold text-2xl lg:text-4xl lg:text-center lg:max-w-[40ch] text-eggshell">
                    "Your favorite language"
                </h2>
                <p class="max-w-[70ch] mt-4  lg:mx-auto lg:text-center text-eggshell">
                    "Leptos makes it easy to integrate Rust backend code with your user interface in a few lines of code. "
                    <code>"#[server]"</code>
                    " functions let you cross the client-server boundary without the boilerplate of setting up a new API endpoint, making it easy to create “full-stack components” that let you write everything from a SQL query to a button click in one place."
                </p>
                <div class="mt-6">
                    <CodeExample
                        code=EXAMPLE_SERVER_FUNCTION_CODE.to_string()
                        shadow=false
                        border=false
                        background="bg-[#F3F4F6] dark:bg-black".to_string()
                    >
                        <ExampleServerFunction/>
                    </CodeExample>
                </div>
                <p class="max-w-[70ch] mt-8 mb-0 lg:mx-auto lg:text-center  text-eggshell">
                    "Leptos harnesses Rust's strong type safety, speed, and concurrency to deliver highly performant and reliable applications."
                </p>
            </div>
            <div class="mt-12 mb-8    px-4  max-w-[1920px] mx-auto ">
                <h2 class="max-w-4xl mx-auto font-bold text-2xl lg:text-4xl lg:text-center lg:max-w-[40ch] text-purple dark:text-eggshell">
                    "Your favorite tools"
                </h2>
                <p class="max-w-[70ch] mt-4 lg:mx-auto lg:text-center  text-purple dark:text-eggshell">
                    "Tools like Tailwind integrate with Leptos perfectly, letting you build on design patterns shared across the Web."
                </p>
                <div class="mt-6">
                    <CodeExample
                        code=EXAMPLE_TAILWIND_CODE.to_string()
                        shadow=true
                        border=true
                        background="bg-[#F3F4F6] dark:bg-black".to_string()
                    >
                        <ExampleTailwind/>
                    </CodeExample>
                </div>
                <p class="max-w-[70ch] mt-8 mb-12 lg:mx-auto lg:text-center  text-purple dark:text-eggshell">
                    "Boost your productivity with great tooling like hot-reloading template updates and a dedicated Leptos language server and VSCode extension, designed to streamline your Leptos coding experience and maintain a consistent code style throughout your project."
                </p>
                <div class="w-full flex justify-center pb-16">
                    <a
                        href="https://leptos-rs.github.io/leptos/"
                        class="font-semibold text-eggshell text-2xl px-8 py-5  bg-gradient-to-r from-dark_blue to-purple rounded-md shadow-[3px_3px_0px_#0d0b29] hover:saturate-200 transition-all"
                    >
                        "Get Started"
                    </a>
                </div>
            </div>
        </Page>
    }
}
