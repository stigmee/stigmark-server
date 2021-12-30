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

import { debug_log } from "./debug.js";
import { api_login } from "./api-stigmark.js";
import { serverAddr, cookieName } from "./config.js";

let login_instance = null;

export function init_login_page(page_nav, msg_ctrl) {
    debug_log('init_login_page 2');

    if (login_instance === null) {
        const instance = {};

        instance.page = document.querySelector('#login-page');
        if (!instance.page) {
            debug_log('#login-page not found');
            return false;
        }

        // ------------------------------------------------

        instance.signupLinkEl = document.querySelector('#login-signup-link');
        if (!instance.signupLinkEl) {
            debug_log('#login-signup-link not found');
            return false;
        }
        instance.signupLinkEl.addEventListener('click', evt => {
            debug_log('opening signup page');
            evt.preventDefault();
            page_nav.switch_to('signup');
        });

        instance.forgotLinkEl = document.querySelector('#login-forgot-link');
        if (!instance.forgotLinkEl) {
            debug_log('#login-forgot-link not found');
            return false;
        }
        instance.forgotLinkEl.addEventListener('click', evt => {
            debug_log('opening forgot page');
            evt.preventDefault();
            page_nav.switch_to('forgot');
        });

        // ------------------------------------------------

        instance.mailInputEl = document.querySelector('#login-mail-input');
        if (!instance.mailInputEl) {
            debug_log('#login-mail-input not found');
            return false;
        }

        instance.passInputEl = document.querySelector('#login-pass-input');
        if (!instance.passInputEl) {
            debug_log('#pass-input not found');
            return false;
        }

        instance.loginBtnEl = document.querySelector('#login-btn');
        if (!instance.loginBtnEl) {
            debug_log('#login-btn not found');
            return false;
        }
        instance.loginBtnEl.addEventListener('click', evt => {
            debug_log('clicked login');
            evt.preventDefault();

            msg_ctrl.close();
            instance.mailInputEl.classList.remove('error');
            instance.passInputEl.classList.remove('error');

            const mail = instance.mailInputEl.value;
            if ((typeof mail !== "string") || mail.trim() == "") {
                msg_ctrl.alert(`invalid mail address`);
                instance.mailInputEl.classList.add('error');
                return;
            }
            const pass = instance.passInputEl.value;
            if ((typeof pass !== "string") || pass.trim() == "") {
                msg_ctrl.alert(`invalid password`);
                instance.passInputEl.classList.add('error');
                return;
            }

            api_login(mail, pass)
                .then(_ => {
                    chrome.cookies.get({ url: serverAddr, name: cookieName })
                        .catch(_ => {
                            debug_log(`could not login: cookie not found`);
                            msg_ctrl.alert(`could not login: cookie not found`);
                        })
                        .then(_ => {
                            page_nav.switch_to('stigmark');
                        })
                })
                .catch(err => {
                    // handle error
                    debug_log(`could not login: ${err}`);
                    msg_ctrl.alert(`could not login: ${err}`);
                });
        });

        debug_log(instance);
        login_instance = instance;
    }

    debug_log('returning "login" controler');
    return {
        show: function() {
            debug_log('showing "login" page');
            msg_ctrl.close();
            login_instance.page.classList.remove('hidden');
        },

        hide: function() {
            debug_log('hidding "login" page');
            login_instance.page.classList.add('hidden');
        },
    };
}
