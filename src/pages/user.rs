use leptos::*;
use leptos_router::*;

type VoidAction<T> = Action<T, Result<(), ServerFnError>>;

#[component]
pub fn Login(login: VoidAction<crate::auth::Login>) -> impl IntoView {
    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Sign in</h1>
                        <p class="text-xs-center">
                            <a href="/register">Need an account?</a>
                        </p>

                        <Show when=move || {
                            login.value().with(|val| val.as_ref().is_some_and(|x| x.is_err()))
                        }>
                            <ul class="error-messages">
                                <li>Incorrect username or password.</li>
                            </ul>
                        </Show>

                        <ActionForm action=login>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    name="username"
                                    placeholder="Username"
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="password"
                                    name="password"
                                    placeholder="Password"
                                />
                            </fieldset>
                            <button type="submit" class="btn btn-lg btn-primary pull-xs-right">
                                Sign in
                            </button>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Register(register: VoidAction<crate::auth::Register>) -> impl IntoView {
    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Sign up</h1>
                        <p class="text-xs-center">
                            <a href="/login">Have an account?</a>
                        </p>

                        <ul class="error-messages">
                            <li>That email is already taken</li>
                        </ul>

                        <ActionForm action=register>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    name="username"
                                    placeholder="Username"
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    name="email"
                                    placeholder="Email"
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="password"
                                    name="password"
                                    placeholder="Password"
                                />
                            </fieldset>
                            <button type="submit" class="btn btn-lg btn-primary pull-xs-right">
                                Sign up
                            </button>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Settings(logout: VoidAction<crate::auth::Logout>) -> impl IntoView {
    view! {
        <div class="settings-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Your Settings</h1>

                        <ul class="error-messages">
                            <li>That name is required</li>
                        </ul>

                        <form>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="URL of profile picture"
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Your Name"
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control form-control-lg"
                                        rows="8"
                                        placeholder="Short bio about you"
                                    ></textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Email"
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="New Password"
                                    />
                                </fieldset>
                                <button class="btn btn-lg btn-primary pull-xs-right">
                                    Update Settings
                                </button>
                            </fieldset>
                        </form>
                        <hr/>
                        <ActionForm action=logout>
                            <button type="submit" class="btn btn-outline-danger">
                                Or click here to logout.
                            </button>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </div>
    }
}
