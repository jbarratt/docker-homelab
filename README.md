# Docker Homelab

This is how I configure docker for my home lab.

I'm using docker-compose because it's bulletproof and portable. For a single machine, kubernetes or the slimmer single machine alternatives doesn't bring much to the party.

## File Layout

```
monitoring.yml # the monitoring stack
minecraft.yml # the minecraft container party
imagesrc/{image name}/ # where the image builders live for custom images
```

## Common Commands

```
docker-compose -f monitoring.yml build
docker-compose -f monitoring.yml up -d
docker-compose -f monitoring.yml stop
docker-compose pull --ignore-pull-failures && docker-compose up -d # update all
```

## Minecraft Commands

### Connecting to rcon in a container

```
docker-compose -f minecraft.yml exec disneyland rcon-cli
```

### Backup Recovery

Assuming you have attached a volume at /recover pointing at a different location:

```
docker-compose -f minecraft.yml exec backup /bin/bash
# restic snapshots
# restic restore <sha> --target /recover/myrestore
```


## Helpful Tips

* [The compose docs](https://docs.docker.com/compose/)
* [Composerize](https://www.composerize.com) lets you turn a docker run command line into a fragment of compose file
* [Docker in your Homelab](https://borked.io/2019/02/13/docker-in-your-homelab.html)

## To Do

* Add the minecraft server
* Add minecraft prom exporter and connect to prometheus
* Add a backup tool for minecraft data, and create limited IAM role to use it
* Experiment with restores and add to runbook
* Add a container updater https://github.com/containrrr/watchtower
* Extend prometheus to do push notifications over telegram if the server is down

## Bedrock Minecraft Server


docker run -d -it -e EULA=TRUE -p 19132:19132/udp itzg/minecraft-bedrock-server


## `.env` file contents

AWS_ACCESS_KEY_ID=""
AWS_SECRET_ACCESS_KEY=""
AWS_DEFAULT_REGION=""
RESTIC_REPOSITORY="s3:https://s3.amazonaws.com/mybucket/mybackuppath/"
RESTIC_FORGET_ARGS="--prune --keep-daily 7 --keep-weekly 52 --keep-monthly 120 --keep-yearly 100"
RESTIC_PASSWORD="looselips"
RCON_PASSWORD="nohackingrconplz"


```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "VisualEditor0",
            "Effect": "Allow",
            "Action": "s3:ListBucket",
            "Resource": "arn:aws:s3:::serialized-backups"
        },
        {
            "Sid": "VisualEditor1",
            "Effect": "Allow",
            "Action": [
                "s3:PutObject",
                "s3:GetObject",
                "s3:DeleteObject"
            ],
            "Resource": "arn:aws:s3:::serialized-backups/gibson/*"
        }
    ]
}
```
