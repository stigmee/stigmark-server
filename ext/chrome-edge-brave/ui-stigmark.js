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

import { get_all_tabs } from "./chrome-ext.js";
import { is_logged, api_add_collection } from "./api-stigmark.js";
import { debug_log } from "./debug.js";

let stigmark_instance = null;

function update_tabs() {
    debug_log("get_all_tabs:");
    get_all_tabs()
        .then(tabs => {
            stigmark_instance.listEl.innerHTML = '';
            stigmark_instance.selected_only_div.className = 'hidden';
            stigmark_instance.entries = [];
            tabs.forEach(tab => {
                debug_log(`tab: "${tab.title}" selected=${tab.highlighted ? 'true' : 'false'}`);
                if (tab.highlighted === true && tab.active === false) {
                    debug_log(' # show checkbox');
                    stigmark_instance.selected_only_div.className = '';
                }
                const url = tab.url;
                if (!url.startsWith('http://') && !url.startsWith('https://'))
                    return;
                const entryEl = document.createElement('li');
                const inputEl = document.createElement('input');
                inputEl.type = 'checkbox';
                inputEl.checked = true;
                const titleEl = document.createElement('span');
                titleEl.innerText = tab.title;
                entryEl.appendChild(inputEl);
                entryEl.appendChild(titleEl);

                stigmark_instance.listEl.appendChild(entryEl);
                stigmark_instance.entries.push({
                    input: inputEl,
                    tab: tab,
                });
            });
        })
        .catch(err => {
            debug_log(`could not get all tabs ${err}`);
        })
        ;
}

export function init_stigmark_page(page_nav, msg_ctrl) {
    debug_log('init_stigmark_page');

    if (stigmark_instance == null) {
        const instance = {};

        instance.page = document.querySelector('#stigmark-page');
        if (!instance.page) {
            debug_log('#stigmark-page not found');
            return false;
        }

        instance.selected_only_div = document.querySelector('#selected-only-div');
        if (!instance.selected_only_div) {
            debug_log('#selected-only-div not found');
            return false;
        }

        instance.listEl = document.querySelector('#stigmark-page ul');
        if (!instance.listEl) {
            debug_log('#stigmark-page ul not found');
            return 0;
        }

        instance.keywordsEl = document.querySelector('textarea');
        if (!instance.keywordsEl) {
            debug_log('textarea not found');
            return 0;
        }

        instance.selectedOnlyCheckboxEl = document.querySelector('#selected-only-checkbox');
        if (!instance.selectedOnlyCheckboxEl) {
            debug_log('#selected-only-checkbox not found');
            return 0;
        }

        instance.sendBtnEl = document.querySelector('#send-btn');
        if (!instance.sendBtnEl) {
            debug_log('#send-btn not found');
            return 0;
        }

        instance.entries = [];

        instance.selectedOnlyCheckboxEl.addEventListener('change', () => {
            instance.entries.forEach(entry => {
                entry.input.checked = entry.tab.highlighted || !instance.selectedOnlyCheckboxEl.checked;
            });
        });

        instance.sendBtnEl.addEventListener('click', () => {
            debug_log('clicked send');
            msg_ctrl.close();

            const urls = [];
            instance.entries.forEach(entry => {
                if (entry.input.checked)
                    urls.push(entry.tab.url);
            });

            const keywords = instance.keywordsEl.value.split(/[ \t]*,[ \t]*/g)
            is_logged()
                .then(_ => {
                    debug_log('logged: call api_add_collection');
                    api_add_collection(urls, keywords)
                        .then(status => {
                            if (status >= 200 && status < 300) {
                                msg_ctrl.info('collection added');
                                window.setTimeout(_ => {
                                    window.close();
                                }, 1000);
                                return;
                            }
                            msg_ctrl.alert(`add collection with status=${status}`);
                        })
                        .catch(err => {
                            msg_ctrl.alert(`add collection failed: ${err}`);
                        })
                        ;
                })
                .catch(err => {
                    page_nav.switch_to('login');
                })
                ;
        });

        stigmark_instance = instance;
    }

    debug_log('returning "stigmark" controler');
    return {
        show: function () {
            debug_log('showing "stigmark" page');
            msg_ctrl.close();
            update_tabs();
            stigmark_instance.page.classList.remove('hidden');
        },

        hide: function () {
            debug_log('hidding "stigmark" page');
            stigmark_instance.page.classList.add('hidden');
        },
    };
}
