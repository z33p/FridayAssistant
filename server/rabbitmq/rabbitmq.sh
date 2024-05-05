kubectl apply -f "https://github.com/rabbitmq/cluster-operator/releases/latest/download/cluster-operator.yml"

# Local Path Provisioner
kubectl apply -f https://raw.githubusercontent.com/rancher/local-path-provisioner/master/deploy/local-path-storage.yaml
kubectl annotate storageclass local-path storageclass.kubernetes.io/is-default-class=true

kubectl apply -f rabbitmq.yml

# # Iniciar RabbitMq
# kubectl port-forward "service/friday-rabbitmq" 15672

# username="$(kubectl get secret friday-rabbitmq-default-user -o jsonpath='{.data.username}' | base64 --decode)"
# echo "username: $username"
# password="$(kubectl get secret friday-rabbitmq-default-user -o jsonpath='{.data.password}' | base64 --decode)"
# echo "password: $password"

# kubectl port-forward "service/friday-rabbitmq" 15672