import request from 'request';


const getCamIds = url => new Promise((resolve, reject) => {
    request(url, (err, res, body) => {
	const doc = new window.DOMParser().parseFromString(body, "text/html");
	const links = Array.from(doc.getElementsByClassName('thumbnail-item'))
		  .map(el => {
		      const href = el.getElementsByTagName('a')[0].outerHTML.match(/href\="((?:\\.|[^"\\])*)"/)[1];
		      const baseurl = url.split('/').filter((_, i) => i < 3).join('/');
		      return baseurl + href;
		      });
	try {
	    var coordinates = links.map(async (link) =>  await getCoordinates(link));
	} catch (e) {
	    return reject(e);
	}

	return resolve(coordinates);
    });
});

const getCoordinates = link => new Promise((resolve, reject) => {
    request(link, (err, res, body) => {
	const doc = new window.DOMParser().parseFromString(body, "text/html");
	const coordinates = Array.from(doc.getElementsByClassName('camera-details')[0].children)
		  .filter((el, i) => [4,5].includes(i))
		  .map(el => parseFloat(el.children[1].innerHTML));

	console.log(coordinates);
	return resolve(coordinates);
    });
});


Array(5).fill().map(async (_, i) => {
    const url = `https://www.insecam.org/en/bycity/Paris/?page=${i}`;
    const camIds = await getCamIds(url);

    return camIds;
});
