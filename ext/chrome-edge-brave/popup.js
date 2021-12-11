// 
//  Stigmee: A 3D browser and decentralized social network.
//  Copyright 2021 Philippe Anel <zexigh@gmail.com>
// 
//  This file is part of Stigmee.
// 
//  Project : stigmarks-chrome-edge-brave extension
//  Version : 0.0-1
// 
//  Stigmee is free software: you can redistribute it and/or modify it
//  under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
// 
//  This program is distributed in the hope that it will be useful, but
//  WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//  General Public License for more details.
// 
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
// 

const serverAddr = 'http://localhost:8000';
// const serverAddr = 'https://stigmark.badro.fr';

const requestUrl = `${serverAddr}/api/v1/stigmarks`;
const loginUrl = `${serverAddr}/api/v1/login`;

const port = chrome.runtime.connect({ name: 'stigmark' });
function debug_log(msg) {
    port.postMessage({ debug: msg });
}

async function get_all_tabs() {
    const queryOptions = { currentWindow: true };
    return chrome.tabs.query(queryOptions);
}

async function send_urls_and_keywords(token, urls, keywords) {
    debug_log(`sending urls=${JSON.stringify(urls)} and keywords=${JSON.stringify(keywords)}`);
    const body = { urls: urls, keys: keywords, token: token };
    const requestData = {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json; charset=utf-8',
            'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify(body),
    };
    try {
        const res = await fetch(requestUrl, requestData);
        if (res.status >= 200 && res.status < 300) {
            debug_log('urls+keys sent');
            return res.status;
        }
        debug_log(`urls+keys failed with status ${res.status}`);
        return res.status;
    }
    catch (e) {
        debug_log(`urls+keys failed with exception ${e.message}`);
    }
    return 0;
}

async function init_app(token) {
    debug_log('init_app');

    const listEl = document.querySelector('ul');
    if (!listEl) {
        debug_log('ul not found');
        return 0;
    }

    const keywordsEl = document.querySelector('textarea');
    if (!keywordsEl) {
        debug_log('textarea not found');
        return 0;
    }

    const onlyInputEl = document.querySelector('#onlySelected');
    if (!onlyInputEl) {
        debug_log('#onlySelected not found');
        return 0;
    }

    const sendBtnEl = document.querySelector('#send');
    if (!sendBtnEl) {
        debug_log('#send not found');
        return 0;
    }

    const errorMsgEl = document.querySelector('#error_send');
    if (!errorMsgEl) {
        debug_log('#error_send not found');
        return 0;
    }

    const entries = [];

    const tabs = await get_all_tabs();
    debug_log(tabs);
    tabs.forEach(tab => {
        const url = tab.url;
        if (!url.startsWith('http://') && !url.startsWith('https://'))
            return;
        const entryEl = document.createElement('li');
        const inputEl = document.createElement('input');
        inputEl.type = 'checkbox';
        inputEl.checked = true;
        const titleEl = document.createElement('span');
        titleEl.innerText = tab.title;
        entryEl.appendChild(inputEl);
        entryEl.appendChild(titleEl);
        listEl.appendChild(entryEl);
        entries.push({
            input: inputEl,
            tab: tab,
        });
    });

    onlyInputEl.addEventListener('change', () => {
        entries.forEach(entry => {
            entry.input.checked = entry.tab.highlighted || !onlyInputEl.checked;
        });
    });

    sendBtnEl.addEventListener('click', async () => {
        debug_log('clicked send');
        errorMsgEl.innerHTML = '';

        const urls = [];
        entries.forEach(entry => {
            if (entry.input.checked)
                urls.push(entry.tab.url);
        });

        const keywords = keywordsEl.value.split(/[ \t]*,[ \t]*/g)
        const is_ok = await send_urls_and_keywords(token, urls, keywords);
        if (!is_ok) {
            errorMsgEl.innerHTML = `could not send url and keywords`;
            return;
        }

        window.close();
    });
}

function login(email, passwd) {
    debug_log(`login: ${email}`);
    return new Promise((resolve, reject) => {
        debug_log(`login: in-promise`);
        const body = {
            mail: email,
            pass: passwd,
        };
        const loginData = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json; charset=utf-8' },
            body: JSON.stringify(body),
        };
        debug_log('sending login request');
        fetch(loginUrl, loginData)
            .then(res => {
                debug_log(`fetch returned ${res.status}`);
                if (res.status >= 200 && res.status < 300) {
                    res.json()
                        .then(data => {
                            debug_log(`fetch data`);
                            resolve(data);
                        })
                        .catch(err => {
                            debug_log(`could not decode json: ${err}`)
                            reject(err);
                        })
                        ;
                    return true;
                }
                debug_log(`fetch failed`);
                reject(`fetch failed`);
            })
            .catch(err => {
                debug_log(`fetch crashed with ${err}`);
                reject(`fetch crashed with ${err}`);
            })
            ;
    });
}

async function init_login() {
    debug_log('init_login');

    const emailEl = document.querySelector('#email');
    if (!emailEl) {
        debug_log('#email not found');
        return 0;
    }

    const passwordEl = document.querySelector('#password');
    if (!passwordEl) {
        debug_log('#password not found');
        return 0;
    }

    const loginBtnEl = document.querySelector('#login');
    if (!loginBtnEl) {
        debug_log('#login not found');
        return 0;
    }

    const signinEl = document.querySelector('#signin');
    if (!signinEl) {
        debug_log('#signin not found');
        return 0;
    }

    const errorMsgEl = document.querySelector('#error_login');
    if (!errorMsgEl) {
        debug_log('#error_login not found');
        return 0;
    }

    const appEl = document.querySelector('#app');
    if (!appEl) {
        debug_log('#appEl not found');
        return;
    }

    const signupEl = document.querySelector('#signup');
    if (!signupEl) {
        debug_log('#signup not found');
        return;
    }
    signupEl.addEventListener('click', evt => {
        chrome.tabs.create({url: 'http://localhost:8000/signup.htm'});
    });

    const forgotEl = document.querySelector('#forgot');
    if (!forgotEl) {
        debug_log('#forgot not found');
        return;
    }
    forgotEl.addEventListener('click', evt => {
        chrome.tabs.create({url: 'http://localhost:8000/forgot.htm'});
    });

    loginBtnEl.addEventListener('click', (evt) => {
        debug_log('clicked login');
        errorMsgEl.innerHTML = '';

        evt.preventDefault();
        const token = login(emailEl.value, passwordEl.value)
            .then(data => {
                if (token !== false) {
                    debug_log(`logged with token ${data.token}`);
                    chrome.storage.local.set({ token: data.token })
                        .catch(res => {
                            errorMsgEl.innerHTML = `could update token`;
                        })
                        .then(_ => {
                            signinEl.classList.add("hidden");
                            appEl.classList.remove("hidden");
                            init_app(data.token);
                        })
                    return;
                }
            })
            .catch(err => {
                // handle error
                debug_log(`could not login: ${err}`);
                errorMsgEl.innerHTML = `could not login: ${err}`;
            });
    });
}

async function is_logged() {
    debug_log('is_logged');
    const res = await chrome.storage.local.get('token');
    debug_log(res);
    if (typeof res.token !== "string" || res.token === "") {
        debug_log('not logged');
        return false;
    }
    debug_log(`logged with token ${res.token}`);
    return res.token;
}

window.addEventListener('load', async () => {
    const signinEl = document.querySelector('#signin');
    if (!signinEl) {
        debug_log('#signin not found');
        return;
    }

    const appEl = document.querySelector('#app');
    if (!appEl) {
        debug_log('#app not found');
        return;
    }

    const logoutEl = document.querySelector('#logout');
    if (!logoutEl) {
        debug_log('#logout not found');
        return;
    }
    logoutEl.addEventListener('click', async (evt) => {
        evt.preventDefault();
        await chrome.storage.local.remove('token');
        window.close();
    });

    debug_log('testing if already logged');

    const token = await is_logged();
    if (!token) {
        signinEl.classList.remove('hidden');
        init_login();
        return;
    }

    // todo: ping to see if we are still connected
    appEl.classList.remove('hidden');
    init_app(token);
});
