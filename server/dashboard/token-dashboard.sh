# Gerar token acesso Dashboard
# kubectl create token admin-user -n kubernetes-dashboard
 
# kubectl get secret admin-user -n kubernetes-dashboard -o jsonpath={".data.token"} | base64 -d
