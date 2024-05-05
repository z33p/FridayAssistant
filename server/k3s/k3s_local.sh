# scp -i MyPrivateLightsail.pem admin@100.27.66.245:kube_config ~/.kube/config

# Scripts para execução localhost
# scp root@<IP_OF_LINUX_MACHINE>:/etc/rancher/k3s/k3s.yaml ~/.kube/config
# Após copia é necessário editar o arquivo ~/.kube/config e substituir o IP do servidor pelo IP da máquina remota
