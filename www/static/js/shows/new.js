function send() {
    let activity = UIWait();
    let data = {
        title: $('[name="title"]').val(),
        subtitle: $('[name="subtitle"]').val(),
        description: $('[name="description"]').val(),
        notes: $('[name="notes"]').val()
    };

    HTTP("POST", "/api/shows", data)
        .success(function (contents) {
            window.location.href = `/shows/${contents.show_id}`;
        })
        .error(function (err) {
            console.log(err);
        })
        .complete(function () {
            UIReady(activity);
        })
        .finish();
}