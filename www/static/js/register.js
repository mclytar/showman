function send() {
    let waiter = UIWait();
    let data = {
        username: $('#registerData [name="email"]').val(),
        password: $('#registerData [name="password"]').val(),
        password_confirm: $('#registerData [name="password_confirm"]').val()
    };
    HTTP("POST", "/api/users", data)
        .success(success)
        .error(HTTP.Conflict, error_conflict)
        .error(HTTP.UnprocessableEntity, error_unprocessable_entity)
        .complete(() => UIReady(waiter))
        .finish();
}

function google_send() {
    Metro.dialog.create({
        title: "<span class='mif-warning fg-red'></span> &nbsp; <span>Non implementato</span>",
        content: "<div>La funzione 'Registrati con Google' non &egrave; ancora implementata.</div>",
        closeButton: true
    });
}

function success() {
    Metro.toast.create("Creazione effettuata!<br>A breve sarai portato alla schermata di accesso.", null, null, null, {
        timeout: 3000,
        clsToast: 'success',
        callback: () => { window.location.href = `/login`; }
    });
}

function error_conflict() {
    ClearFormError('registerData');
    SetFormError('registerData', 'email', 'Email gi&agrave; in uso. <a href="/login">Forse si desidera accedere?</a>');
}

function error_unprocessable_entity(response) {
    ClearFormError('registerData');
    if (response.username !== undefined) {
        SetFormError('registerData', 'email', 'Formato non valido.');
    }
    if (response.password !== undefined) {
        SetFormError('registerData', 'password', 'La password deve contenere almeno 5 caratteri.');
    }
    if (response.password_confirm !== undefined) {
        SetFormError('registerData', 'password_confirm', 'Le due password devono coincidere.');
    }
}