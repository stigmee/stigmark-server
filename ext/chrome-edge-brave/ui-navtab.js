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

import { debug_log } from "./debug.js";

let nav_instance = null;

export function init_navtab(msg_ctrl) {
    debug_log('init_navtab');

    if (nav_instance === null) {
        const instance = {};

        instance.nav = document.querySelector('nav');
        if (!instance.nav) {
            debug_log('#signup not found');
            return false;
        }

        nav_instance = instance;
    }

    const page_ctrls = {};
    const nav_ctrl = {
        add_page: (page_name, page_ctrl, show_nav, button_name) => {
            debug_log(`adding page ${page_name}`);
            const ctrl = page_ctrl(nav_ctrl, msg_ctrl);
            for (let n in ctrl) {
                debug_log(` # ${page_name}.${n}`);
            }
            if (ctrl === false) {
                debug_log('failed');
                return false;
            }
            page_ctrls[page_name] = {
                ctrl: ctrl,
                show_nav: show_nav,
            };
            if (button_name) {
                page_ctrls[page_name].button = nav_ctrl.add_button(button_name, evt => {
                    nav_ctrl.switch_to(page_name);
                });
            }
            return true;
        },

        add_button: (button_name, on_click) => {
            const button = document.createElement('button');
            button.innerHTML = button_name;
            nav_instance.nav.appendChild(button);
            if (on_click) {
                button.addEventListener('click', on_click);
            } else {
                button.className = 'disabled';
            }
            return button;
        },

        switch_to: (page_name, param) => {
            const c = page_ctrls[page_name];
            if (!c) {
                debug_log(` # could not find page ${page_name}`);
                return;
            }
            for (let c0 in page_ctrls) {
                page_ctrls[c0].ctrl.hide();
                if (page_ctrls[c0].button) {
                    page_ctrls[c0].button.classList.remove('selected');
                }
            }
            if (c.show_nav) {
                debug_log('showing nav');
                nav_instance.nav.classList.remove('hidden');
            } else {
                debug_log('hidding nav');
                nav_instance.nav.classList.add('hidden');
            }
            if (c.button) {
                c.button.classList.add('selected');
            }
            c.ctrl.show(param);
        }
    };
    return nav_ctrl;
}
