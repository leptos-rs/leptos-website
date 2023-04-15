use crate::components::CodeExample::*;
use crate::components::ExampleServerFunction::*;
use crate::components::ExampleTailwind::*;
use crate::components::FeatureList::*;
use crate::components::GettingStartedSquare::*;
use crate::components::HeroHeader::*;
use crate::components::InteractiveCodeExample::*;
use crate::components::Page::*;
use crate::components::SpeedStats::*;
use leptos::*;
use leptos_meta::*;

#[server(PerformMarkdownCodeToHtml, "/api")]
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
        String::from("Simple tooling with zero config"),
        String::from("Works with Javascript disabled"),
        String::from("Easily manage state without fighting the borrow checker"),
        String::from("Write “server functions” that work across both the server and client"),
        String::from("Amazing code completion across your whole application"),
    ];

    view! { cx,
        <Title text="Home"/>
        <Page>
            <HeroHeader/>

            <div class="mt-20 mb-8    px-4  max-w-[1920px] mx-auto flex flex-col gap-8 justify-center lg:flex-row md:gap-12">
                <div class="w-full lg:max-w-[45ch]">
                    <h2 class="font-bold text-2xl lg:text-4xl lg:max-w-[35ch] text-purple dark:text-eggshell">"Sustainable, maintainable, and engaging"</h2>
                    <p class="mt-4 lg:mx-auto text-lg font-medium  text-purple dark:text-eggshell">
                        "Experience the future of Rust web app development with a framework that keeps us - and you - excited to create exceptional digital experiences."
                    </p>
                    <p class="mt-4 lg:mx-auto text-purple dark:text-eggshell">
                        "Building applications in the most loved language using a framework gives developers a joyful experience, making Leptos apps fun to build and best of all, easy to maintain and grow!"
                    </p>
                </div>
                <div class="w-full lg:max-w-[45ch]">
                    <FeatureList items={feature_list_items} />
                </div>
            </div>

            <div class="mt-12 mb-8    px-4  max-w-[1920px] mx-auto ">
                <h2 class="max-w-4xl mx-auto font-bold text-2xl lg:text-4xl lg:text-center lg:max-w-[40ch] text-purple dark:text-eggshell">"High-performance"</h2>
                <p class="max-w-[70ch] mt-4 lg:mx-auto lg:text-center  text-purple dark:text-eggshell">
                    "Engineered for optimal performance, Leptos has earned its place among the top-performing frameworks."
                </p>
                <div class="mt-6">
                    <SpeedStats shadow={true} border={true} />
                </div>
            </div>

            <div class="mt-12 mb-8    px-4  max-w-[1920px] mx-auto ">
                <h2 class="max-w-4xl mx-auto font-bold text-2xl lg:text-4xl lg:text-center lg:max-w-[40ch] text-purple dark:text-eggshell">"Your favorite UI patterns"</h2>
                <p class="max-w-[70ch] mt-4 lg:mx-auto lg:text-center  text-purple dark:text-eggshell">
                    "Easily create and combine components to build websites and web-apps. You can even use Leptos to build UI for native applications."
                </p>
                <div class="mt-6">
                    <InteractiveCodeExample shadow={true} border={true} background="bg-[#F3F4F6] dark:bg-black".to_string()/>
                </div>
                <p class="max-w-[70ch] mt-8 mb-16 lg:mx-auto lg:text-center  text-purple dark:text-eggshell">
                    "Leptos' extensive library of UI patterns allows you to design unique and intuitive user interfaces, while its compatibility with native applications ensures you can create a cohesive brand experience across platforms."
                </p>
            </div>

            <div class="my-8 lg:my-12  bg-gradient-to-tr from-red via-purple to-black px-4 py-12 max-w-[1920px] mx-auto  4xl:rounded-md bg-white 4xl:shadow-[10px_10px_0px_#190E3825] ">
                <h2 class="max-w-4xl mx-auto font-bold text-2xl lg:text-4xl lg:text-center lg:max-w-[40ch] text-eggshell">"Your favorite language"</h2>
                <p class="max-w-[70ch] mt-4  lg:mx-auto lg:text-center text-eggshell">
                    "Leptos uses Rust's powerful macros to reduce boiler-plate, so you can focus on what matters. In this example, an isometric server function is generated for both the server and the client in one fell swoop."
                </p>
                <div class="mt-6">
                    <Transition fallback=|| ()>
                        <CodeExample code={EXAMPLE_SERVER_FUNCTION_CODE.to_string()} shadow={false} border={false} background="bg-[#F3F4F6] dark:bg-black".to_string()>
                            <ExampleServerFunction/>
                        </CodeExample>
                    </Transition>
                </div>
                <p class="max-w-[70ch] mt-8 mb-0 lg:mx-auto lg:text-center  text-eggshell">
                    "Leptos harnesses Rust's strong type safety, speed, and concurrency to deliver highly performant and reliable applications. Enjoy the best of both worlds with familiar UI patterns and the unparalleled performance of cutting-edge Rust technology."
                </p>
            </div>

            <div class="mt-12 mb-8    px-4  max-w-[1920px] mx-auto ">
                <h2 class="max-w-4xl mx-auto font-bold text-2xl lg:text-4xl lg:text-center lg:max-w-[40ch] text-purple dark:text-eggshell">"Your favorite tools"</h2>
                <p class="max-w-[70ch] mt-4 lg:mx-auto lg:text-center  text-purple dark:text-eggshell">
                    "Tools like Tailwind integrate with Leptos perfectly. Use all the best tools the web has to offer."
                </p>
                <div class="mt-6">
                    <Transition fallback=|| ()>
                        <CodeExample code={EXAMPLE_TAILWIND_CODE.to_string()} shadow={true} border={true} background="bg-[#F3F4F6] dark:bg-black".to_string()>
                            <ExampleTailwind/>
                        </CodeExample>
                    </Transition>
                </div>
                <p class="max-w-[70ch] mt-8 mb-12 lg:mx-auto lg:text-center  text-purple dark:text-eggshell">
                    "Boost your productivity with the dedicated VSCode code formatting extension, designed to streamline your Leptos coding experience and maintain a consistent code style throughout your project. Maximize your efficiency and build stunning projects that stand out with Leptos as your trusted partner in web development."
                </p>
            </div>

            <div id="getting-started" class="mt-12 mb-8 px-4 max-w-[1920px] mx-auto">
                <h2 class="max-w-4xl mx-auto font-bold text-2xl lg:text-4xl lg:text-center lg:max-w-[40ch] text-purple dark:text-eggshell">"Getting started"</h2>
                <div id="ssr-commands" class="mt-6 max-w-[70ch] mx-auto">
                    <h3 class="mt-4 lg:mx-auto text-xl font-medium  text-purple dark:text-eggshell">
                        "Want Server Side Rendering (SSR)?"
                    </h3>
                    <p class="max-w-[70ch] lg:mx-auto text-purple dark:text-eggshell">
                        "cargo-leptos is a build tool that's designed to make it easy to build apps that run on both the client and the server, with seamless integration."
                    </p>
                    <div class="my-2 bg-black dark:bg-purple w-full text-eggshell rounded-md p-3 overflow-scroll">
                        <pre class="text-[14px] lg:text-[16px]">
                            <p>"cargo install cargo-leptos"</p>
                            <p>"cargo leptos new --git https://github.com/leptos-rs/start"</p>
                            <p>"cd [your project name]"</p>
                            <p>"cargo leptos watch"</p>
                        </pre>
                    </div>
                    <p class="max-w-[70ch] lg:mx-auto text-purple dark:text-eggshell">
                        "Now open http://localhost:8000 in your browser to view the page."
                    </p>
                </div>
                <div id="csr-commands" class="mt-6 max-w-[70ch] mx-auto">
                    <h3 class="mt-4 lg:mx-auto text-xl font-medium  text-purple dark:text-eggshell">
                        "Want Client Side Rendering (CSR)?"
                    </h3>
                    <p class="max-w-[70ch] lg:mx-auto text-purple dark:text-eggshell">
                        "Trunk is a WASM web application bundler for Rust."
                    </p>
                    <div class="my-2 bg-black dark:bg-purple w-full text-eggshell rounded-md p-3 overflow-scroll">
                        <pre class="text-[14px] lg:text-[16px]">
                            <p>"cargo install trunk"</p>
                            <p>"cargo init leptos-tutorial"</p>
                            <p>"cargo add leptos"</p>
                            <p>"cd [your project name]"</p>
                            <p>"trunk serve --open"</p>
                        </pre>
                    </div>
                    <p class="max-w-[70ch] lg:mx-auto text-purple dark:text-eggshell">
                        "Now open http://localhost:8000 in your browser to view the page."
                    </p>
                </div>
                <div class="mt-12 max-w-4xl mx-auto grid gap-4 lg:grid-cols-3 grid-rows-1">
                    <GettingStartedSquare 
                        tag={String::from("Documentation")} 
                        title={String::from("Read the Documentation")}
                        body={String::from("View the official Leptos documentation")}
                        link={String::from("https://docs.rs/leptos/latest/leptos/")}
                        background_class={String::from("bg-gradient-to-tr from-[#EF3939] to-[#471069]")}
                    />
                    <GettingStartedSquare 
                        tag={String::from("Getting started")} 
                        title={String::from("Read the Introductory Guide")}
                        body={String::from("A beginner friendly guide to everything Leptos")}
                        link={String::from("https://leptos-rs.github.io/leptos/")}
                        background_class={String::from("bg-gradient-to-tl from-[#94D9DC] to-[#627AF7]")}
                    />
                    <GettingStartedSquare 
                        tag={String::from("Video series")} 
                        title={String::from("Watch the Video Series")}
                        body={String::from("Watch a video series by the creator of Leptos")}
                        link={String::from("https://youtube.com/playlist?list=PLg4xWRFolJIo0lZawIlEaYr180e1w-ZUH")}
                        background_class={String::from("bg-gradient-to-tr from-[#CDD698] to-[#5A986D]")}
                    />
                </div>
            </div>
        </Page>
    }
}
