window.addEventListener('load', evt => {
    const formEl = document.querySelector('form');
    if (!formEl) {
        alert('could not find "form"');
        return;
    }

    formEl.addEventListener('submit', evt => {
        evt.preventDefault();
        const data = new FormData(formEl);
        const pass1 = data.get('pass1');
        const pass2 = data.get('pass2');
        if (pass1 != pass2) {
            alert("different passwords");
            return;
        }
        const body = JSON.stringify({
            user: data.get('user'),
            mail: data.get('mail'),
            pass: pass1,
        });
        const headers = new Headers();
        headers.append('Content-Type', 'application/json');
        const request = fetch('/api/v1/signup',
            {
                method: 'POST',
                cache: 'no-cache',
                headers: headers,
                body: body,
            });
        request
            .then(response => {
                if (response.status != 201) {
                    alert("failed to signup");
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
                alert("signup failed");
                return;
            });
    });
});
