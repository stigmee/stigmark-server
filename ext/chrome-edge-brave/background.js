chrome.runtime.onConnect.addListener((port) => {
    console.log(port);
    port.onMessage.addListener((msg) => {
        console.log(msg);
    });
});
