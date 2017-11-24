import {fakeLoad, hideCam, showCam, destroyElem, destroySmooth, parentNode} from './front';

mapboxgl.accessToken =  'pk.eyJ1IjoiYm9laG0tcyIsImEiOiJjajl1N2pkdm43MDFiMzNwYTByazg4cm9hIn0.SLPzRAQSB7Ee739X9N8eAQ';

const map = new mapboxgl.Map({
    container: 'map',
    style: 'mapbox://styles/mapbox/dark-v9',
    center: [2.346991, 48.854435],
    zoom: 11
});

const buildMarkers = geojson =>
      geojson.features.forEach(marker => {
	  const el = document.createElement('div');

	  el.classList.add('marker');
	  el.dataset.description = marker.properties.description;
	  new mapboxgl.Marker(el)
	      .setLngLat(marker.geometry.coordinates)
	      .addTo(map);
      });

const buildCamList = geojson => {
    const el = document.createElement('div');

    el.classList.add('cam-list');
    geojson.features.forEach(camera => {
	const cam = document.createElement('div');

	cam.onclick = e => goToCam(camera.geometry.coordinates);
	cam.classList.add('cam');
	cam.innerHTML = `<span> ${camera.properties.description}</span>`;
	el.appendChild(cam);
    });

    const mapElem = document.getElementById('map');
    mapElem.prepend(el);
};

const goToCam = coordinates => {
    map.flyTo({
	center: [
	    coordinates[0],
	    coordinates[1]
	],
	zoom: 15
    });
};

const addThreat = (marker, threatInfo) => {
    marker.classList.add('threat');
    marker.style.zIndex = 999;
    marker.onclick = e => {
	const overflowMarker = marker.cloneNode(true);

	overflowMarker.classList.add('overflowMarker');
	overflowMarker.style.transform = 'initial';
	marker.style.cursor = 'default';
	marker.appendChild(overflowMarker);
	marker.appendChild(threatDisplay(threatInfo));
	marker.onclick = e => {
	    if (Array.from(e.target.classList).includes('cam-title')) {
		const camContainer = document.getElementById('cam-view');
		const camUrl = e.target.dataset.camurl;

		showCam(camContainer, camUrl);
	    } else {
		removeThreat(marker);
	    }
	};
    };
};
const removeThreat = marker => {
    while (marker.firstChild)
	marker.removeChild(marker.firstChild);
    initThreat();
};

const threatDisplay = threatInfo => {
    const div = document.createElement('div');

    div.classList.add('threatContainer');
    div.innerHTML = `
    <div class="status-info threat-info">
      <div class="line _1">
	<span class="text-info"> NEW THREAT : ${threatInfo.threatType} </span>
	<span class="text-info right-info">CAM ${threatInfo.camId} </span>
      </div>
      <div class="line _2">
	<span class="text-info"> TARGET : ${threatInfo.isIdentified}</span>
      </div>
    </div>
    <div class="status-info threat-detail">
      <div class="text-info data-acquisition">
        DATA ACQUISITION ... 100%
      </div>
      <div class="right-left-container">
        <div class="left-part suspect-img" style="background: url('${threatInfo.threatImg}') center center no-repeat">
        </div>
        <div class="right-part">
          <div> PROBABILITY RATE </div>
          <div class="probability cl-red"> ${threatInfo.probability}%</div>
          <div class="weapon"> WEAPON : <span class="cl-red">${threatInfo.weapon}</span></div>
          <div class="suspectFullName"> ${threatInfo.suspectFullName}</div>
          <div class="suspectDetails"> ${threatInfo.suspectAge}y, ${threatInfo.suspectSize}</div>
        </div>
      </div>
      <div class="cam-title" data-camurl="${threatInfo.camUrl}" > CAMERA NUMBER ${threatInfo.camId}</div>
    </div>
`;

    return div;
};

const initThreat = () => {
    const markers = document.getElementsByClassName('marker');
    const choosenMarker = Array.from(markers)
	  .filter(marker => 'Camera 2' === marker.dataset.description)[0];
    const camId = choosenMarker.dataset.description.match(/[0-9]/)[0];

    addThreat(choosenMarker, {
	camId,
	camUrl: "./img/stream.jpg",
        threatType: 'THIEF',
	isIdentified: 'IDENTIFIED',
	threatImg: './img/threat.jpg',
	probability: '70',
	weapon: 'KNIFE',
	suspectFullName: 'Mickal Crocks',
	suspectAge: 27,
	suspectSize: '1m85'
    });
};

export {map, buildMarkers, buildCamList, goToCam, addThreat, removeThreat, threatDisplay, initThreat};
