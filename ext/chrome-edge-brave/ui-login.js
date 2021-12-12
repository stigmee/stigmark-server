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
import { api_login } from "./token.js";
import { signupUrl, forgotUrl } from "./urls.js";
import { create_tab, storgage_set } from "./chrome-ext.js";

let login_instance = null;

export function init_login_page(page_nav, msg_ctrl) {
    debug_log('init_login_page');

    if (login_instance === null) {
        const instance = {};

        instance.page = document.querySelector('#signin-page');
        if (!instance.page) {
            debug_log('#signin-page not found');
            return false;
        }

        // ------------------------------------------------

        instance.signupLinkEl = document.querySelector('#signup');
        if (!instance.signupLinkEl) {
            debug_log('#signup not found');
            return false;
        }
        instance.signupLinkEl.addEventListener('click', evt => {
            debug_log('opening signup page in new tab');
            evt.preventDefault();
            create_tab(signupUrl);
        });

        instance.forgotLinkEl = document.querySelector('#forgot');
        if (!instance.forgotLinkEl) {
            debug_log('#forgot not found');
            return false;
        }
        instance.forgotLinkEl.addEventListener('click', evt => {
            debug_log('opening forgot page in new tab');
            evt.preventDefault();
            create_tab(forgotUrl);
        });

        // ------------------------------------------------

        instance.mailInputEl = document.querySelector('#mail-input');
        if (!instance.mailInputEl) {
            debug_log('#mail-input not found');
            return false;
        }

        instance.passInputEl = document.querySelector('#pass-input');
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
            msg_ctrl.close();

            evt.preventDefault();
            const token = api_login(instance.mailInputEl.value, instance.passInputEl.value)
                .then(data => {
                    debug_log(`logged with token ${data.token}`);
                    storgage_set({ token: data.token })
                        .catch(err => {
                            debug_log(`could update token ${err}`);
                            msg_ctrl.alert(`could update token`);
                        })
                        .then(_ => {
                            page_nav.switch_to('stigmark');
                        })
                })
                .catch(err => {
                    // handle error
                    debug_log(`could not login: ${err}`);
                    msg_ctrl.alert(`could not login`);
                });
        });

        debug_log(instance);
        login_instance = instance;
    }

    debug_log('returning "login" controler');
    return {
        show: function() {
            debug_log('showing "login" page');
            login_instance.page.classList.remove('hidden');
        },

        hide: function() {
            debug_log('hidding "login" page');
            login_instance.page.classList.add('hidden');
        },
    };
}
