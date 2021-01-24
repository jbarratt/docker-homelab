docker run -d -p 25565:25565 -v /home/jbarratt/data/minecraft:/data --name mc -e EULA=TRUE --restart always itzg/minecraft-server

Run backups with Restic:


	https://restic.readthedocs.io/en/stable/030_preparing_a_new_repo.html

	Using this wrapper docker container:

		https://github.com/Lobaro/restic-backup-docker


		s3:https://s3.amazonaws.com/serialized-backups/gibson/minecraft

	RESTIC_REPOSITORY - the location of the restic repository. Default /mnt/restic. For S3: s3:https://s3.amazonaws.com/BUCKET_NAME
	RESTIC_PASSWORD - the password for the restic repository. Will also be used for restic init during first start when the repository is not initialized.
	BACKUP_CRON - A cron expression to run the backup. Note: cron daemon uses UTC time zone. Default: 0 */6 * * * aka every 6 hours.
	RESTIC_FORGET_ARGS - Optional. Only if specified restic forget is run with the given arguments after each backup. Example value: -e "RESTIC_FORGET_ARGS=--prune --keep-last 10 --keep-hourly 24 --keep-daily 7 --keep-weekly 52 --keep-monthly 120 --keep-yearly 100"
	AWS_ACCESS_KEY_ID - Optional. When using restic with AWS S3 storage.
	AWS_SECRET_ACCESS_KEY - Optional. When using restic with AWS S3 storage.


	/data - This is the data that gets backed up. Just mount it to wherever you want.
	Set --hostname in the network settings so the backups aren't stuck to the restic random container name

	-v ~/home/user/hooks:/hooks
		Call your pre backup script pre-backup.sh and post backup script post-backup.sh

	Get snapshots:
		docker exec -ti restic-backup-var restic snapshots

	Recover:
		docker exec -ti restic-backup-var restic restore --include /data/path/to/files --target / abcdef12


	https://hub.docker.com/r/itzg/rcon

Inspiration from

	https://hub.docker.com/r/itzg/mc-backup

Also, get minecraft-exporter going:

	https://github.com/Joshi425/minecraft-exporter

or

	https://github.com/itzg/mc-monitor

```
For instance, imagine a scenario when the initial launch has completed, but you now want to change the worldmap for your server.

Assuming you have a shared directory to your container, you can then (after first launch) drag and drop your premade maps or worlds into the \worlds\ directory. Note: each world should be placed in its own folder under the \worlds\ directory.

Once your maps are in the proper path, you can then specify which map the server uses by changing the level-name value in server.properties to match the name of your map.
```
