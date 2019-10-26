#[macro_export]
macro_rules! cookie_get_auth {
    ($req:expr) => {
        match $req.cookie("auth") {
            None => None,
            Some(token) => {
                let token = token.value().to_owned();

                let aux = showman_core::get_navigation_auxiliary_data(&$req);

                let claims = $crate::claims::Claims::from_token(&token, &aux, &$crate::SIGN_KEY);

                match claims {
                    Ok(claims) => Some(claims),
                    Err(_) => None
                }
            }
        }
    }
}

#[macro_export]
macro_rules! cookie_require_auth {
    ($req:expr, $err:expr) => {
        if let Some(claims) = cookie_get_auth!($req) {
            claims
        } else {
            return $err;
        }
    }
}