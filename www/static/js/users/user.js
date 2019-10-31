function send_avatar() {
    console.log("UNIMPLEMENTED");
}

function send_user() {
    console.log("UNIMPLEMENTED");
}

function send_auth_password() {

    console.log("UNIMPLEMENTED");
}

function send_auth_google() {
    console.log("UNIMPLEMENTED");
}

function ready() {
    let id = Meta('showman-id');

    let activity = UIWait();

    let progress = {user: false,
        avatar: false,
        auth: false,
        update: function () {
            if (this.user && this.avatar && this.auth) {
                UIReady(activity);
            }
        }};

    HTTP('GET', `/api/users/${id}`)
        .success(function (contents) {
            let role = Role(contents.role);

            $('[name="name"]').val(contents.name);
            $('[name="surname"]').val(contents.surname);
            $('[name="role"]').val(role.caption);
        })
        .error(function (err) {
            console.log("Unexpected error: ", err);
        })
        .complete(() => { progress.user = true; progress.update(); })
        .finish();
    HTTP('GET', `/api/users/${id}/avatar`)
        .success(function (contents) {

        })
        .error(function (err) {
            console.log("Unexpected error: ", err);
        })
        .complete(() => { progress.avatar = true; progress.update(); })
        .finish();
    HTTP('GET', `/api/users/${id}/auth`)
        .success(function (contents) {

        })
        .error(function (err) {
            console.log("Unexpected error: ", err);

        })
        .complete(() => { progress.auth = true; progress.update(); })
        .finish();
}

$(ready);