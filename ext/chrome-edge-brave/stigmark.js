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

import { requestUrl } from "./urls.js";
import { debug_log } from "./debug.js";
// import { add_bookmarks } from "./chrome-ext.js";

export function api_send_urls_and_keywords(token, urls, keywords) {
    debug_log(`sending urls=${JSON.stringify(urls)} and keywords=${JSON.stringify(keywords)}`);
    return new Promise((resolve, reject) => {
        const body = { urls: urls, keys: keywords, token: token };
        const requestData = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json; charset=utf-8',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify(body),
        };
        fetch(requestUrl, requestData)
            .then(res => {
                if (res.status >= 200 && res.status < 300) {
                    res.json()
                        .then(data => {
                            debug_log(data);
                            // add_bookmarks(urls, keywords);
                        })
                        .catch(err => {
                            debug_log(`could not decode json: ${err}`);
                        })
                        ;
                }
                resolve(res.status);
            })
            .catch(exc => {
                debug_log(`urls+keys failed with exception ${exc.message}`);
                reject(exc.message);
            })
            ;
    });
}
