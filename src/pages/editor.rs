#![allow(clippy::empty_docs)]

#[cfg(feature = "ssr")]
use crate::models::article::Article;

use crate::{
    app::ArticleSlugParam, error_template::error_boundary_fallback,
    models::article::ArticleEditFields,
};
use leptos::*;
use leptos_router::*;

#[server]
async fn get_article_for_editing(slug: String) -> Result<ArticleEditFields, ServerFnError> {
    let author = crate::auth::require_login()?;
    Article::for_editing(&slug, &author).await.map_err(|e| {
        tracing::error!("could not get article for editing: {:?}", e);
        ServerFnError::ServerError("could not get article for editing".into())
    })
}

// NOTE: the macro does some magic that doesn't understand plain typedef name,
// so the outer `Result` is not inlined here.
type CreateOrUpdateResult = Result<String, Vec<String>>;

#[server]
async fn create_or_update_post(
    slug: Option<String>,
    title: String,
    about: String,
    body: String,
    tags: String,
) -> Result<CreateOrUpdateResult, ServerFnError> {
    let author = crate::auth::require_login()?;
    let tags = tags.to_lowercase();
    let tags: Vec<_> = tags.split_whitespace().collect();

    let res;
    if let Some(slug) = slug {
        res = Article::update(&author, &slug, &title, &about, &body, &tags)
            .await
            .map(|res| match res {
                Some(errors) => Err(errors),
                None => Ok(slug),
            })
            .map_err(|e| {
                tracing::error!("article update failed: {:?}", e);
                ServerFnError::ServerError("article update failed".into())
            });
    } else {
        res = Article::create(&author, &title, &about, &body, &tags)
            .await
            .map_err(|e| {
                tracing::error!("article creation failed: {:?}", e);
                ServerFnError::ServerError("article creation failed".into())
            });
    }
    if let Ok(Ok(slug)) = &res {
        leptos_axum::redirect(&format!("/article/{}", slug));
    }
    res
}

#[component]
fn EditorForm(
    #[prop(into)] button_label: String,
    #[prop(optional)] slug: Signal<String>,
    #[prop(optional)] fields: Option<ArticleEditFields>,
    action: Action<CreateOrUpdatePost, Result<CreateOrUpdateResult, ServerFnError>>,
) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <input type="hidden" name="slug" value=slug/>
            <fieldset>
                <fieldset class="form-group">
                    <input
                        type="text"
                        class="form-control form-control-lg"
                        placeholder="Article Title"
                        name="title"
                        value=fields.as_ref().map(|a| a.title.clone())
                    />
                </fieldset>
                <fieldset class="form-group">
                    <input
                        type="text"
                        class="form-control"
                        placeholder="What's this article about?"
                        name="about"
                        value=fields.as_ref().map(|a| a.description.clone())
                    />
                </fieldset>
                <fieldset class="form-group">
                    <textarea
                        class="form-control"
                        rows="8"
                        placeholder="Write your article (in markdown)"
                        name="body"
                        prop:value=fields.as_ref().map(|a| a.body.clone()).unwrap_or_default()
                    ></textarea>
                </fieldset>
                <fieldset class="form-group">
                    <input
                        type="text"
                        class="form-control"
                        placeholder="Enter tags"
                        name="tags"
                        value=fields.as_ref().map(|a| a.tags.join(" "))
                    />
                    // TODO: client side fancy stuff for tags
                    <div class="tag-list">
                        <span class="tag-default tag-pill">
                            <i class="ion-close-round"></i>
                            tag
                        </span>
                    </div>
                </fieldset>
                <button
                    disabled=action.pending()
                    class="btn btn-lg pull-xs-right btn-primary"
                    type="submit"
                >
                    {button_label}
                </button>
            </fieldset>
        </ActionForm>
    }
}

#[component]
pub fn Edit() -> impl IntoView {
    let params = use_params::<ArticleSlugParam>();
    let slug = Signal::derive(move || params().expect("slug").slug);
    let post = create_server_action::<CreateOrUpdatePost>();
    let to_edit = create_blocking_resource(slug, get_article_for_editing);
    let result = post.value();
    let errors = move || {
        if let Some(Ok(Err(errors))) = result() {
            errors
        } else {
            Vec::new()
        }
    };
    let edit_form = move || {
        to_edit().map(|a| {
            a.map(|a| {
                view! { <EditorForm button_label="Save" slug=slug fields=a action=post/> }
            })
        })
    };
    view! {
        <div class="editor-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-10 offset-md-1 col-xs-12">
                        <ul class="error-messages">
                            <For each=errors key=|e| e.clone() let:error>
                                <li>{error}</li>
                            </For>
                        </ul>

                        <Suspense fallback=|| "Loading...">
                            <ErrorBoundary fallback=error_boundary_fallback>
                                {edit_form}
                            </ErrorBoundary>
                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn New() -> impl IntoView {
    let post = create_server_action::<CreateOrUpdatePost>();
    let result = post.value();
    let errors = move || {
        if let Some(Ok(Err(errors))) = result() {
            errors
        } else {
            Vec::new()
        }
    };
    view! {
        <div class="editor-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-10 offset-md-1 col-xs-12">
                        <ul class="error-messages">
                            <For each=errors key=|e| e.clone() let:error>
                                <li>{error}</li>
                            </For>
                        </ul>

                        <EditorForm button_label="Publish article" action=post/>
                    </div>
                </div>
            </div>
        </div>
    }
}
