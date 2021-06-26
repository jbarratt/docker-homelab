.PHONY: cleanup

cleanup:
	docker-compose -f dns.yml -f minecraft.yml -f monitoring.yml up --remove-orphans -d 
