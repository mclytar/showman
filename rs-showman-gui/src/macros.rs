#[macro_export]
macro_rules! web_page {
    ($path:expr) => {
        actix_web::web::get().to(|req: actix_web::HttpRequest| {
            let mut settings = Settings::new();

            if let Some(claims) = cookie_get_auth!(req) {
                settings.set_var("username", claims.display());
                settings.set_var("user_id", &format!("{}", claims.user_id()));
                settings.set_template("appbar", if claims.role() == Role::Maintainer || claims.role() == Role::Administrator { "admin" } else { "user" });
            }

            $crate::preprocessor::load(&format!("./www/gui/{}", $path), &settings).map(|s| HttpResponse::Ok().body(s))
        })
    };
}

macro_rules! __reserved_web_page__init {
    ($req:ident, $settings:ident) => {{
        let aux = showman_core::get_navigation_auxiliary_data(&$req);
        let token = match $req.cookie("auth") {
            Some(cookie) => cookie.value().to_owned(),
            None => return $crate::preprocessor::err::unauthorized(&$settings)
        };
        let mut claims = match showman_auth_base::claims::Claims::from_token(&token, &aux, &showman_auth_base::SIGN_KEY) {
            Ok(claims) => claims,
            Err(_) => return $crate::preprocessor::err::unauthorized(&$settings)
        };
        let needs_update = claims.needs_update();
        if needs_update {
            claims.fetch_update(&aux);
        }
        (claims, needs_update)
    }}
}

macro_rules! __reserved_web_page__settings {
    ($settings:ident, $claims:ident) => {
        $settings.set_var("username", $claims.display());
        $settings.set_var("user_id", &format!("{}", $claims.user_id()));
        $settings.set_template("appbar", if $claims.role() == Role::Maintainer || $claims.role() == Role::Administrator { "admin" } else { "user" });
    }
}

macro_rules! __reserved_web_page__end {
    ($path:expr, $settings:ident, $claims:ident, $needs_update:ident) => {{
        let page = $crate::preprocessor::load(&format!("./www/gui/{}", $path), &$settings)?;

        let mut response = HttpResponse::Ok();

        if $needs_update {
            match $claims.token() {
                Ok(token) => response.cookie(showman_core::authentication_cookie(&token)),
                Err(_) => return $crate::preprocessor::err::internal_server_error(&$settings)
            };
        }

        Ok(response.body(page))
    }}
}

#[macro_export]
macro_rules! reserved_web_page {
    ($path:expr, $($param:ident : $type:ty),*, ($req:ident, $claims:ident, $settings:ident) => $exec:block) => {
        actix_web::web::get().to(|$req: actix_web::HttpRequest, params: actix_web::web::Path<($($type),*)>| {
            // Initialize and check for authentication.
            let mut $settings = Settings::new();
            let ($claims, needs_update) = __reserved_web_page__init!($req, $settings);

            // Fetch parameters.
            #[allow(unused_parens)]
            let ($($param),*) = params.into_inner();

            // Set up parameters.
            __reserved_web_page__settings!($settings, $claims);
            $(
                $settings.set_var(stringify!($param), &format!("{}", $param));
            )*

            // Execute custom block.
            $exec

            // Write response.
            __reserved_web_page__end!($path, $settings, $claims, needs_update)
        })
    };
    ($path:expr, $($param:ident : $type:ty),*) => {
        reserved_web_page!($path, $($param : $type),*, (req, claims, settings) => {})
    };
    ($path:expr, ($req:ident, $claims:ident, $settings:ident) => $exec:block) => {
        actix_web::web::get().to(|$req: actix_web::HttpRequest| {
            // Initialize and check for authentication.
            let mut $settings = Settings::new();
            let ($claims, needs_update) = __reserved_web_page__init!($req, $settings);

            // Set up parameters.
            __reserved_web_page__settings!($settings, $claims);

            // Execute custom block.
            $exec

            // Write response.
            __reserved_web_page__end!($path, $settings, $claims, needs_update)
        })
    };
    ($path:expr) => {
        reserved_web_page!($path, (req, claims, settings) => {})
    };
}