macro_rules! get_auth_user_data {
    ($req:ident) => {{
        $req.cookie("auth")
            .and_then(|c| {
                use showman_auth_base::session;

                let auth_token = c.value();

                session::update(auth_token)
                    .and_then(|_| session::get(&auth_token))
                    .ok()
            })
    }};
    ($req:ident, $settings:ident) => {{
        let auth_token = match $req.cookie("auth") {
            Some(cookie) => cookie.value().to_owned(),
            None => return $crate::preprocessor::err::unauthorized(&$settings)
        };
        match showman_auth_base::session::update(&auth_token) {
            Ok(_) => {},
            Err(_) => return $crate::preprocessor::err::unauthorized(&$settings)
        }
        match showman_auth_base::session::get(&auth_token) {
            Ok(auth) => auth,
            Err(_) => return $crate::preprocessor::err::unauthorized(&$settings)
        }
    }}
}

macro_rules! settings_setup {
    ($auth_data:ident, $settings:ident) => {{
        $settings.set_var("username", $auth_data.name());
        $settings.set_var("user_id", &format!("{}", $auth_data.user_id()));
        $settings.set_template("appbar", if $auth_data.role() == Role::Maintainer || $auth_data.role() == Role::Administrator { "admin" } else { "user" });
    }}
}

#[macro_export]
macro_rules! web_page {
    ($path:expr) => {
        actix_web::web::get().to(|req: actix_web::HttpRequest| {
            let mut settings = Settings::new();

            if let Some(session_data) = get_auth_user_data!(req) {
                settings_setup!(session_data, settings);
            }

            $crate::preprocessor::load(&format!("./www/gui/{}", $path), &settings).map(|s| HttpResponse::Ok().body(s))
        })
    };
}

#[macro_export]
macro_rules! reserved_web_page {
    ($path:expr, $($param:ident : $type:ty),*, ($req:ident, $user_data:ident, $settings:ident) => $exec:block) => {
        actix_web::web::get().to(|$req: actix_web::HttpRequest, params: actix_web::web::Path<($($type),*)>| {
            let mut $settings = Settings::new();
            let $user_data = get_auth_user_data!($req, $settings);

            #[allow(unused_parens)]
            let ($($param),*) = params.into_inner();

            settings_setup!($user_data, $settings);
            $(
                $settings.set_var(stringify!($param), &format!("{}", $param));
            )*

            $exec

            let page_contents = $crate::preprocessor::load(&format!("./www/gui/{}", $path), &$settings)?;

            Ok(HttpResponse::Ok().body(page_contents))
        })
    };
    ($path:expr, $($param:ident : $type:ty),*) => {
        reserved_web_page!($path, $($param : $type),*, (req, user_data, settings) => {})
    };
    ($path:expr, ($req:ident, $user_data:ident, $settings:ident) => $exec:block) => {
        actix_web::web::get().to(|$req: actix_web::HttpRequest| {
            let mut $settings = Settings::new();
            let $user_data = get_auth_user_data!($req, $settings);

            settings_setup!($user_data, $settings);

            $exec

            let page_contents = $crate::preprocessor::load(&format!("./www/gui/{}", $path), &$settings)?;

            Ok(HttpResponse::Ok().body(page_contents))
        })
    };
    ($path:expr) => {
        reserved_web_page!($path, (req, user_data, settings) => {})
    };
}