use crate::{
    app::{use_current_user, Feed, FeedKind, FollowButton, NavLink, NBSP},
    error_template::error_boundary_fallback,
    models::user::Profile,
};
use leptos::*;
use leptos_router::*;

pub fn profile_link(username: &str) -> String {
    format!("/profile/{}", username)
}

#[component(transparent)]
pub fn ProfileRoute() -> impl IntoView {
    view! {
        <Route path="/profile/:username" view=Profile>
            // TODO: this also fails with TrailingSlash::Redirect, so giving up on that
            // no the routing is bit more fiddly, but whatever
            // TODO: maybe add redirection logic on 404 to strip trailing /
            <Route path="/" view=|| view! { <ProfileFeed/> }/>
            <Route path="/favorites" view=|| view! { <ProfileFeed favorites=true/> }/>
        </Route>
    }
}

#[component]
pub fn ProfileImg(src: Option<String>, #[prop(optional)] class: &'static str) -> impl IntoView {
    // TODO: check if the view updates correctly
    match src {
        Some(url) if url.starts_with("https://") => {
            view! { <img src=url class=class/> }.into_view()
        }
        Some(text) => text.into_view(),
        None => "🙂".into_view(),
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let user = use_current_user();
    let params = use_params::<UserParam>();
    let username = move || params().expect("username in path").username;

    let profile = create_blocking_resource(username, profile_data);

    let profile_details = move || {
        profile().map(|p| {
            p.map(|p| {
                let p = create_rw_signal(p);
                view! {
                    <div class="col-xs-12 col-md-10 offset-md-1">
                        <ProfileImg src=p().image class="user-img"/>
                        <h4>{move || p().username}</h4>
                        <p>{move || p().bio}</p>
                        <Show
                            when=move || {
                                user.with(|u| {
                                    u.as_ref().is_some_and(|u| u.username == username())
                                })
                            }

                            fallback=move || {
                                view! { <FollowButton class="action-btn" profile=p.split()/> }
                            }
                        >

                            <A href="/settings" class="btn btn-sm btn-outline-secondary action-btn">
                                <i class="ion-gear-a"></i>
                                {NBSP}
                                Edit Profile Settings
                            </A>
                        </Show>
                    </div>
                }
            })
        })
    };

    view! {
        <div class="profile-page">
            <div class="user-info">
                <div class="container">
                    <div class="row">
                        <Transition fallback=|| "Loading profile...">
                            <ErrorBoundary fallback=error_boundary_fallback>
                                {profile_details}
                            </ErrorBoundary>
                        </Transition>
                    </div>
                </div>
            </div>
            <div class="container">
                <div class="row">
                    <Outlet/>
                </div>
            </div>
        </div>
    }
}

#[derive(Params, PartialEq, Eq, Clone)]
struct UserParam {
    username: String,
}

#[component]
fn ProfileFeed(#[prop(optional)] favorites: bool) -> impl IntoView {
    let params = use_params::<UserParam>();
    let username = move || params().expect("username in path").username;
    let profile = move || profile_link(&username());
    let fav = move || format!("{}/favorites", profile());
    let kind = if favorites {
        Signal::derive(move || FeedKind::Favorited(username()))
    } else {
        Signal::derive(move || FeedKind::By(username()))
    };
    view! {
        <Feed kind=kind>
            <NavLink href=Signal::derive(profile)>My Articles</NavLink>
            <NavLink href=Signal::derive(fav)>Favorited Articles</NavLink>
        </Feed>
    }
}

#[server]
async fn profile_data(username: String) -> Result<Profile, ServerFnError> {
    let for_user = crate::auth::authenticated_username();
    crate::models::user::User::profile(&username, for_user.as_deref())
        .await
        .map_err(|e| {
            tracing::error!("failed to get profile: {:?}", e);
            ServerFnError::ServerError("Could not fetch profile data".into())
        })
}
