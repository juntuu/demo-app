use crate::{
    app::{use_current_user, ArticleSlugParam, FollowButton, TagList, NBSP},
    error_template::error_boundary_fallback,
    models::{article::Article, comment::Comment},
    pages::profile::{profile_link, ProfileImg},
};
use leptos::*;
use leptos_meta::Script;
use leptos_router::*;

#[component]
pub fn Article() -> impl IntoView {
    let params = use_params::<ArticleSlugParam>();
    let slug = Signal::derive(move || params().map(|p| p.slug).unwrap_or_default());
    let article = create_blocking_resource(slug, get_article);

    // Inject script to head for the markdown renderer component
    view! {
        <Script
            type_="module"
            src="https://cdn.jsdelivr.net/gh/zerodevx/zero-md@2/dist/zero-md.min.js"
        />
        <div class="article-page">
            <Suspense fallback=|| "Loading article...">
                <ErrorBoundary fallback=error_boundary_fallback>
                    {move || {
                        article().map(|res| res.map(|article| view! { <ArticleContent article/> }))
                    }}

                </ErrorBoundary>
            </Suspense>
            <div class="row">
                <Comments article_slug=slug/>
            </div>
        </div>
    }
}

#[component]
pub fn ArticlePreview(#[prop(into)] article: RwSignal<Article>) -> impl IntoView {
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
            </A>
            <TagList outline=true tags=move || article.with(|a| a.tags.clone())/>
        </div>
    }
}

#[server]
#[cfg_attr(feature = "ssr", tracing::instrument)]
async fn toggle_favorite(article: String, current: bool) -> Result<bool, ServerFnError> {
    let logged_in = crate::auth::require_login()?;
    if sqlx::query_scalar!(
        "select author = ? from article where slug = ?",
        logged_in,
        article
    )
    .fetch_optional(crate::db::get())
    .await?
    .unwrap_or(1)
        != 0
    {
        // Can't favorite own article
        tracing::debug!("own article");
        return Ok(false);
    }
    if current {
        sqlx::query!(
            "delete from favorite where user = ? and article = ?",
            logged_in,
            article
        )
    } else {
        sqlx::query!(
            "insert or ignore into favorite (user, article) values (?, ?)",
            logged_in,
            article
        )
    }
    .execute(crate::db::get())
    .await
    .map(|res| {
        tracing::trace!("result: {:?}", res);
        res.rows_affected() == 1
    })
    .map_err(|e| {
        tracing::error!("failed to toggle follow: {:?}", e);
        ServerFnError::ServerError("database error".into())
    })
}

#[component]
fn FavoriteButton(article: RwSignal<Article>, #[prop(optional)] compact: bool) -> impl IntoView {
    let user = use_current_user();
    let toggle = create_server_action::<ToggleFavorite>();
    let pending = toggle.pending();
    let result = toggle.value();
    let disabled = move || {
        with!(|user, article| {
            user.as_ref()
                .map_or(true, |user| user.username == article.author.username)
                || pending()
        })
    };
    let favorited = move || article.with(|a| a.favorited);

    create_effect(move |_| {
        let success = result.with(|res| matches!(res, Some(Ok(true))));
        if success {
            article.update(|a| {
                if a.favorited {
                    a.favorited = false;
                    a.favorites_count -= 1;
                } else {
                    a.favorited = true;
                    a.favorites_count += 1;
                }
            });
        }
    });

    let text = if compact { "" } else { "Favorite article" };

    view! {
        <ActionForm action=toggle>
            <button type="submit" disabled=disabled class="btn btn-sm btn-outline-primary">
                <i class="ion-heart"></i>
                {NBSP}
                {text}
                <span class="counter">"(" {move || article.with(|a| a.favorites_count)} ")"</span>
            </button>
            <input type="hidden" name="article" value=move || article.with(|a| a.slug.clone())/>
            <input type="hidden" name="current" value=move || favorited().to_string()/>
        </ActionForm>
    }
}

#[component]
fn ArticleMeta(#[prop(into)] article: Signal<Article>, children: Children) -> impl IntoView {
    let author = Signal::derive(move || article.with(|a| a.author.clone()));
    let author_link = move || author.with(|a| profile_link(&a.username));
    view! {
        <div
            class="article-meta"
            style="display: flex; flex-direction: row; justify-content: center; gap: 5px"
        >
            <A href=author_link>
                {move || author.with(|p| view! { <ProfileImg src=p.image.clone()/> })}
            </A>
            <div class="info">
                <A href=author_link class="author">
                    {move || author.with(|a| a.username.clone())}
                </A>
                <span class="date">{move || article.with(|a| a.created_at.clone())}</span>
                <span class="date">
                    {move || {
                        article.with(|a| a.updated_at.as_ref().map(|date| format!("({})", date)))
                    }}

                </span>
            </div>
            {children()}
        </div>
    }
}

#[server]
async fn get_article(slug: String) -> Result<Article, ServerFnError> {
    tracing::info!("fetching article: {}", slug);
    let user = crate::auth::authenticated_username();
    Ok(Article::get(&slug, user.as_deref()).await?)
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
    .await?;
    // TODO: could go back to previous page
    leptos_axum::redirect("/");
    Ok(())
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
    // The body is not affected by ArticleActions
    let body = article.body.clone();
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
                    // A bit of a hack to reset styles
                    <div style="all: initial">
                        <noscript>
                            <pre>{&body}</pre>
                        </noscript>
                        <zero-md>
                            <script type="text/markdown">{&body}</script>
                        </zero-md>
                    </div>
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
    Ok(Comment::for_article(&slug).await?)
}

#[server]
async fn delete_comment(id: i64) -> Result<(), ServerFnError> {
    let author = crate::auth::require_login()?;
    sqlx::query!("delete from comment where id = ? and user = ?", id, author)
        .execute(crate::db::get())
        .await?;
    Ok(())
}

#[component]
fn CommentCard(comment: Comment, children: Children) -> impl IntoView {
    let author = comment.author.clone();
    let link = profile_link(&author.username);
    view! {
        <div class="card">
            <div class="card-block">
                <p class="card-text">{&comment.body}</p>
            </div>
            <div
                class="card-footer"
                style="display: flex; flex-direction: row; justify-content: center; gap: 5px"
            >
                <A href=link.clone() class="comment-author">
                    <ProfileImg src=author.image class="comment-author-img"/>
                </A>
                <A href=link class="comment-author">
                    {&author.username}
                </A>
                <span class="date-posted">{&comment.created_at}</span>
                {children()}
            </div>
        </div>
    }
}

#[server]
async fn post_comment(article: String, comment: String) -> Result<i64, ServerFnError> {
    let user = crate::auth::require_login()?;
    Ok(Comment::create(&article, &user, &comment).await?)
}

#[component]
fn Comments(#[prop(into)] article_slug: Signal<String>) -> impl IntoView {
    let user = use_current_user();
    let delete = create_server_action::<DeleteComment>();
    let post = create_server_action::<PostComment>();
    let post_result = post.value();

    let comments = create_resource(
        move || (article_slug(), post.version()(), delete.version()()),
        |(slug, _, _)| comments(slug),
    );

    let comment_ref: NodeRef<html::Textarea> = create_node_ref();

    create_effect(move |_| {
        match post_result() {
            Some(Ok(_)) => {
                // Clear the comment field after succesfull post
                comment_ref()
                    .expect("<textarea> should be mounted")
                    .set_value("");
            }
            Some(Err(e)) => {
                // Or show error after failed one (this doesn't work without js)
                comment_ref()
                    .expect("<textarea> should be mounted")
                    .set_custom_validity(&e.to_string());
            }
            _ => {}
        }
    });

    let delete_button = move |id: i64| {
        // Now a single action is shared between all comments, and thus
        // all buttons will be disabled while one delete is pending.
        //
        // This is just fine.
        view! {
            <ActionForm action=delete>
                <input type="hidden" name="id" value=id/>
                <button
                    type="submit"
                    disabled=delete.pending()
                    class="btn btn-sm btn-outline-danger"
                >
                    <i class="ion-trash-a"></i>
                </button>
            </ActionForm>
        }
    };

    // TODO: maybe "subscribe" for new comments and update real time
    let comment_list = move || {
        comments().map(|data| {
            data.map(|comments| {
                let user = user.with(|u| u.as_ref().map(|u| u.username.clone()));
                comments
                    .into_iter()
                    .map(|comment| {
                        let id = comment.id;
                        if user.as_deref() == Some(&comment.author.username) {
                            view! { <CommentCard comment=comment>{delete_button(id)}</CommentCard> }
                        } else {
                            view! { <CommentCard comment=comment>""</CommentCard> }
                        }
                    })
                    .collect_view()
            })
        })
    };

    view! {
        <div class="col-xs-12 col-md-8 offset-md-2">
            <ActionForm class="card comment-form" action=post>
                <input type="hidden" name="article" value=article_slug/>
                <div class="card-block">
                    <textarea
                        node_ref=comment_ref
                        class="form-control"
                        placeholder="Write a comment..."
                        rows="3"
                        name="comment"
                    ></textarea>
                </div>
                <div class="card-footer">
                    <ProfileImg src=None class="comment-author-img"/>
                    <button type="submit" class="btn btn-sm btn-primary">
                        Post Comment
                    </button>
                </div>
            </ActionForm>
            <Transition fallback=move || "Loading comments...">
                <ErrorBoundary fallback=error_boundary_fallback>{comment_list}</ErrorBoundary>
            </Transition>
        </div>
    }
}
