#!/usr/bin/env python3

from http.server import BaseHTTPRequestHandler, HTTPServer
from libnmap.process import NmapProcess
from libnmap.parser import NmapParser
from time import sleep
import requests
import socket

class SonnenClient:

    def __init__(self):
        self.ip = None
        self.update_ip()

    def update_ip(self):
        # hardcode it because I'm lazy AND it's a little tricky to discover from inside docker
        subnet = "192.168.7.0/24"
        nm = NmapProcess(subnet, options="-T5 -n -p 8080 --open --min-parallelism 255")
        nm.run_background()

        while nm.is_running():
            sleep(0.5)
        nmap_report = NmapParser.parse(nm.stdout)
        for host in nmap_report.hosts:
            print(host.address)
            rv = requests.get(f"http://{host.address}:8080/api/v1/status")
            if rv.status_code == 200:
                if 'BackupBuffer' in rv.json():
                    print("Found device")
                    self.ip = host.address
                    return
        print("Unable to find any device")
        self.ip = None
        raise Exception("Unable to find device")

    def metrics(self):
        if self.ip is None:
            self.update_ip()
        if self.ip is None:
            return None
        rv = requests.get(f"http://{self.ip}:8080/api/v1/status")
        return rv.json()

# There are more elegant ways to do this, but it needs to be available to all the
# RequestHandler instances, so .... this works.
CLIENT = SonnenClient()

class SonnenMetricsServer(BaseHTTPRequestHandler):

    def get_metrics(self):
        data = CLIENT.metrics()
        return f"""
# HELP sonnen_online Metric scraping successful
# TYPE sonnen_online gauge
sonnen_online{{host="{CLIENT.ip}",sn="69957"}} 0
# HELP sonnen_production_watts Number of watts being produced by solar
# TYPE sonnen_production_watts gauge
sonnen_production_watts{{host="{CLIENT.ip}",sn="69957"}} {data['Production_W']}
# HELP sonnen_state_of_charge_percent Percent charged of the battery
# TYPE sonnen_state_of_charge_percent gauge
sonnen_state_of_charge_percent{{host="{CLIENT.ip}",sn="69957"}} {data['RSOC']}
# HELP sonnen_grid_feed_in_watts Number of watts being fed into the grid (negative denotes grid purchase)
# TYPE sonnen_grid_feed_in_watts gauge
sonnen_grid_feed_in_watts{{host="{CLIENT.ip}",sn="69957"}} {data['GridFeedIn_W']}
# HELP sonnen_consumption_watts Number of watts being consumed
# TYPE sonnen_consumption_watts gauge
sonnen_consumption_watts{{host="{CLIENT.ip}",sn="69957"}} {data['Consumption_W']}
# HELP sonnen_discharge_watts Number of watts being discharged by the battery (negative means battery charging)
# TYPE sonnen_discharge_watts gauge
sonnen_discharge_watts{{host="{CLIENT.ip}",sn="69957"}} {data['Pac_total_W']}
"""

    def do_GET(self):
        print("getting it")
        metrics = None
        err_string = ""
        try:
            metrics = self.get_metrics()
        except Exception as error:
            err_string = str(error)

        if metrics is None:
            self.send_response(500)
            self.send_header("Content-type", "text/plain")
            self.wfile.write(bytes("Error fetching metrics", "utf-8"))
            self.wfile.write(bytes(err_string, "utf-8"))
            return

        self.send_response(200)
        self.send_header("Content-type", "text/plain")
        self.end_headers()
        self.wfile.write(bytes(metrics, "utf-8"))


if __name__ == '__main__':
    server = HTTPServer(("0.0.0.0", 9423), SonnenMetricsServer)
    print("server started")

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass

    server.server_close()
