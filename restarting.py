#!/usr/bin/env python3

import docker

def main():
  client = docker.from_env()
  for container in client.containers.list(all=True):
    try:
      policy = container.attrs['HostConfig']['RestartPolicy']['Name']
      if len(policy) > 1 and policy != "no":
        print(f"{container.name}: {container.id[0:8]}: {container.attrs['HostConfig']['RestartPolicy']['Name']}")
    except:
      pass

if __name__ == '__main__':
  main()
