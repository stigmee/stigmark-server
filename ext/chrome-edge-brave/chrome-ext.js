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

export function get_all_tabs() {
    const queryOptions = { currentWindow: true };
    return chrome.tabs.query(queryOptions);
}

export function create_tab(url) {
    return chrome.tabs.create({url: url});
}

export function storgage_set(object) {
    return chrome.storage.local.set(object);
}

export function storage_remove(name) {
    return chrome.storage.local.remove(name);
}

/*
export function add_bookmarks(urls, keywords) {
    let found = 0;
    chrome.bookmarks.get("stigmarks", results => {
        console.log(results);
        if (results) {
            results.forEach(result => {
                console.log(result);
                found++;
            });
        }
    });
}
*/
