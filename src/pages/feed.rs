use std::num::NonZeroU8;

use crate::{
    error_template::error_boundary_fallback, models::article::Feed, pages::article::ArticlePreview,
};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FeedKind {
    Feed,
    Global,
    By(String),
    Favorited(String),
    Tag(String),
}

#[component]
pub fn Feed(#[prop(into)] kind: MaybeSignal<FeedKind>, children: Children) -> impl IntoView {
    let query = use_query_map();
    let pagination = create_memo(move |_| {
        query.with(|m| {
            let offset = m
                .get("offset")
                .and_then(|s| s.parse().ok())
                .unwrap_or_default();
            let limit = m
                .get("limit")
                .and_then(|s| s.parse().ok())
                .and_then(NonZeroU8::new)
                .unwrap_or_else(|| NonZeroU8::new(10).unwrap());
            Page { offset, limit }
        })
    });
    let feed = create_blocking_resource(
        move || (kind(), pagination()),
        |(kind, page)| get_feed(kind, page),
    );
    let previews = move || {
        feed().map(|data| {
            data.map(|Feed { articles, count }| {
                view! {
                    <For
                        each=move || articles.clone()
                        key=|article| article.slug.clone()
                        let:article
                    >
                        <ArticlePreview article=create_rw_signal(article)/>
                    </For>
                    <Pagination page=pagination count=count/>
                }
            })
        })
    };

    view! {
        <div class="col-md-9">
            <div class="feed-toggle">
                <ul class="nav nav-pills outline-active">{children()}</ul>
            </div>

            <Suspense fallback=|| "Loading feed...">
                <ErrorBoundary fallback=error_boundary_fallback>{previews}</ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
fn Pagination(#[prop(into)] page: Signal<Page>, count: u32) -> impl IntoView {
    let page_links = move || {
        let Page { offset, limit } = page();
        let limit = u8::from(limit) as u32;
        (0..count)
            .step_by(limit as usize)
            .enumerate()
            .map(|(page, start)| {
                let class = if start <= offset && offset < start + limit {
                    "page-item active"
                } else {
                    "page-item"
                };
                view! {
                    <li class=class>
                        <A class="page-link" href=format!("?offset={}&limit={}", start, limit)>
                            {page + 1}
                        </A>
                    </li>
                }
            })
            .collect_view()
    };
    view! {
        <ul class="pagination">
        {page_links}
        </ul>
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
struct Page {
    offset: u32,
    limit: NonZeroU8,
}

#[server]
async fn get_feed(kind: FeedKind, page: Page) -> Result<Feed, ServerFnError> {
    use crate::models::article::FeedOptions;

    let options = FeedOptions {
        user: crate::auth::authenticated_username(),
        offset: page.offset,
        limit: page.limit.into(),
    };
    match kind {
        FeedKind::Feed => {
            let Some(user) = &options.user else {
                return Err(ServerFnError::ServerError("Not logged in".into()));
            };
            Feed::feed(user, &options).await
        }
        FeedKind::Global => Feed::global(&options).await,
        FeedKind::By(user) => Feed::by(&user, &options).await,
        FeedKind::Favorited(user) => Feed::favorited(&user, &options).await,
        FeedKind::Tag(tag) => Feed::tag(&tag, &options).await,
    }
    .map_err(|e| {
        tracing::error!("sql error when fetching feed: {:?}", e);
        ServerFnError::ServerError("Could not fetch feed".into())
    })
}
