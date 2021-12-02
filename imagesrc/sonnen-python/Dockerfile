FROM python:3-slim-bullseye

RUN apt-get update && apt-get install -y --no-install-recommends nmap && rm -rf /var/lib/apt/lists/*
RUN pip install python-libnmap requests
ADD sonnen_exporter.py /
CMD ["python", "./sonnen_exporter.py"]
