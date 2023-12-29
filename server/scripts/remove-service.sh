echo "Stopping images"
docker stop health-monitoring-server-api
docker stop health-monitoring-server-static

echo "Removing images"
docker rm health-monitoring-server-api
docker rm health-monitoring-server-static

docker rmi health-monitoring-server-api
docker rmi health-monitoring-server-static