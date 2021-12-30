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

import { loginUrl, signupUrl, stigmersUrl, stigmarksUrl } from "./config.js";
import { debug_log } from "./debug.js";
import { serverAddr, cookieName } from "./config.js";

export function is_logged() {
    debug_log('is_logged');
    return new Promise((resolve, reject) => {
        chrome.cookies.get({ url: serverAddr, name: cookieName })
            .then(_ => {
                debug_log(` # found cookie`);
                resolve();
            })
            .catch(_ => {
                debug_log(` # cookie not found`);
                reject(err);
            })
            ;
    });
}

export function api_login(mail, passwd) {
    debug_log(`api-login: ${mail}`);
    return new Promise((resolve, reject) => {
        debug_log(`api-login: in-promise`);
        const body = {
            mail: mail,
            pass: passwd,
        };
        const loginData = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json; charset=utf-8' },
            body: JSON.stringify(body),
        };
        debug_log('api-login: sending login request');
        fetch(loginUrl, loginData)
            .then(res => {
                debug_log(`api-login: fetch returned ${res.status}`);
                if (res.status >= 200 && res.status < 300) {
                    res.json()
                        .then(data => {
                            debug_log(`api-login: fetch data`);
                            resolve(data);
                        })
                        .catch(err => {
                            debug_log(`api-login: could not decode json: ${err}`)
                            reject(err);
                        })
                        ;
                    return true;
                }
                debug_log(`api-login: fetch failed`);
                reject(`api-login: fetch failed`);
            })
            .catch(err => {
                debug_log(`api-login: fetch crashed with ${err}`);
                reject(`api-login: fetch crashed with ${err}`);
            })
            ;
    });
}

export function api_signup(name, mail, pass) {
    debug_log(`api-signup: ${mail}`);
    return new Promise((resolve, reject) => {
        debug_log(`api-signup: in-promise`);
        const body = JSON.stringify({
            user: name,
            mail: mail,
            pass: pass,
        });
        const headers = new Headers();
        headers.append('Content-Type', 'application/json');
        const signupData = {
            method: 'POST',
            cache: 'no-cache',
            headers: headers,
            body: body,
        };
        const request = fetch(signupUrl, signupData);
        request
            .then(res => {
                if (res.status >= 200 && res.status < 300) {
                    res.json()
                        .then(data => {
                            debug_log(`api-signup: fetch data`);
                            resolve(data);
                        })
                        .catch(err => {
                            debug_log(`api-signup: could not decode json: ${err}`)
                            reject(err);
                        });
                    return;
                }
                if (res.status == 409) {
                    // Conflict
                    debug_log(`api-signup: user already registered`);
                    reject(`already registered`);
                    return;
                }
                debug_log(`api-signup: fetch failed with ${res.status}`);
                reject(`api-signup: ${res.status}`);
            })
            .catch(err => {
                debug_log(`api-signup: fetch crashed with ${err}`);
                reject(`api-signup: fetch crashed with ${err}`);
            })
            ;
    });
}

export function api_follow(mail) {
    debug_log(`api-follow: ${mail}`);
    return new Promise((resolve, reject) => {
        debug_log(`api-follow: in-promise`);
        const body = JSON.stringify({
            user_mail: mail,
        });
        const followData = {
            method: 'POST',
            cache: 'no-cache',
            headers: {
                'Content-Type': 'application/json; charset=utf-8',
            },
            body: body,
        };
        const request = fetch(stigmersUrl, followData);
        request
            .then(res => {
                if (res.status >= 200 && res.status < 300) {
                    res.json()
                        .then(data => {
                            debug_log(`api-follow: fetch data`);
                            resolve(data);
                        })
                        .catch(err => {
                            debug_log(`api-follow: could not decode json: ${err}`)
                            reject(err);
                        });
                    return;
                }
                if (res.status == 404) {
                    // Conflict
                    debug_log(`api-follow: user already subscribed`);
                    reject(`stigmer not found`);
                    return;
                }
                if (res.status == 409) {
                    // Conflict
                    debug_log(`api-follow: user already subscribed`);
                    reject(`already subscribed`);
                    return;
                }
                debug_log(`api-follow: fetch failed with ${res.status}`);
                reject(`api-follow: ${res.status}`);
            })
            .catch(err => {
                debug_log(`api-follow: fetch crashed with ${err}`);
                reject(`api-follow: fetch crashed with ${err}`);
            })
            ;
    });
}

export function api_add_collection(urls, keywords) {
    debug_log(`sending urls=${JSON.stringify(urls)} and keywords=${JSON.stringify(keywords)}`);
    return new Promise((resolve, reject) => {
        const body = { urls: urls, keys: keywords };
        const requestData = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json; charset=utf-8',
            },
            body: JSON.stringify(body),
        };
        fetch(stigmarksUrl, requestData)
            .then(res => {
                if (res.status >= 200 && res.status < 300) {
                    res.json()
                        .then(data => {
                            debug_log(data);
                            // add_bookmarks(urls, keywords);
                        })
                        .catch(err => {
                            debug_log(`could not decode json: ${err}`);
                        })
                        ;
                }
                resolve(res.status);
            })
            .catch(exc => {
                debug_log(`urls+keys failed with exception ${exc.message}`);
                reject(exc.message);
            })
            ;
    });
}
