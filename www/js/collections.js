const getQueryParams = (params, url) => {
    let href = url;
    // this is an expression to get query strings
    let regexp = new RegExp('[?&]' + params + '=([^&#]*)', 'i');
    let qString = regexp.exec(href);
    return qString ? qString[1] : null;
};

window.addEventListener('load', evt => {
    const token = getQueryParams('token', window.location.href);
    if (token === null) {
        alert('token not found');
        return;
    }

    const sectionEl = document.querySelector('section');
    if (!sectionEl) {
        alert('could not find <section>');
        return;
    }

    const headers = new Headers();
    headers.append('Authorization', `Bearer ${token}`);
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


                        const rowEl = document.createElement('td');
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
});
