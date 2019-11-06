/*let test_shows = [
    {id: 2, access: false, notes: "Natale 2019"},
    {id: 1, access: true, title: "Hardy Potter e lo spettacolo di prova", subtitle: "Questo &egrave; lo spettacolo utilizzato per testare il software", notes: "Test 2019"}
];*/

function ready() {
    let activity = UIWait();

    HTTP("GET", "/api/shows")
        .success(function (contents) {
            let list = $('#show-list');
            list.empty();
            for (let show of contents) {
                let image, title, subtitle, notes;
                if (show.access) {
                    image = $('<img class="avatar" src="https://picsum.photos/200/300/?random" alt="Spettacolo">');
                    title = $(`<a href="/shows/${show.id}" class="label text-medium">${show.title}</a>`);
                    subtitle = $(`<span class="label">${show.subtitle}</span>`);
                    notes = $(`<span class="second-label">${show.notes}</span>`);
                } else {
                    image = $('<img class="avatar" src="https://picsum.photos/200/300/?random&blur=10&grayscale" alt="Non autorizzato">');
                    title = $('<span class="label fg-gray text-medium">Non sei autorizzato a vedere questo spettacolo</span>');
                    subtitle = $(`<a href="#" class="label">Richiedi l'autorizzazione</a>`);
                    notes = $(`<span class="second-label">${show.notes}</span>`);
                }
                $('<li>')
                    .append(image)
                    .append(title)
                    .append(subtitle)
                    .append(notes)
                    .appendTo(list);
            }
        })
        .error(function (err) {
            console.log("UNIMPLEMENTED");
        })
        .complete(() => UIReady(activity))
        .finish();
}

$(ready);