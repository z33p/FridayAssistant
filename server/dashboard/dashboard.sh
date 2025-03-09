# Dashboard
helm repo add kubernetes-dashboard https://kubernetes.github.io/dashboard/

helm upgrade --install kubernetes-dashboard kubernetes-dashboard/kubernetes-dashboard --create-namespace --namespace kubernetes-dashboard

kubectl apply -f service-account.yaml

kubectl apply -f cluster-role-binding.yml

kubectl apply -f secret.yml

