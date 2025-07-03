COMPOSE_FILE=docker-compose.dev.yml

.PHONY: up up-all up-back up-db up-front down down-clean

# Levanta todos los servicios
up: up-all

up-all:
	docker-compose -f $(COMPOSE_FILE) up -d

# Levanta el backend (actix_api) junto con la base de datos
up-back:
	docker-compose -f $(COMPOSE_FILE) up -d gateway actix_api db --build

# Levanta solo el servicio de la base de datos
up-db:
	docker-compose -f $(COMPOSE_FILE) up -d db

# Levanta solo el servicio del frontend
up-front:
	docker-compose -f $(COMPOSE_FILE) up -d yew_front

# Baja todos los servicios sin borrar volúmenes ni imágenes
down:
	docker-compose -f $(COMPOSE_FILE) down

# Baja todos los servicios y elimina volúmenes e imágenes
down-clean:
	docker-compose -f $(COMPOSE_FILE) down -v --rmi all
	docker builder prune -af
	docker buildx prune -af

db-shell:
	docker-compose -f $(COMPOSE_FILE) exec db psql -U postgres -d app

prod-ls:
	docker --context testing service ls

api-logs:
	docker --context testing service logs tapi_actix_api

front-logs:
	docker --context testing service logs tapi_yew_front

nginx-logs:
	docker --context testing service logs tapi_nginx

prod-force-clean:
	docker --context testing system prune -a --volumes 

prod-down:
	docker --context testing stack rm tapi 
	docker --context testing network rm tapi_app_network

prod-deploy:
	docker --context testing stack deploy -c compose.yml tapi --with-registry-auth

run-cmd:
	@cd $(path) && "./run.cmd"
