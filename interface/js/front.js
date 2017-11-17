document.addEventListener('DOMContentLoaded', e => {
    const loadingPercent = document.getElementById('loading-percent');

    fakeLoad(loadingPercent, 100, 20);
});

const fakeLoad = (elem, n = 100, t = 75) => {
    elem.innerHTML = `${100 -n}%`;
    return n !== 0
	? setTimeout(_ => fakeLoad(elem, n - 1, t), t)
	: destroySmooth(parentNode(elem, 2));
};

const parentNode = (el, n) => 0 === n ? el : parentNode(el.parentNode, n - 1);
const destroyElem = elem => elem.parentNode.removeChild(elem);
const destroySmooth = elem => {
    elem.style.transition = '0.4s';
    elem.style.opacity = '0';
    setTimeout(_ => destroyElem(elem), 400);
};
