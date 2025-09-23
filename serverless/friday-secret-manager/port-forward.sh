# Description: Forward the port of the pod to the local machine
# The pod name may change, so you need to get the pod name first
kubectl port-forward pod/friday-secret-manager-7944b6496d-c7hqm 5000:5000