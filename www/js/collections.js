const getQueryParams = (params, url) => {
    let href = url;
    // this is an expression to get query strings
    let regexp = new RegExp('[?&]' + params + '=([^&#]*)', 'i');
    let qString = regexp.exec(href);
    return qString ? qString[1] : null;
};

function makeLink(value) {
    return `<a target="_blank" href="${value}">${value}</a>`;
}

window.addEventListener('load', evt => {
    const sectionEl = document.querySelector('section');
    if (!sectionEl) {
        alert('could not find <section>');
        return;
    }

    update_collection();

    function update_collection() {
        const headers = new Headers();
        // headers.append('Authorization', `Bearer ${token}`);
        headers.append('Content-Type', 'application/json');
        const request = fetch('/api/v1/stigmarks',
            {
                method: 'GET',
                cache: 'no-cache',
                headers: headers,
            });
        request
            .then(response => {
                if (response.status != 200) {
                    alert("failed to enum collections");
                    return;
                }
                response.json()
                    .then(collections => {
                        sectionEl.innerHTML = "";

                        const tableEl = document.createElement('table');
                        sectionEl.appendChild(tableEl);

                        for (let i in collections) {
                            const collection = collections[i];

                            const rowEl = document.createElement('tr');
                            tableEl.appendChild(rowEl);
                            rowEl.dataset['collection'] = collection.collection_id;

                            const cellUserNameEl = document.createElement('td');
                            rowEl.appendChild(cellUserNameEl);
                            cellUserNameEl.innerHTML = collection.user_name;

                            const cellDateEl = document.createElement('td');
                            rowEl.appendChild(cellDateEl);
                            cellDateEl.innerHTML = collection.creation_date.split('T')[0];

                            const cellUrlsIdEl = document.createElement('td');
                            rowEl.appendChild(cellUrlsIdEl);
                            const urls = collection.urls.map(makeLink);
                            cellUrlsIdEl.innerHTML = urls.join('<br/>');

                            const cellKeywordsIdEl = document.createElement('td');
                            rowEl.appendChild(cellKeywordsIdEl);
                            cellKeywordsIdEl.innerHTML = collection.keywords.join("<br/>");
                        }
                    })
                    .catch(err => {
                        alert("could not decode json");
                        return;
                    });
            })
            .catch(err => {
                alert("login failed");
                return;
            });
    }
});
