echo "Applying nginx YAML files..."
kubectl apply -f nginx-config.yml
kubectl apply -f nginx-daemonset.yml

echo "Restarting nginx daemonset..."
kubectl rollout restart daemonset nginx-proxy

echo "Deployment and restart complete."