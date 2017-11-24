const GeoJSONMarker = geojson => {
    const el = document.createElement('div');

    el.classList.add('marker');
    el.dataset.description = geojson.properties.description;

    return map => new mapboxgl.Marker(el)
    	      .setLngLat(geojson.geometry.coordinates)
    	      .addTo(map);
};

const cameraGeoJSON = [];
const cameraMarkers = cameraGeoJSON.map(cameraGeoJSON => GeoJSONMarker(cameraGeoJSON));

cameraMarkers.forEach(marker => marker(map));
