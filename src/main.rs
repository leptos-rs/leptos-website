use cfg_if::cfg_if;

// boilerplate to run in different modes
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use axum::{
            routing::{post, },
            extract::{Extension, },

            Router,
        };
        use leptos_website::app::*;
        use leptos_website::fallback::file_and_error_handler;
        use leptos_axum::{generate_route_list, LeptosRoutes};
        use std::sync::Arc;
        use leptos_website::pages::Home::PerformMarkdownCodeToHtml;
        use leptos_website::components::ExampleServerFunction::SaveFavorites;
        use tower_http::{compression::CompressionLayer};

        #[tokio::main]
        async fn main() {
            simple_logger::init_with_level(log::Level::Warn).expect("couldn't initialize logging");

            _ = PerformMarkdownCodeToHtml::register();
            _ = SaveFavorites::register();

            /* sqlx::migrate!()
                .run(&mut conn)
                .await
                .expect("could not run SQLx migrations"); */

            // Setting this to None means we'll be using cargo-leptos and its env vars
            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

            // build our application with a route
            let app = Router::new()
            .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
            .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> } )
            .fallback(file_and_error_handler)
            .layer(Extension(Arc::new(leptos_options)))
            .layer(CompressionLayer::new());

            // run our app with hyper
            // `axum::Server` is a re-export of `hyper::Server`
            log!("listening on http://{}", &addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }
}
