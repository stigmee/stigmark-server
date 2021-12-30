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
            mail: data.get('mail'),
            pass: data.get('pass'),
        });
        const headers = new Headers();
        headers.append('Content-Type', 'application/json');
        const request = fetch('/api/v1/login',
            {
                method: 'POST',
                cache: 'no-cache',
                headers: headers,
                body: body,
            });
        request
            .then(response => {
                if (response.status != 201) {
                    alert("failed to login");
                    return;
                }
                response.json()
                    .then(json => {
                        // document.cookie = `stigmark=${json.token}; expires=Fri, 31 Dec 9999 23:59:59 GMT; SameSite=None; Secure`;
                        // window.location.href = `/search.htm?token=${json.token}`;
                        window.location.href = `/search.htm`;
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
});
