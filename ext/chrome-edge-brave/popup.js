const requestUrl = 'https://stigmark.badro.fr/api/v1/stigmarks';

const port = chrome.runtime.connect({ name: 'stigmark' });

window.addEventListener('load', async () => {
    const listEl = document.querySelector('ul');
    if (!listEl)
        return 0;

    const queryOptions = { currentWindow: true };
    const tabs = await chrome.tabs.query(queryOptions);
    // port.postMessage(tabs);
    const entries = [];

    tabs.forEach(tab => {
        const entryEl = document.createElement('li');
        const inputEl = document.createElement('input');
        inputEl.type = 'checkbox';
        inputEl.checked = true;
        const titleEl = document.createElement('span');
        titleEl.innerText = tab.title;
        // const urlEl = document.createElement('em');
        // urlEl.innerText = tab.url;
        entryEl.appendChild(inputEl);
        entryEl.appendChild(titleEl);
        // entryEl.appendChild(urlEl);
        listEl.appendChild(entryEl);

        entries.push({
            input: inputEl,
            tab: tab,
        });
    });

    const onlyInputEl = document.querySelector('#onlySelected');
    if (onlyInputEl) {
        onlyInputEl.addEventListener('change', () => {
            entries.forEach(entry => {
                entry.input.checked = entry.tab.highlighted || !onlyInputEl.checked;
            });
        });
    }

    const keywordsEl = document.querySelector('textarea');
    if (keywordsEl) {

    }

    const sendBtnEl = document.querySelector('button');
    if (sendBtnEl) {
        sendBtnEl.addEventListener('click', async () => {
            const urls = [];
            entries.forEach(entry => {
                if (entry.input.checked)
                    urls.push(entry.tab.url);
            });

            const keys = keywordsEl.value.split(/[ \t]*,[ \t]*/g)
            const body = { urls: urls, keys: keys };

            port.postMessage({ posting: body });
            const requestData = {
                method: 'POST',
                headers: { 'Content-Type': 'application/json; charset=utf-8' },
                body: JSON.stringify(body),
            };
            fetch(requestUrl, requestData)
                .then(res => {
                    if (res.status >= 200 && res.status < 300) {
                        port.postMessage('sent');
                    }
                })
                .catch(err => {
                    port.postMessage(`fetch failed with ${err}`);
                });
        });
    }
});
