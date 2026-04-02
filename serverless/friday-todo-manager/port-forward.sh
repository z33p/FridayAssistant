# Description: Forward the port of the pod to the local machine
# The pod name may change, so you need to get the pod name first
kubectl port-forward pod/friday-todo-manager-74f6c56d75-pjcdj 5000:5000 