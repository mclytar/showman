function UIWait() {
    return Metro.activity.open({
        type: 'cycle',
        overlayColor: '#FFF',
        overlayAlpha: .5,
        text: `<div class="mt-2 text-small">Attendi...</div>`
    });
}

function UIReady(activity) {
    Metro.activity.close(activity);
}

function SetFormError(form, field, error) {
    let $field = $(`#${form} [name="${field}"]`);
    $field.parent()
        .parent()
        .children('.invalid_feedback')
        .remove();
    $field.parent()
        .after(`<span class="invalid_feedback">${error}</span>`);

    Metro.validator.reset_state($field);
    Metro.validator.set_invalid_state($field);
}

function ClearFormError(form, field = null) {
    if (field === null) {
        let $field = $(`#${form} [name]`);
        $field.parent()
            .parent()
            .children('.invalid_feedback')
            .remove();

        Metro.validator.reset_state($field);
    } else {
        let $field = $(`#${form} [name="${field}"]`);
        $field.parent()
            .parent()
            .children('.invalid_feedback')
            .remove();

        Metro.validator.reset_state($field);
    }
}

function HTTP(method, url, data) {
    return {
        ajax: {
            method: method,
            url: url,
            data: data
        },
        success_handlers: [],
        error_handlers: [],
        complete_handlers: [],
        success: function (handler) {
            if (typeof handler !== "function") {
                throw new TypeError(`Expected .success(function), got .success(${typeof handler})`);
            } else {
                this.success_handlers.push(handler);
            }
            return this;
        },
        error: function (code, handler) {
            if (typeof code === "function" && typeof handler === "undefined") {
                this.error_handlers.push({
                    code: null,
                    handler: code
                });
            } else if (typeof code === "number" && typeof handler === "function") {
                this.error_handlers.push({
                    code: code,
                    handler: handler
                });
            } else {
                if (typeof handler === "undefined") {
                    throw new TypeError(`Expected .success(function) or .success(number, function), got .success(${typeof code})`);
                } else {
                    throw new TypeError(`Expected .success(function) or .success(number, function), got .success(${typeof code}, ${typeof handler})`);
                }
            }
            return this;
        },
        complete: function (handler) {
            if (typeof handler !== "function") {
                throw new TypeError(`Expected .success(function), got .success(${typeof handler})`);
            } else {
                this.complete_handlers.push(handler);
            }
            return this;
        },
        finish: function () {
            $.ajax(this.ajax)
                .then(function (response) {
                    let data;
                    try {
                        data = JSON.parse(response);
                    } catch {
                        data = response;
                    }
                    this.success_handlers.forEach((value) => value(data));
                    this.complete_handlers.forEach((value) => value());
                }.bind(this), function (xhr) {
                    let data;
                    try {
                        data = JSON.parse(xhr.responseText);
                    } catch {
                        data = xhr.responseText;
                    }
                    let handlers = this.error_handlers.filter((e) => e.code === xhr.status);
                    if (handlers.length > 0) {
                        handlers.forEach((e) => e.handler(data));
                    } else {
                        this.error_handlers.filter((e) => e.code === null)
                            .forEach((e) => e.handler(data));
                    }
                    this.complete_handlers.forEach((value) => value());
                }.bind(this));
        }
    }
}

HTTP.is_success = function (code) { return code >= 200 && code <= 299; };
HTTP.Ok = 200;
HTTP.Created = 201;
HTTP.Accepted = 202;
HTTP.NoContent = 204;
HTTP.is_redirect = function (code) { return code >= 300 && code <= 399; };
HTTP.MovedPermanently = 301;
HTTP.Found = 302;
HTTP.SeeOther = 303;
HTTP.NotModified = 304;
HTTP.TemporaryRedirect = 307;
HTTP.is_client_error = function (code) { return code >= 400 && code < 499; };
HTTP.BadRequest = 400;
HTTP.Unauthorized = 401;
HTTP.Forbidden = 403;
HTTP.NotFound = 404;
HTTP.MethodNotAllowed = 405;
HTTP.NotAcceptable = 406;
HTTP.RequestTimeout = 408;
HTTP.Conflict = 409;
HTTP.Gone = 410;
HTTP.PreconditionFailed = 412;
HTTP.UnsupportedMediaType = 415;
HTTP.UnprocessableEntity = 422;
HTTP.is_server_error = function (code) { return code >= 500 && code < 599; };
HTTP.InternalServerError = 500;
HTTP.NotImplemented = 501;

function Role(desc) {
    switch (desc) {
        case 'user':
            return Role.User;
        case 'maintainer':
            return Role.Maintainer;
        case 'administrator':
            return Role.Administrator;
        default:
            return Role.Invalid;
    }
}

Role.Maintainer = {name: 'maintainer', caption: 'Maintainer'};
Role.Administrator = {name: 'admin', caption: 'Amministratore'};
Role.User = {name: 'user', caption: 'Utente'};

Role.Invalid = {name: 'invalid', caption: 'DATI NON VALIDI'};

/**
 * @return {String|null}
 */
function Meta(name) {
    let $tag = $(`meta[name='${name}']`);

    if ($tag.length !== 0) {
        return $tag.val('content');
    }

    return null;
}

$username = $('[data-showman-content="username"]');
if ($username.text().trim() === "") {
    $username.prepend("<i>Utente</i>");
    Metro.notify.create("Per poter completare la tua iscrizione, vai sul profilo ed imposta nome e cognome.", "Iscrizione incompleta", {
        keepOpen: true
    });
}