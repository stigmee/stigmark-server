window.addEventListener('load', evt => {
    const sectionEl = document.querySelector('section');
    if (!sectionEl) {
        alert('could not find <section>');
        return;
    }

    update_stigmers();

    function update_stigmers() {
        const headers = new Headers();
        headers.append('Content-Type', 'application/json');
        const request = fetch('/api/v1/stigmers',
            {
                method: 'GET',
                cache: 'no-cache',
                headers: headers,
            });
        request
            .then(response => {
                if (response.status != 200) {
                    alert("failed to enum stigmers");
                    return;
                }
                response.json()
                    .then(subscriptions => {
                        if (subscriptions.length == 0) {
                            sectionEl.innerHTML = "you have no subscription yet";
                            sectionEl.className = "center";
                            return;
                        }

                        sectionEl.innerHTML = "";
                        sectionEl.className = "";

                        const tableEl = document.createElement('table');
                        sectionEl.appendChild(tableEl);

                        for (let i in subscriptions) {
                            const subscription = subscriptions[i];

                            const rowEl = document.createElement('tr');
                            tableEl.appendChild(rowEl);
                            rowEl.dataset['stigmer_id'] = subscription.stigmer_id;

                            const cellStigmerNameEl = document.createElement('td');
                            rowEl.appendChild(cellStigmerNameEl);
                            cellStigmerNameEl.innerHTML = subscription.stigmer_name;

                            const cellStigmerMailEl = document.createElement('td');
                            rowEl.appendChild(cellStigmerMailEl);
                            cellStigmerMailEl.innerHTML = subscription.stigmer_mail;

                            const cellStatusEl = document.createElement('td');
                            rowEl.appendChild(cellStatusEl);
                            if (subscription.forbidden_at)
                                cellStatusEl.innerHTML = "forbidden";
                            else if (subscription.authorized_at)
                                cellStatusEl.innerHTML = "authorized";
                            else 
                                cellStatusEl.innerHTML = "pending";
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
