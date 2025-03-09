# Description: Forward the port of the pod to the local machine
# The pod name may change, so you need to get the pod name first
kubectl port-forward pod/friday-secret-manager-688779b4c7-f9qf6 5000:5000 