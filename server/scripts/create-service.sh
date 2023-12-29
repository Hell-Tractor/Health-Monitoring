echo "Creating docker images for health-monitoring-server"
docker build -t health-monitoring-server-api-docker -f Dockerfile.api .
docker build -t health-monitoring-server-static-docker -f Dockerfile.static .

echo "running docker images"
docker run -d -p 9999:9999 -e MYSQL_PASSWORD=$MYSQL_PASSWORD health-monitoring-server-api-docker
docker run -d -p 9998:9998 health-monitoring-server-static-docker