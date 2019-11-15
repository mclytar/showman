function send() {
    let waiter = UIWait();
    let data = {
        username: $('#loginData [name="email"]').val(),
        password: $('#loginData [name="password"]').val()
    };
    HTTP("POST", "/auth/session", data)
        .success(goto_index)
        .error(HTTP.Unauthorized, error_unauthorized)
        .complete(() => UIReady(waiter))
        .finish();
}

function goto_index() {
    setTimeout(() => window.location.assign("/"), 10);
}

function error_unauthorized() {
    SetFormError('loginData', 'email', 'Credenziali errate.');
    SetFormError('loginData', 'password', 'Credenziali errate.');
}