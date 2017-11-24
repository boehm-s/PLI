const fakeLoad = (elem, n = 100, t = 75) => {
    elem.innerHTML = `${100 -n}%`;
    return n !== 0
	? setTimeout(_ => fakeLoad(elem, n - 1, t), t)
	: destroySmooth(parentNode(elem, 2));
};

const hideCam = e => {
    const camView = document.getElementById('cam-view');

    camView.style.opacity = 0;
    camView.style.zIndex = -1;
    setTimeout(_ => {
	camView.style.opacity = 1;
	camView.classList.remove('cam-view-display');
    }, 400);
};

const showCam = (container, camImgPath) => {
    const camView = document.getElementById('cam-view');

    camView.classList.add('cam-view-display');
    setInterval(function displayCam() {
	container.style.background = `url(${camImgPath}) #000`;
	container.style.backgroundSize = 'cover';
    }, 16);
};

/* dom manip */

const parentNode = (el, n) => 0 === n ? el : parentNode(el.parentNode, n - 1);
const destroyElem = elem => elem.parentNode.removeChild(elem);
const destroySmooth = elem => {
    elem.style.transition = '0.4s';
    elem.style.opacity = '0';
    setTimeout(_ => destroyElem(elem), 400);
};

export {fakeLoad, hideCam, showCam, destroyElem, destroySmooth, parentNode}
