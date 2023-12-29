echo "Creating docker images for health-monitoring-server"
docker build -t health-monitoring-server-api -f Dockerfile.api .
docker build -t health-monitoring-server-static -f Dockerfile.static .

echo "running docker images"
docker run -d -p 9999:9999 health-monitoring-server-api -e MYSQL_PASSWORD=$MYSQL_PASSWORD
docker run -d -p 9998:9998 health-monitoring-server-static