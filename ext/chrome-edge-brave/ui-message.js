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

let message_instance = null;

export function init_message() {
    if (message_instance === null) {
        const instance = {};

        instance.box = document.querySelector('#message-box');
        if (!instance.box) {
            debug_log('#message-box not found');
            return false;
        }

        message_instance = instance;
    }

    return {
        alert: (msg) => {
            message_instance.box.innerHTML = msg;
            message_instance.box.className = 'alert';
        },
        info: (msg) => {
            message_instance.box.innerHTML = msg;
            message_instance.box.className = 'info';
        },
        close: () => {
            message_instance.box.innerHTML = '';
            message_instance.box.className = 'hidden';
        }
    };
}
