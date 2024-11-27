
- 常用开发命令

```
python3 -m venv .
pip install eventlet
pip install gunicorn
pip install flask


gunicorn -b :7201 -k geventwebsocket.gunicorn.workers.GeventWebSocketWorker -w 1 app:app
gunicorn -b :7201 -w 1 app:app
gunicorn -c setting.py app:app

```