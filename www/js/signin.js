window.addEventListener('load', evt => {
    const formEl = document.querySelector('form');
    if (!formEl) {
        alert('could not find "form"');
        return;
    }

    formEl.addEventListener('submit', evt => {
        evt.preventDefault();
        const data = new FormData(formEl);
        const body = JSON.stringify({
            user: data.get('user'),
            pass: data.get('pass'),
        });
        const headers = new Headers();
        headers.append('Content-Type', 'application/json');
        const request = fetch('/api/v1/signin',
            {
                method: 'POST',
                cache: 'no-cache',
                headers: headers,
                body: body,
            });
        request
            .then(response => {
                if (response.status != 201) {
                    alert("failed to signin");
                    return;
                }
                response.json()
                    .then(json => {
                        console.log(json);
                    })
                    .catch(err => {
                        alert("could not decode json");
                        return;
                    });
            })
            .catch(err => {
                alert("signin failed");
                return;
            });
    });
});
