

python:
	cd email_backend; python -m PyInstaller --onefile email_backend.py
	cp email_backend\dist\email_backend.exe target\debug\email_backend.exe