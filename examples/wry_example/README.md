# fullstack desktop web app using wry

This also shows how to communicate between the backend and the frontend using a window's ipc postMessage and a webview's evaluate_script.

To build:
```
cd frontend
livid deploy --using=../wry_backend
./bundle/wry_backend
```