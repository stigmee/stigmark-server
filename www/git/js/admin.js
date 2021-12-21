const getQueryParams = (params, url) => {
    let href = url;
    // this is an expression to get query strings
    let regexp = new RegExp('[?&]' + params + '=([^&#]*)', 'i');
    let qString = regexp.exec(href);
    return qString ? qString[1] : null;
};

function makeLink(value) {
    return `<a target="_blank" href="${value}">${value}</a>`;
}

window.addEventListener('load', evt => {
    const sectionEl = document.querySelector('section');
    if (!sectionEl) {
        alert('could not find <section>');
        return;
    }
});
