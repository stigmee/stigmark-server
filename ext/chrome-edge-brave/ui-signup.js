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
import { api_signup, api_is_logged } from "./api-stigmark.js";

let signup_instance = null;

export function init_signup_page(page_nav, msg_ctrl) {
    debug_log('init_signup_page');

    if (signup_instance === null) {
        const instance = {};

        instance.page = document.querySelector('#signup-page');
        if (!instance.page) {
            debug_log('#signup-page not found');
            return false;
        }

        // ------------------------------------------------

        instance.loginLinkEl = document.querySelector('#signup-login-link');
        if (!instance.loginLinkEl) {
            debug_log('#signup-login-link not found');
            return false;
        }
        instance.loginLinkEl.addEventListener('click', evt => {
            debug_log('opening login page in new tab');
            evt.preventDefault();
            page_nav.switch_to('login');
        });

        // ------------------------------------------------

        instance.nameInputEl = document.querySelector('#signup-name-input');
        if (!instance.nameInputEl) {
            debug_log('#signup-name-input not found');
            return false;
        }

        instance.mailInputEl = document.querySelector('#signup-mail-input');
        if (!instance.mailInputEl) {
            debug_log('#signup-mail-input not found');
            return false;
        }

        instance.pass1InputEl = document.querySelector('#signup-pass1-input');
        if (!instance.pass1InputEl) {
            debug_log('#signup-pass1-input not found');
            return false;
        }

        instance.pass2InputEl = document.querySelector('#signup-pass2-input');
        if (!instance.pass2InputEl) {
            debug_log('#signup-pass2-input not found');
            return false;
        }

        instance.signupBtnEl = document.querySelector('#signup-btn');
        if (!instance.signupBtnEl) {
            debug_log('#signup-btn not found');
            return false;
        }
        instance.signupBtnEl.addEventListener('click', evt => {
            debug_log('clicked signup');
            evt.preventDefault();

            msg_ctrl.close();
            instance.nameInputEl.classList.remove('error');
            instance.mailInputEl.classList.remove('error');
            instance.pass1InputEl.classList.remove('error');
            instance.pass2InputEl.classList.remove('error');

            const name = instance.nameInputEl.value;
            if ((typeof name !== "string") || name.trim() == "") {
                msg_ctrl.alert(`invalid name`);
                instance.nameInputEl.classList.add('error');
                return;
            }
            const mail = instance.mailInputEl.value;
            if ((typeof mail !== "string") || mail.trim() == "") {
                msg_ctrl.alert(`invalid mail address`);
                instance.mailInputEl.classList.add('error');
                return;
            }
            const pass1 = instance.pass1InputEl.value;
            if ((typeof pass1 !== "string") || pass1.trim() == "") {
                msg_ctrl.alert(`invalid password`);
                instance.pass1InputEl.classList.add('error');
                return;
            }
            const pass2 = instance.pass2InputEl.value;
            if ((typeof pass2 !== "string") || pass2.trim() == "") {
                msg_ctrl.alert(`invalid password`);
                instance.pass2InputEl.classList.add('error');
                return;
            }

            if (pass1 != pass2) {
                msg_ctrl.alert('different password');
                instance.pass2InputEl.classList.add('error');
                return;
            }
            api_signup(name, mail, pass1)
                .then(_ => {
                    api_is_logged()
                        .catch(_ => {
                            debug_log(`could not signup: cookie not found`);
                            msg_ctrl.alert(`could not signup: cookie not found`);
                        })
                        .then(_ => {
                            page_nav.switch_to('stigmark');
                        })
                })
                .catch(err => {
                    // handle error
                    debug_log(`could not signup: ${err}`);
                    msg_ctrl.alert(`could not signup: ${err}`);
                });
        });

        debug_log(instance);
        signup_instance = instance;
    }

    debug_log('returning "signup" controler');
    return {
        show: function() {
            debug_log('showing "signup" page');
            msg_ctrl.close();
            signup_instance.page.classList.remove('hidden');
        },

        hide: function() {
            debug_log('hidding "signup" page');
            signup_instance.page.classList.add('hidden');
        },
    };
}
