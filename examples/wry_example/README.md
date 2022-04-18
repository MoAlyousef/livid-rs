# fullstack desktop web app using wry

This also shows how to communicate between the backend and the frontend using a window's ipc postMessage and a webview's evaluate_script.
Wry has the advantage of incorporating a systray, navigation policies, native window handling and other niceties on top of webview.

To build:
```
cd frontend
livid deploy --using=../wry_backend
./bundle/wry_backend
```