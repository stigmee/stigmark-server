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
    // const token = getQueryParams('token', window.location.href);
    // if (token === null) {
    //     alert('token not found');
    //     return;
    // }

    const sectionEl = document.querySelector('section');
    if (!sectionEl) {
        alert('could not find <section>');
        return;
    }

    const buttonEl = document.querySelector('button');
    if (!buttonEl) {
        alert('could not find <button>');
        return;
    }
    buttonEl.addEventListener('click', evt => {
        buttonEl.disabled = true;

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
                buttonEl.disabled = false;
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

                            const cellIdEl = document.createElement('td');
                            rowEl.appendChild(cellIdEl);
                            cellIdEl.innerHTML = `collection ${collection.collection_id}`;

                            const cellUserIdEl = document.createElement('td');
                            rowEl.appendChild(cellUserIdEl);
                            cellUserIdEl.innerHTML = `shared by ${collection.user_id}`;

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
                buttonEl.disabled = false;
                alert("login failed");
                return;
            });
    });
});
