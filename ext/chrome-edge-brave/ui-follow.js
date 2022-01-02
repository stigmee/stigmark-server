// 
//  Stigmee: A 3D browser and decentralized social network.
//  Copyright 2021-2022 Philippe Anel <zexigh@gmail.com>
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

import { is_logged, api_follow } from "./api-stigmark.js";
import { debug_log } from "./debug.js";

let follow_instance = null;

export function init_follow_page(page_nav, msg_ctrl) {
    debug_log('init_follow_page');

    if (follow_instance === null) {
        const instance = {};

        instance.page = document.querySelector('#follow-page');
        if (!instance.page) {
            debug_log('#follow-page not found');
            return;
        }

        // ------------------------------------------------

        instance.mailInputEl = document.querySelector('#follow-mail-input');
        if (!instance.mailInputEl) {
            debug_log('#signup-mail-input not found');
            return false;
        }

        // ------------------------------------------------

        instance.followBtnEl = document.querySelector('#follow-btn');
        if (!instance.followBtnEl) {
            debug_log('#follow-btn not found');
            return false;
        }
        instance.followBtnEl.addEventListener('click', evt => {
            debug_log('clicked follow');
            evt.preventDefault();

            instance.mailInputEl.classList.remove('error');
            msg_ctrl.close();

            const mail = instance.mailInputEl.value;
            if ((typeof mail !== "string") || mail.trim() == "") {
                msg_ctrl.alert(`invalid mail address`);
                instance.mailInputEl.classList.add('error');
                return;
            }

            is_logged()
                .then(_ => {
                    debug_log('logged: call api_follow');
                    api_follow(mail)
                        .then(data => {
                            debug_log(`subscribed`);
                        })
                        .catch(err => {
                            // handle error
                            debug_log(`could not subscribe: ${err}`);
                            msg_ctrl.alert(`could not subscribe: ${err}`);
                        });
                })
                .catch(err => {
                    page_nav.switch_to('login');
                })
                ;
        });

        // ------------------------------------------------

        follow_instance = instance;
    }

    debug_log('returning "follow" controler');
    return {
        show: function () {
            debug_log('showing "follow" page');
            msg_ctrl.close();
            follow_instance.page.classList.remove('hidden');
        },

        hide: function () {
            debug_log('hidding "follow" page');
            follow_instance.page.classList.add('hidden');
        },
    }
}
