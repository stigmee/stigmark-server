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

// const serverAddr = 'http://localhost:8000';
const serverAddr = 'https://stigmark.badro.fr';

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
            'Content-Type': 'application/json; charset=utf-8'
        },
        body: JSON.stringify(body),
    };
    try {
        const res = await fetch(requestUrl, requestData);
        if (res.status >= 200 && res.status < 300) {
            debug_log('urls+keys sent');
            return true;
        }
        debug_log(`urls+keys failed with status ${res.status}`);
    }
    catch (e) {
        debug_log(`urls+keys failed with exception ${e.message}`);
    }
    return false;
}

async function init_app(token) {
    const listEl = document.querySelector('ul');
    if (!listEl)
        return 0;

    const keywordsEl = document.querySelector('textarea');
    if (!keywordsEl)
        return 0;

    const onlyInputEl = document.querySelector('#onlySelected');
    if (!onlyInputEl)
        return 0;

    const sendBtnEl = document.querySelector('#send');
    if (!sendBtnEl)
        return 0;

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
        const urls = [];
        entries.forEach(entry => {
            if (entry.input.checked)
                urls.push(entry.tab.url);
        });

        const keywords = keywordsEl.value.split(/[ \t]*,[ \t]*/g)
        await send_urls_and_keywords(token, urls, keywords);

        window.close();
    });
}

async function login(email, passwd) {
    const body = {
        email: email,
        passwd: passwd,
    };
    const loginData = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json; charset=utf-8' },
        body: JSON.stringify(body),
    };
    try {
        debug_log('sending login request');
        const res = await fetch(loginUrl, loginData);
        debug_log('fetch sent');
        debug_log(res);
        if (res.status >= 200 && res.status < 300) {
            debug_log('logged in');
            // TODO: extract token from answer
            return "foo";
        }
        debug_log(`fetch failed with ${res.status}`);
    }
    catch (e) {
        debug_log(`fetch failed with ${err}`);
    }
    return false;
}

async function init_login() {
    const emailEl = document.querySelector('#email');
    if (!emailEl)
        return 0;

    const passwordEl = document.querySelector('#password');
    if (!passwordEl)
        return 0;

    const guestBtnEl = document.querySelector('#guest');
    if (!guestBtnEl)
        return 0;

    const loginBtnEl = document.querySelector('#login');
    if (!loginBtnEl)
        return 0;

    const signinEl = document.querySelector('#signin');
    if (!signinEl)
        return;

    const appEl = document.querySelector('#app');
    if (!appEl)
        return;

    guestBtnEl.addEventListener('click', async (evt) => {
        debug_log('clicked guest');
        evt.preventDefault();
        const token = await login('', '');
        debug_log('token');
        debug_log(token);
        if (token !== false) {
            await chrome.storage.local.set({token: 'blah'});
            const res = await chrome.storage.local.get('token');
            debug_log(res);
            signinEl.classList.add("hidden");
            appEl.classList.remove("hidden");
            init_app(token);
            return;
        }
        // handle error
    });

    loginBtnEl.addEventListener('click', async (evt) => {
        debug_log('clicked login');
        evt.preventDefault();
        const token = await login(emailEl.value, passwordEl.value)
        if (token !== false) {
            signinEl.classList.add("hidden");
            appEl.classList.remove("hidden");
            init_app(token);
            return;
        }
        // handle error
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
