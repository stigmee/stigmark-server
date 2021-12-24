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
import { init_message } from "./ui-message.js";
import { init_login_page } from "./ui-login.js";
import { init_signup_page } from "./ui-signup.js";
import { init_forgot_page } from "./ui-forgot.js";
import { init_navtab } from "./ui-navtab.js";
import { init_stigmark_page } from "./ui-stigmark.js";
import { init_follow_page } from "./ui-follow.js";
import { init_search_page } from "./ui-search.js";
import { is_logged } from "./api-stigmark.js";
import { storage_remove } from "./chrome-ext.js";

window.addEventListener('load', () => {
    debug_log('window loaded');

    const msg_ctrl = init_message();
    const nav_ctrl = init_navtab(msg_ctrl);
    if (nav_ctrl.add_page('login', init_login_page, false) === false) {
        msg_ctrl.alert('could not initialize page "login"');
        return;
    }
    if (nav_ctrl.add_page('signup', init_signup_page, false) === false) {
        msg_ctrl.alert('could not initialize page "signup"');
        return;
    }
    if (nav_ctrl.add_page('forgot', init_forgot_page, false) === false) {
        msg_ctrl.alert('could not initialize page "forgot"');
        return;
    }
    if (nav_ctrl.add_page('search', init_search_page, true, 'search') === false) {
        msg_ctrl.alert('could not initialize page "search"');
        return;
    }
    if (nav_ctrl.add_page('stigmark', init_stigmark_page, true, 'share') === false) {
        msg_ctrl.alert('could not initialize page "share"');
        return;
    }
    if (nav_ctrl.add_page('follow', init_follow_page, true, 'follow') === false) {
        msg_ctrl.alert('could not initialize page "follow"');
        return;
    }
    if (nav_ctrl.add_button('news') === false) {
        msg_ctrl.alert('could not initialize button "news"');
        return;
    }
    if (nav_ctrl.add_button('config') === false) {
        msg_ctrl.alert('could not initialize button "config"');
        return;
    }
    if (nav_ctrl.add_button('logout', evt => {
        evt.preventDefault();
        storage_remove('token')
            .then(_ => {
                nav_ctrl.switch_to('login');
                // window.close();
            })
            .catch(err => {
                debug_log(`could not logout: ${err}`);
            });
    }) == false) {
        msg_ctrl.alert('could not initialize button "logout"');
        return;
    }

    debug_log('testing if already logged');
    is_logged()
        .then(token => {
            nav_ctrl.switch_to('stigmark', token);
        })
        .catch(_ => {
            nav_ctrl.switch_to('login');
        })
        ;
});
