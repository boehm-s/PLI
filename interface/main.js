import electron	from 'electron';
import path	from 'path';
import url	from 'url';

const app = electron.app;
const BrowserWindow = electron.BrowserWindow;

let mainWindow;

const createWindow = () => {
    mainWindow = new BrowserWindow({
	width: 800,
	height: 600,
	webPreferences: {
	    webgl: true,
	    "web-security": false
	}
    });
    mainWindow.loadURL(url.format({
	pathname: path.join(__dirname, 'index.html'),
	protocol: 'file:',
	slashes: true
    }));
    mainWindow.maximize();
    mainWindow.on('closed', _ => mainWindow = null);
};

app.on('ready',			createWindow);
app.on('window-all-closed',	_ => process.platform !== 'darwin' ? app.quit() : null);
app.on('activate',		_ => mainWindow === null ? createWindow() : null);
