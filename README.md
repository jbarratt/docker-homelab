# Docker Homelab

This is how I configure docker for my home lab.

I'm using docker-compose because it's bulletproof and portable. For a single machine, kubernetes or the slimmer single machine alternatives doesn't bring much to the party.

## File Layout

```
Makefile # common processes
docker-compose.yml # the main file
imgsrc/{image name}/ # where the image builders live for custom images
```

## Helpful Tips

* [The compose docs](https://docs.docker.com/compose/)
* [Composerize](https://www.composerize.com) lets you turn a docker run command line into a fragment of compose file
* [Docker in your Homelab](https://borked.io/2019/02/13/docker-in-your-homelab.html)

Getting AWS credentials into docker containers seems tricky to do well without running swarm.

You can do by ENV file that's .gitignored:

.env
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=

add a .aws/credentials file and volume share it in, :ro

```
${HOME}/.aws/credentials:/root/.aws/credentials:ro
```

## To Do

* Port the prometheus setup over
* Add the minecraft server
* Add minecraft prom exporter and connect to prometheus
* Add a backup tool for minecraft data, and create limited IAM role to use it
* Experiment with restores and add to runbook
* Add a container updater https://github.com/containrrr/watchtower
* Extend prometheus to do push notifications over telegram if the server is down

## Bedrock Minecraft Server
