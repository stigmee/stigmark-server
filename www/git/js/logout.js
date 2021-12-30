window.addEventListener('load', _ => {
    const logoutData = {
        method: 'DELETE',
    };
    fetch("/api/v1/login", logoutData)
        .then(res => {
            console.log(`DELETE /api/v1/login returned ${res.status}`);
            if (res.status >= 200 && res.status < 300) {
                window.location.href = `/login.htm`;
                return;
            }
            alert(`failed to logout with ${res.status}`);
        })
        .catch(err => {
            alert(`logout crashed with ${err}`);
        })
        ;
});
