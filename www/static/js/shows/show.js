function ready() {
    let activity = UIWait();

    let show_id = Meta('showman-show-id');

    HTTP("GET", `/api/shows/${show_id}`)
        .success(function (contents) {
            $('#show-title').text(contents.title);
            $('#show-subtitle').text(contents.subtitle);
            $('#show-description').html(contents.description.replace(/\n/gi, "<br>"));
            $('#show-notes').text(contents.notes);
            console.log(contents);
        })
        .error(function (err) {
            console.log(err);
        })
        .complete(function () {
            UIReady(activity);
        })
        .finish();
}

function get_item(node) {
    let show_id = Meta('showman-show-id');

    let item = $(node).attr('data-href');
    console.log(item);
    window.location.assign(`/shows/${show_id}/${item}`);
}

$(ready);