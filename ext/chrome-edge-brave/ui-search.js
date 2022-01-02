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

let search_instance = null;

export function init_search_page(page_nav, msg_ctrl) {
    debug_log('init_search_page');

    if (search_instance === null) {
        const instance = {};

        instance.page = document.querySelector('#search-page');
        if (!instance.page) {
            debug_log('#search-page not found');
            return;
        }

        search_instance = instance;
    }
 
    debug_log('returning "search" controler');
    return {
        show: function() {
            debug_log('showing "search" page');
            msg_ctrl.close();
            search_instance.page.classList.remove('hidden');
        },

        hide: function() {
            debug_log('hidding "search" page');
            search_instance.page.classList.add('hidden');
        },
    }
}
