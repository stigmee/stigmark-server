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

let forgot_instance = null;

export function init_forgot_page(page_nav, msg_ctrl) {
    debug_log('init_forgot_page');

    if (forgot_instance === null) {
        const instance = {};

        instance.page = document.querySelector('#forgot-page');
        if (!instance.page) {
            debug_log('#forgot-page not found');
            return false;
        }

        // ------------------------------------------------

        instance.signupLinkEl = document.querySelector('#forgot-signup-link');
        if (!instance.signupLinkEl) {
            debug_log('#forgot-signup-link not found');
            return false;
        }
        instance.signupLinkEl.addEventListener('click', evt => {
            debug_log('opening signup page in new tab');
            evt.preventDefault();
            page_nav.switch_to('signup');
        });

        // ------------------------------------------------

        instance.mailInputEl = document.querySelector('#forgot-mail-input');
        if (!instance.mailInputEl) {
            debug_log('#forgot-mail-input not found');
            return false;
        }

        instance.recoverBtnEl = document.querySelector('#forgot-btn');
        if (!instance.recoverBtnEl) {
            debug_log('#forgot-btn not found');
            return false;
        }
        instance.recoverBtnEl.addEventListener('click', evt => {
            debug_log('clicked recover button');
            evt.preventDefault();
            msg_ctrl.alert(`recover not implemented yet`);
        });

        debug_log(instance);
        forgot_instance = instance;
    }

    debug_log('returning "forgot" controler');
    return {
        show: function() {
            debug_log('showing "forgot" page');
            msg_ctrl.close();
            forgot_instance.page.classList.remove('hidden');
        },

        hide: function() {
            debug_log('hidding "forgot" page');
            forgot_instance.page.classList.add('hidden');
        },
    };
}
