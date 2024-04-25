use crate::{
    app::{use_current_user, ArticleSlugParam, FavoriteButton, FollowButton, TagList, NBSP},
    error_template::error_boundary_fallback,
    models::{article::Article, comment::Comment}, pages::profile::{profile_link, ProfileImg},
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Article() -> impl IntoView {
    let user = use_current_user();
    let params = use_params::<ArticleSlugParam>();
    let slug = Signal::derive(move || params().map(|p| p.slug).unwrap_or_default());
    let article = create_blocking_resource(
        move || (slug(), user().map(|u| u.username)),
        |(slug, user)| get_article(slug, user),
    );
    view! {
        <div class="article-page">
            <Transition fallback=|| "Loading article...">
                <ErrorBoundary fallback=error_boundary_fallback>
                    {move || {
                        article().map(|res| res.map(|article| view! { <ArticleContent article/> }))
                    }}

                </ErrorBoundary>
            </Transition>
            <div class="row">
                <Comments article_slug=slug/>
            </div>
        </div>
    }
}

#[component]
pub fn Preview(#[prop(into)] article: RwSignal<Article>) -> impl IntoView {
    let article_link = move || article.with(|a| format!("/article/{}", a.slug));
    view! {
        <div class="article-preview">
            <ArticleMeta article=article>
                <FavoriteButton article=article compact=true/>
            </ArticleMeta>
            <A href=article_link class="preview-link">
                <h1>{move || article.with(|a| a.title.clone())}</h1>
                <p>{move || article.with(|a| a.description.clone())}</p>
                <span>Read more...</span>
                <TagList outline=true tags=move || article.with(|a| a.tags.clone())/>
            </A>
        </div>
    }
}

fn format_date(date: &str) -> String {
    date.to_string()
}

#[component]
fn ArticleMeta(#[prop(into)] article: Signal<Article>, children: Children) -> impl IntoView {
    let author = Signal::derive(move || article.with(|a| a.author.clone()));
    let author_link = move || author.with(|a| profile_link(&a.username));
    let image = Signal::derive(move || author.with(|p| p.image.clone()));
    view! {
        <div
            class="article-meta"
            style="display: flex; flex-direction: row; justify-content: center; gap: 5px"
        >
            <A href=author_link>
                <ProfileImg src=image/>
            </A>
            <div class="info">
                <A href=author_link class="author">
                    {move || author.with(|a| a.username.clone())}
                </A>
                <span class="date">{move || article.with(|a| format_date(&a.created_at))}</span>
                <span class="date">
                    {move || {
                        article.with(|a| a.updated_at.as_ref().map(|updated| format_date(updated)))
                    }}

                </span>
            </div>
            {children()}
        </div>
    }
}

#[server]
async fn get_article(slug: String, user: Option<String>) -> Result<Article, ServerFnError> {
    Article::get(&slug, user.as_deref()).await.map_err(|e| {
        tracing::error!("could not get article: {:?}", e);
        ServerFnError::ServerError("Failed to get article".into())
    })
}

#[server]
async fn delete_article(slug: String) -> Result<(), ServerFnError> {
    let author = crate::auth::require_login()?;
    sqlx::query!(
        "delete from article where slug = ? and author = ?",
        slug,
        author
    )
    .execute(crate::db::get())
    .await
    .map(|_| {
        // TODO: could go back to previous page
        leptos_axum::redirect("/");
    })
    .map_err(|e| {
        tracing::error!("failed to delete article: {:?}", e);
        ServerFnError::ServerError("Failed to delete article".into())
    })
}

#[component]
fn ArticleActions(#[prop(into)] article: RwSignal<Article>) -> impl IntoView {
    let user = use_current_user();
    let is_logged_in = move || user.with(Option::is_some);
    let author = Signal::derive(move || article.with(|a| a.author.username.clone()));
    let is_author = Signal::derive(move || {
        user.with(|user| user.as_ref().is_some_and(|user| user.username == author()))
    });
    let profile = create_slice(article, |a| a.author.clone(), |a, new| a.author = new);
    let delete = create_server_action::<DeleteArticle>();

    view! {
        <ArticleMeta article=article>
            <Show
                when=is_author
                fallback=move || {
                    view! {
                        <Show when=is_logged_in>
                            <FollowButton profile=profile/>
                        </Show>
                        <FavoriteButton article=article/>
                    }
                }
            >

                <div>
                    <A
                        href=move || article.with(|a| format!("/editor/{}", a.slug))
                        class="btn btn-sm btn-outline-secondary"
                    >
                        <i class="ion-edit"></i>
                        Edit Article
                    </A>
                </div>
                <ActionForm action=delete>
                    <input
                        type="hidden"
                        name="slug"
                        value=move || article.with(|a| a.slug.clone())
                    />
                    <button
                        type="submit"
                        disabled=delete.pending()
                        class="btn btn-sm btn-outline-danger"
                    >
                        <i class="ion-trash-a"></i>
                        Delete Article
                    </button>
                </ActionForm>
            </Show>
        </ArticleMeta>
    }
}

#[component]
fn ArticleContent(article: Article) -> impl IntoView {
    let article = create_rw_signal(article);
    view! {
        <div class="banner">
            <div class="container">
                <h1>{move || article.with(|a| a.title.clone())}</h1>
                <ArticleActions article/>
            </div>
        </div>

        <div class="container page">
            <div class="row article-content">
                <div class="col-md-12">
                    // TODO: This is a bit of a hack, but let's roll with it for now
                    <div id="content" style="all: initial">
                        <pre>{move || article.with(|a| a.body.clone())}</pre>
                        <div></div>
                    </div>
                    // <script type="module">
                    // "
                    // import { marked } from 'https://cdn.jsdelivr.net/npm/marked/lib/marked.esm.js';
                    // import DOMPurify from 'https://cdn.jsdelivr.net/npm/dompurify@3.1.0/+esm'
                    // const [pre, target] = document.getElementById('content').children;
                    // pre.style.display = 'none';
                    // target.innerHTML = DOMPurify.sanitize(marked.parse(pre.textContent));
                    // "
                    // </script>
                    <TagList outline=true tags=move || article.with(|a| a.tags.clone())/>
                </div>
            </div>

            <hr/>

            <div class="article-actions">
                <ArticleActions article/>
            </div>
        </div>
    }
}

#[server]
async fn comments(slug: String) -> Result<Vec<Comment>, ServerFnError> {
    Comment::for_article(&slug).await.map_err(|e| {
        tracing::error!("failed to fetch comments: {:?}", e);
        ServerFnError::ServerError("Failed to fetch comments".into())
    })
}

#[component]
fn CommentCard(comment: Comment) -> impl IntoView {
    let author = comment.author.clone();
    let link = profile_link(&author.username);
    view! {
        <div class="card">
            <div class="card-block">
                <p class="card-text">{&comment.body}</p>
            </div>
            <div class="card-footer">
                <A href=link.clone() class="comment-author">
                    <ProfileImg src=author.image class="comment-author-img"/>
                </A>
                {NBSP}
                <A href=link class="comment-author">
                    {&author.username}
                </A>
                <span class="date-posted">{&comment.created_at}</span>
            </div>
        </div>
    }
}

#[component]
fn Comments(#[prop(into)] article_slug: MaybeSignal<String>) -> impl IntoView {
    let comments = create_resource(article_slug, comments);
    view! {
        <div class="col-xs-12 col-md-8 offset-md-2">
            <form class="card comment-form">
                <div class="card-block">
                    <textarea
                        class="form-control"
                        placeholder="Write a comment..."
                        rows="3"
                    ></textarea>
                </div>
                <div class="card-footer">
                    <ProfileImg src=None class="comment-author-img"/>
                    <button class="btn btn-sm btn-primary">Post Comment</button>
                </div>
            </form>

            // TODO: Maybe try `Transition`
            <Suspense fallback=move || "Loading comments...">
                <ErrorBoundary fallback=error_boundary_fallback>
                    {move || {
                        comments()
                            .map(|data| {
                                data.map(|comments| {
                                    view! {
                                        <For
                                            each=move || comments.clone()
                                            key=|comment| comment.id
                                            let:comment
                                        >
                                            <CommentCard comment=comment/>
                                        </For>
                                    }
                                })
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
