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

* Add minecraft prom exporter and connect to prometheus
* Monitor the backups
* Extend prometheus to do push notifications over telegram if the server is down
* solve the rcon work so it works with multiple containers


## `.env` file contents

The `.env` file is not checked in because it has secrets or host-dependent values in it.

```
AWS_ACCESS_KEY_ID=""
AWS_SECRET_ACCESS_KEY=""
AWS_DEFAULT_REGION=""
RESTIC_REPOSITORY="s3:https://s3.amazonaws.com/mybucket/mybackuppath/"
RESTIC_FORGET_ARGS="--prune --keep-daily 7 --keep-weekly 52 --keep-monthly 120 --keep-yearly 100"
RESTIC_PASSWORD="looselips"
RCON_PASSWORD="nohackingrconplz"
```

## Sample IAM policy

```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "VisualEditor0",
            "Effect": "Allow",
            "Action": "s3:ListBucket",
            "Resource": "arn:aws:s3:::mybucketname"
        },
        {
            "Sid": "VisualEditor1",
            "Effect": "Allow",
            "Action": [
                "s3:PutObject",
                "s3:GetObject",
                "s3:DeleteObject"
            ],
            "Resource": "arn:aws:s3:::mybucketname/myhost/*"
        }
    ]
}
```

## Alerting

Alertmanager's config is not in git because it has creds. But it's stupid simple

```
route:
  receiver: pushover

receivers:
  - name: pushover
    pushover_configs:
      - token: app token
        user_key: your user key
```

This can be tested as so:

```
curl -H "Content-Type: application/json" -d '[{"status": "firing", "labels":{"alertname":"TestAlert1"}}]' localhost:9093/alertmanager/api/v1/alerts
```
