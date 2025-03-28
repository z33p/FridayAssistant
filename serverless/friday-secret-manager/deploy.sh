# Ask user for Docker tag version
echo "Enter Docker tag version (e.g. 2024-12-22.v1):"

# Check if dockerTag was passed as argument and if it wasn't ask the user for it
if [ -z "$1" ]; then
    read dockerTag
else
    dockerTag=$1
fi

while true; do
    # Check if dockerTag is in the correct format
    if [[ ! $dockerTag =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}.v[0-9]+$ ]]; then
        echo "Invalid Docker tag version. Please enter a valid version (e.g. 2024-12-22.v1):"
        read dockerTag
        continue
    fi

    # Check if inside the file k8s/friday-secret-manager-deployment.yml the image version is the provided by the user
    if ! grep -q "friday-secret-manager:$dockerTag" k8s/friday-secret-manager-deployment.yml; then
        echo "The Docker tag version provided does not match the image version in k8s/friday-secret-manager-deployment.yml. Please enter a valid version:"
        read dockerTag
        continue
    fi

    # If checks pass, break the loop
    break
done

# Create Docker image from Dockerfile
docker build --pull --rm -f "FridaySecretManager.Dockerfile" -t friday-secret-manager:$dockerTag "."

# Push Docker image to Docker Hub
 docker image push docker.io/z33p/friday-secret-manager:$dockerTag 

# Deploy Docker image to Kubernetes
kubectl apply -f k8s/friday-secret-manager-deployment.yml