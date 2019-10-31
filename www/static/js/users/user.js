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
    HTTP('GET', '/api/users/1')
        .success(function (contents) {
            $('[name="name"]').val(contents.name);
            $('[name="surname"]').val(contents.surname);
            let role = Role(contents.role);
            $('[name="role"]').val(role.caption);
        })
        .error(function (err) {
            console.log("Unexpected error: ", err);
        })
        .complete(function () {
            console.log("Completed!");
        }).finish();
    console.log("UNIMPLEMENTED");
}

$(ready);