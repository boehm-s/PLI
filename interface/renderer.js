// All of the Node.js APIs are available in this process.

mapboxgl.accessToken =  'pk.eyJ1IjoiYm9laG0tcyIsImEiOiJjajl1N2pkdm43MDFiMzNwYTByazg4cm9hIn0.SLPzRAQSB7Ee739X9N8eAQ';

const map = new mapboxgl.Map({
    container: 'map',
    style: 'mapbox://styles/mapbox/dark-v9',
    center: [-74.50, 40],
    zoom: 9
});
