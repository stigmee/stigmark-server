import { delete_cookie } from "/js/utils/cookies.js";

window.addEventListener('load', evt => {
    delete_cookie('stigmark');
    window.location.href = `/login.htm`;
});
