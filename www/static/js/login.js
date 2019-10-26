function send() {
    let waiter = UIWait();
    let data = {
        username: $('#loginData [name="email"]').val(),
        password: $('#loginData [name="password"]').val()
    };
    HTTP("POST", "/auth/session", data)
        .success(() => window.location.href = "/")
        .error(HTTP.Unauthorized, error_unauthorized)
        .complete(() => UIReady(waiter))
        .finish();
}

function error_unauthorized() {
    SetFormError('loginData', 'email', 'Credenziali errate.');
    SetFormError('loginData', 'password', 'Credenziali errate.');
}