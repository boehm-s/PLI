// All of the Node.js APIs are available in this process.

import {fakeLoad,
	hideCam} from './js/front';
import {map,
	buildCamList,
	buildMarkers,
	initThreat} from './js/map-utils';

import cams from './cams';
import geojson from './geojson';

map.on('load', _ => {
    buildMarkers(geojson);
    setTimeout(_ => {
	initThreat();
	buildCamList(geojson);

    }, 2000);
});


document.addEventListener('DOMContentLoaded', e => {
    const loadingPercent = document.getElementById('loading-percent');
    const backToMap = document.getElementById('back-to-map');

    fakeLoad(loadingPercent, 100, 20);
    backToMap.onclick = hideCam;
});
